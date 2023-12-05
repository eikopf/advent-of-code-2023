use std::{str::FromStr, ops::Range};
use aoc::Solution;
use nom::{
    IResult, 
    sequence::{preceded, Tuple, terminated}, 
    bytes::complete::{tag, is_not}, 
    multi::separated_list1, 
    character::complete::{u32, newline, multispace1}, 
    combinator::map_res, 
    Parser, error::Error, Finish
};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

/// Represents a struct that knows whether
/// or not it has been transformed in some way.
///
/// This is used to enforce the idempotency of [CategoryMap]s.
enum TransformLock<T> {
    /// Denotes that this data should be transformed.
    Locked(T),
    /// Denotes that this data should not be transformed.
    Unlocked(T),
}

/// Wraps a given vector with [TransformLock::Unlocked] elementwise.
fn unlocked<T>(vec: Vec<T>) -> Vec<TransformLock<T>> {
    vec
        .into_iter()
        .map(|x| TransformLock::Unlocked(x))
        .collect()
}

/// Clears the elementwise locks from the given vector.
fn strip_locks<T>(vec: Vec<TransformLock<T>>) -> Vec<T> {
    vec.into_iter().map(|locked| match locked {
        TransformLock::Locked(x) => x,
        TransformLock::Unlocked(x) => x,
    }).collect()
}

/// Represents an individual line in a map.
#[derive(Debug, Clone)]
struct RangeMap {
    /// The starting source number (the second field of a line).
    source_start: usize,
    /// The starting target (destination) number (the first field of a line).
    target_start: usize,
    /// The length of the ranges (the third field of a line).
    len: usize,
}

impl RangeMap {
    /// Consumes self and some input, and returns the image
    /// of the input under this mapping. If the image has been
    /// transformed, then it will be locked; otherwise it will
    /// remain unlocked. This function also respects the locked
    /// status of incoming input.
    fn apply(self, vec: Vec<TransformLock<usize>>) -> Vec<TransformLock<usize>> {
        vec.into_iter().map(move |x| match x {
            TransformLock::Locked(x) => TransformLock::Locked(x),
            TransformLock::Unlocked(x) => {
                if (x >= self.source_start) && (x < self.source_start + self.len) {
                    TransformLock::Locked((x - self.source_start) + self.target_start)
                } else {
                    TransformLock::Unlocked(x)
                }
            }
        }).collect()
    }

    fn process_value(&self, value: usize) -> usize {
        if (value >= self.source_start) && (value < self.source_start + self.len) {
            value - self.source_start + self.target_start
        } else {
            value
        }
    }
}

/// Represents a complete mapping between categories.
#[derive(Debug, Clone)]
struct CategoryMap {
    /// Represents the individual lines of the map.
    range_maps: Vec<RangeMap>,
}

impl CategoryMap {
   fn apply(self, vec: Vec<usize>) -> Vec<usize> {
        strip_locks(self
            .range_maps
            .into_iter()
            .fold(unlocked(vec), 
                |v, range_map| { 
                    range_map
                        .apply(v)
                }
            )
        )
    }

    fn process_value(&self, value: usize) -> usize {
        for map in &self.range_maps {
            let image = map.process_value(value);
            
            if image != value {
                return image;
            }
        }

        value
    } 
}

/// Represents the complete source data, with the maps stored
/// in-order such that applying them sequentially will produce
/// a seed-location mapping.
#[derive(Debug, Clone)]
struct Almanac<T> {
    /// The seeds given by the first line of the source data.
    seeds: Vec<T>,
    /// The category maps given by each individual map.
    maps: Vec<CategoryMap>,
}

impl Almanac<usize> {
    /// Consumes self and returns the result of
    /// applying all the maps in self in series.
    fn apply_all(self) -> Vec<usize> {
        self
            .maps
            .into_iter()
            .fold(self.seeds, |acc, map|{ map.apply(acc) })
    }
}

impl Almanac<Range<usize>> {
    fn get_minimum_location(self) -> usize {
        self
            .seeds
            .par_iter()
            .flat_map(|range| range.clone().into_iter())
            .fold_with(usize::MAX, |minimum, seed| {
                let location = self
                    .maps
                    .clone()
                    .into_iter()
                    .fold(seed, 
                        |image, map| 
                        map.process_value(image)
                    );

                if location < minimum {
                    location
                } else {
                    minimum
                }
            })
            .min()
            .unwrap()
    }
}

impl FromStr for Almanac<usize> {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parser = seeds
            .and(separated_list1(multispace1, map));

        match parser.parse(s).finish() {
            Ok((_, (seeds, maps))) => Ok(Almanac { seeds, maps }),
            Err(Error { input, code }) => Err(Error { input: input.to_string(), code }),
        }
    }
}

impl From<Almanac<usize>> for Almanac<Range<usize>> {
    fn from(value: Almanac<usize>) -> Self {
        Almanac {
            maps: value.maps,
            seeds: value
                .seeds
                .chunks_exact(2)
                .map(|chunk|{
                    let a = usize::min(chunk[0], chunk[1]);
                    let b = usize::max(chunk[0], chunk[1]);
                    a..b
                })
                .collect(),
        }
    }
}

/// Parses the first line of the input into a list of seeds,
/// and consumes the trailing whitespace.
fn seeds(source: &str) -> IResult<&str, Vec<usize>> {
    let mut parser = terminated(preceded(
        tag("seeds: "), 
        separated_list1(
            tag(" "), 
            map_res(u32, |x| usize::try_from(x))
        )), multispace1);

    parser.parse(source)
}

/// Parses an individual line in a map, leaving a trailing newline.
fn map_line(source: &str) -> IResult<&str, RangeMap> {
    let mut parser = (
        map_res(u32, |x| x.try_into()), 
        preceded(
            tag(" "), 
            map_res(u32, |x| x.try_into()), 
        ), 
        preceded(
            tag(" "), 
            map_res(u32, |x| x.try_into()), 
        ));

    parser
        .parse(source)
        .map(|(tail, (target_start, source_start, len))|{
        (tail, RangeMap {
            source_start,
            target_start,
            len,
        })
    })
}

/// Parses a complete map.
fn map(source: &str) -> IResult<&str, CategoryMap> {
    let mut parser = preceded(
        is_not("\n").and(newline), 
        separated_list1(
            newline, 
            map_line
        )
    );

    parser
        .parse(source)
        .map(|(tail, range_maps)|{
        (tail, CategoryMap { range_maps })
    })
}

/// Reads the input from stdin and returns the answer to question 1.
///
/// The answer to this question is the lowest location number that
/// corresponds to any of the initial seeds; equivalently this is just
/// the minimum of the image of the seeds under the sequential image
/// of all the given maps.
fn get_q1_result() -> anyhow::Result<usize> {
    let source = aoc::read_stdin_to_string();
    let almanac = Almanac::from_str(&source)?;
    let locations = almanac.apply_all();
    match locations.iter().min() {
        Some(&min) => Ok(min),
        None => Err(anyhow::Error::msg("locations has no minimum")),
    }
}

/// Reads the input from stdin and returns the answer to question 2.
fn get_q2_result() -> anyhow::Result<usize> {
    let source = aoc::read_stdin_to_string();
    let almanac: Almanac<Range<usize>> = Almanac::from_str(&source)?.into();
    Ok(almanac.get_minimum_location())
}

fn main() {
    let cli: Solution = argh::from_env();
    let res: usize = match cli.question {
        aoc::Question::One => get_q1_result(),
        aoc::Question::Two => get_q2_result(),
    }.unwrap();
    println!("{}", res);
}

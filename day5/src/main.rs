use std::ops::Range;

use aoc::Solution;
use nom::{
    bytes::complete::{is_not, tag},
    character::complete::{multispace1, newline, u32},
    combinator::map_res,
    error::Error,
    multi::separated_list1,
    sequence::{preceded, terminated, Tuple},
    Finish, IResult, Parser,
};
use rangemap::RangeMap;

/// Represents the complete source data, with the maps stored
/// in-order such that applying them sequentially will produce
/// a seed-location mapping.
#[derive(Debug, Clone)]
struct Almanac {
    /// The category maps given by each individual map.
    maps: Vec<RangeMap<usize, isize>>,
}

/// Parses the first line of the input into a list of seeds,
/// and consumes the trailing whitespace.
fn seeds(source: &str) -> IResult<&str, Vec<usize>> {
    let mut parser = terminated(
        preceded(
            tag("seeds: "),
            separated_list1(tag(" "), map_res(u32, |x| usize::try_from(x))),
        ),
        multispace1,
    );

    parser.parse(source)
}

/// Parses an individual line in a map, leaving a trailing newline.
fn map_line(source: &str) -> IResult<&str, (Range<usize>, isize)> {
    let mut parser = (
        map_res(u32, |x| usize::try_from(x)),
        preceded(tag(" "), map_res(u32, |x| usize::try_from(x))),
        preceded(tag(" "), map_res(u32, |x| usize::try_from(x))),
    );

    parser
        .parse(source)
        .map(|(tail, (target_start, source_start, len))| {
            (
                tail,
                (
                    (source_start)..(source_start + len),
                    (target_start as isize - source_start as isize),
                ),
            )
        })
}

/// Parses a complete map.
fn map(source: &str) -> IResult<&str, Vec<(Range<usize>, isize)>> {
    let mut parser = preceded(
        is_not("\n").and(newline),
        separated_list1(newline, map_line),
    );

    parser.parse(source)
}

/// Parses the given input and returns a list of seed values and an almanac composed
/// of the mappings defined by the input.
fn read_input(source: &str) -> anyhow::Result<(Vec<usize>, Almanac)> {
    let mut parser = (seeds, separated_list1(newline, map));
    match parser.parse(source).finish() {
        Ok((_, (seeds, maps))) => {
            let mut almanac = Almanac { maps: Vec::new() };

            for map in maps {
                let mut range_map = RangeMap::new();
                for (range, offset) in map {
                    range_map.insert(range, offset);
                }
                almanac.maps.push(range_map);
            }

            Ok((seeds, almanac))
        }
        Err(Error { input, code }) => Err(Error {
            input: input.to_string(),
            code,
        }
        .into()),
    }
}

/// Reads the input from stdin and returns the answer to question 1.
///
/// The answer to this question is the lowest location number that
/// corresponds to any of the initial seeds; equivalently this is just
/// the minimum of the image of the seeds under the sequential image
/// of all the given maps.
fn get_q1_result() -> anyhow::Result<usize> {
    let source = aoc::read_stdin_to_string();
    let (seeds, almanac) = read_input(&source)?;

    todo!()
}

/// Reads the input from stdin and returns the answer to question 2.
fn get_q2_result() -> anyhow::Result<usize> {
    todo!()
}

fn main() {
    let cli = Solution::new();
    let res: usize = match cli.question {
        aoc::Question::One => get_q1_result(),
        aoc::Question::Two => get_q2_result(),
    }
    .unwrap();
    println!("{}", res);
}

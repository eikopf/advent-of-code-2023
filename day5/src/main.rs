use std::ops::Range;

use aoc::Solution;
use nom::{
    bytes::complete::{is_not, tag},
    character::complete::{multispace1, newline, u32},
    combinator::map_res,
    multi::separated_list1,
    sequence::{preceded, terminated, Tuple},
    IResult, Parser,
};
use rangemap::RangeMap;

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
    vec.into_iter()
        .map(|x| TransformLock::Unlocked(x))
        .collect()
}

/// Clears the elementwise locks from the given vector.
fn strip_locks<T>(vec: Vec<TransformLock<T>>) -> Vec<T> {
    vec.into_iter()
        .map(|locked| match locked {
            TransformLock::Locked(x) => x,
            TransformLock::Unlocked(x) => x,
        })
        .collect()
}

/// Represents a complete mapping between categories.
#[derive(Debug, Clone)]
struct CategoryMap {
    /// Represents the individual lines of the map.
    range_maps: Vec<RangeMap<usize, isize>>,
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

/// Reads the input from stdin and returns the answer to question 1.
///
/// The answer to this question is the lowest location number that
/// corresponds to any of the initial seeds; equivalently this is just
/// the minimum of the image of the seeds under the sequential image
/// of all the given maps.
fn get_q1_result() -> anyhow::Result<usize> {
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

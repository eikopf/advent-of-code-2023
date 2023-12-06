use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace1, newline, u16},
    combinator::map_res,
    error::Error,
    multi::separated_list1,
    sequence::{preceded, Tuple},
    Finish, IResult, Parser,
};

use rayon::iter::{
    IntoParallelIterator, 
    IndexedParallelIterator, 
    ParallelIterator
};

/// Represents a single race (column) from the source data.
#[derive(Clone, Debug, PartialEq, Eq)]
struct Race {
    /// The duration of the race in milliseconds.
    duration: usize,
    /// The record distance in the race in millimeters.
    record_distance: usize,
}

/// Parses the entire input into a list of races.
fn parse_inputs(source: &str) -> IResult<&str, Vec<Race>> {
    let times = preceded(
        tag("Time:").and(multispace1),
        separated_list1(multispace1, map_res(u16, |x| usize::try_from(x))),
    );

    let distances = preceded(
        tag("Distance:").and(multispace1),
        separated_list1(multispace1, map_res(u16, |x| usize::try_from(x))),
    );

    (times, newline, distances)
        .parse(source)
        .map(|(tail, (times, _, distances))| {
            (
                tail,
                times
                    .into_iter()
                    .zip(distances.into_iter())
                    .map(|(duration, record_distance)| Race {
                        duration,
                        record_distance,
                    })
                    .collect(),
            )
        })
}

/// Parses the entire input into a single race.
fn parse_single_race(source: &str) -> IResult<&str, Race> {
    let time = preceded(
        tag("Time:").and(multispace1),
        map_res(separated_list1(multispace1, digit1), |digits| {
            digits.join("").parse::<usize>()
        }),
    );

    let distance = preceded(
        tag("Distance:").and(multispace1),
        map_res(separated_list1(multispace1, digit1), |digits| {
            digits.join("").parse::<usize>()
        }),
    );

    (time, newline, distance)
        .parse(source)
        .map(|(tail, (duration, _, record_distance))| {
            (
                tail,
                Race {
                    duration,
                    record_distance,
                },
            )
        })
}

/// For some reason this function boundary makes the compiler
/// understand that it's fine to drop the source after parsing
/// it.
///
/// Maybe there are some implied lifetimes here?
fn get_races() -> anyhow::Result<Vec<Race>> {
    let source = aoc::read_stdin_to_string();
    match parse_inputs(&source).finish() {
        Ok((_, races)) => Ok(races),
        Err(Error { input, code }) => Err(Error {
            input: input.to_string(),
            code,
        }
        .into()),
    }
}

fn get_race() -> anyhow::Result<Race> {
    let source = aoc::read_stdin_to_string();
    match parse_single_race(&source).finish() {
        Ok((_, race)) => Ok(race),
        Err(Error { input, code }) => Err(Error {
            input: input.to_string(),
            code,
        }
        .into()),
    }
}

fn get_q1_result() -> anyhow::Result<usize> {
    let races = get_races()?;

    Ok(races
        .into_iter()
        .map(
            |Race {
                 duration,
                 record_distance,
             }| {
                (0..=duration)
                    .zip((0..=duration).rev())
                    .zip(std::iter::repeat(record_distance))
            },
        )
        .map(|iter| iter.filter(|((a, b), dist)| (a * b) > *dist).count())
        .product())
}

fn get_q2_result() -> anyhow::Result<usize> {
    let race = get_race()?;

    Ok((0..(race.duration))
        .into_par_iter()
        .zip((0..(race.duration)).into_par_iter().rev())
        .filter(|(a, b)| a * b > race.record_distance)
        .count())
}

fn main() {
    let cli = aoc::Solution::new();
    let res = match cli.question {
        aoc::Question::One => get_q1_result(),
        aoc::Question::Two => get_q2_result(),
    }
    .unwrap();
    println!("{}", res);
}

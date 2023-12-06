use nom::{
    IResult, 
    sequence::{preceded, Tuple}, 
    bytes::complete::tag, 
    character::complete::{multispace1, u16, newline}, 
    Parser, multi::separated_list1, combinator::map_res
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
        separated_list1(
            multispace1, 
            map_res(u16, |x| usize::try_from(x)),
        )
    );

    let distances = preceded(
        tag("Distance:").and(multispace1), 
        separated_list1(
            multispace1, 
            map_res(u16, |x| usize::try_from(x)),
        )
    );

    (times, newline, distances)
        .parse(source)
        .map(|(tail, (times, _, distances))| 
            (tail, times
                .into_iter()
                .zip(distances.into_iter())
                .map(|(duration, record_distance)| Race { duration, record_distance })
                .collect()
            )
        )
}

/// For some reason this function boundary makes the compiler
/// understand that it's fine to drop the source after parsing
/// it.
///
/// Maybe there are some implied lifetimes here?
fn get_races() -> Vec<Race> {
    let source = aoc::read_stdin_to_string();
    let (_, races) = parse_inputs(&source).unwrap();
    races
}

fn get_q1_result() -> anyhow::Result<usize> {
    let races = get_races();
    eprintln!("{:?}", races);

    Ok(races
        .into_iter()
        .map(|Race { duration, record_distance }| 
            (0..=duration)
                    .zip((0..=duration).rev())
                    .zip(std::iter::repeat(record_distance)))
        .map(|iter| iter.filter(|((a, b), dist)| (a * b) > *dist).count())
        .product())
}

fn get_q2_result() -> anyhow::Result<usize> {
    todo!()
}

fn main() {
    let cli = aoc::Solution::new();
    let res = match cli.question {
        aoc::Question::One => get_q1_result(),
        aoc::Question::Two => get_q2_result(),
    }.unwrap();
    println!("{}", res);

}

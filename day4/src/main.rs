use std::str::FromStr;

use aoc::{Solution, Question};
use nom::{
    IResult, 
    sequence::{preceded, terminated}, 
    bytes::complete::tag, 
    character::complete::{u32, multispace1}, 
    Parser, 
    multi::separated_list1, 
    combinator::{map_res, opt}, 
    Finish, 
    error::Error
};

/// Represents an individual scratchcard
struct Card {
    /// The ID number of the card, which denotes its position in the sequence.
    id: usize,
    /// The marked winning numbers, to the left of the bar.
    ///
    /// Technically this would be more efficient to store in
    /// a [usize; 10] since there are always 10 values in the
    /// given input, but I don't want to bother with lifetimes
    /// and manual indices.
    winning: Vec<usize>,
    /// The actual numbers given to the holder of the scratchcard.
    ///
    /// This could also be stored in a constant-length array, as a
    /// [usize; 25], but as with the winning numbers, I don't really
    /// want to deal with that manually.
    actual: Vec<usize>,
}

impl Card {
    /// Returns the number of points that this card is worth.
    fn points(&self) -> usize {
        let mut match_count = 0;
        
        for winning in &self.winning {
            for actual in &self.actual {
                if winning == actual {
                    match_count += 1;
                }
            }
        }

        if match_count == 0 {
            0
        } else {
            2usize.pow(match_count - 1)
        }
    }
}

// The entrypoint to the parser is implemented here.
impl FromStr for Card {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parser = card.and(number_list).and(number_list);
        let (_, ((id, winning), actual)) = parser
            .parse(s)
            .finish()
            .map_err(|Error { input, code }| {
                Error {
                    input: input.to_string(),
                    code,
                }
            })?;

        Ok(Card { id, winning, actual })
    }
}

/// Parses the leading section of an input line, including trailing whitespace.
///
/// The source data has a bunch of variable-length whitespace, so I'm using the
/// [multispace1] function to handle it. The reason for this seems to be that
/// the extra whitespace makes each input line the exact same length (116).
fn card(source: &str) -> IResult<&str, usize> {
    let mut parser = preceded(
        tag("Card").and(multispace1), 
        terminated(
            u32, 
            tag(":").and(multispace1)
        )
    );

    parser
        .parse(source)
        .map(|(tail, x)| (tail, x.try_into().unwrap()))
}

/// Parses a whitespace-delimited list of integers, optionally followed by a sequence
/// of whitespace, a single "|" character, and a sequence of whitespace.
///
/// This function can parse both the winning and actual numbers from the source string,
/// but will only consume one at a time.
fn number_list(source: &str) -> IResult<&str, Vec<usize>> {
    let mut parser = terminated(
        separated_list1(
            multispace1, 
            map_res(u32, |x| usize::try_from(x))
        ), opt(multispace1.and(tag("|")).and(multispace1)));

    parser.parse(source)
}

/// Reads the input from stdin and returns the answer to question 1.
///
/// The answer to question 1 is defined as the sum of the number of
/// points in the input, and the points are defined as 2^(k-1) where k
/// is the number of actual numbers that are also winning numbers. Note
/// the exceptional case where k = 0, in which the result should be 0.
fn get_q1_result() -> anyhow::Result<usize> {
    Ok(std::io::stdin()
        .lines()
        .into_iter()
        .map(|line| Card::from_str(
            &line.unwrap())
            .unwrap()
            .points())
        .sum()
    )
}

fn main() {
    let cli: Solution = argh::from_env();
    let res = match cli.question {
        Question::One => get_q1_result(),
        Question::Two => todo!(),
    }.unwrap();

    eprintln!("{}", res);
}

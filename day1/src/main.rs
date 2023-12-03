use std::io;

use aoc::{Question, Solution};
use nom::{
    IResult, 
    character::complete::{alpha0, anychar}, 
    sequence::delimited, 
    multi::{separated_list0, many1}, 
    character::complete::{alpha1, one_of}, 
    Parser, 
    combinator::{opt, map_res, map}, 
    branch::alt, bytes::complete::tag
};

/// Parses a single digit from the source input.
fn single_digit(source: &str) -> IResult<&str, u8> {
    let mut parser = map_res(one_of("123456789"), |c| String::from(c).parse::<u8>());
    parser.parse(source)
}

/// Parses a single named digit from the given source.
fn named_digit(source: &str) -> IResult<&str, u8> {
    let mut parser = alt((
        tag("one"),
        tag("two"),
        tag("three"),
        tag("four"),
        tag("five"),
        tag("six"),
        tag("seven"),
        tag("eight"),
        tag("nine"),
    ));

    parser.parse(source).map(|(tail, name)| (tail, match name {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => unreachable!(),
    }))
}

/// Parses a line into a Vec<u8>, according to the question 1 specification.
fn literal_digits(source: &str) -> IResult<&str, Vec<u8>> {
    let mut parser = delimited(
        opt(alpha0), 
        separated_list0(
            alpha1,
            many1(single_digit),
        ), 
        opt(alpha0), 
    );
    parser.parse(source).map(|(tail, data)| (tail, data.into_iter().flatten().collect()))
}

/// Parses a line into a Vec<u8>, according to the question 2 specification.
fn literal_and_named_digits(source: &str) -> IResult<&str, Vec<u8>> {
    let mut parser = many1(
        alt((named_digit, single_digit, map(anychar, |_| 0)))
    );

    parser.parse(source).map(|(tail, v)| (tail, v.into_iter().filter(|&x| x != 0).collect()))
}

/// Computes the answer to question 1 by taking input from stdin
fn get_q1_result() -> anyhow::Result<usize> {
    let lines = io::stdin().lines();
    let mut acc: usize = 0;

    for line in lines {
        let input = line.unwrap();
        let digits = literal_digits(&input).unwrap().1;
        let first = *digits.first().unwrap() as usize;
        let last = *digits.last().unwrap() as usize;
        acc += (first * 10) + last;
    }

    Ok(acc)
}

fn get_q2_result() -> anyhow::Result<usize> {
    let lines = io::stdin().lines();
    let mut acc: usize = 0;

    for line in lines {
        let input = line.unwrap();
        let digits = literal_and_named_digits(&input).unwrap().1;
        let first = *digits.first().unwrap() as usize;
        let last = *digits.last().unwrap() as usize;
        acc += (first * 10) + last;
    }

    Ok(acc)
}

fn main() {
    let cli: Solution = argh::from_env();
    let res = match cli.question {
        Question::One => get_q1_result(),
        Question::Two => get_q2_result(),
    }.unwrap();
    println!("{:?}", res);
}

use std::io;

use nom::{
    IResult, 
    character::complete::{alpha0, digit1}, 
    sequence::delimited, 
    multi::{separated_list0, many1}, 
    character::complete::{u8, alpha1, one_of}, 
    Parser, 
    error::VerboseError,
    combinator::opt
};

/// Parses the input into a Vec<u8>, according to the question 1 specification.
fn parse_literal_digits(source: &str) -> IResult<&str, Vec<u8>> {
    let mut parser = delimited(
        opt(alpha0), 
        separated_list0(
            alpha1,
            many1(one_of("0123456789")).map(|v| 
                v.into_iter().map(|c| 
                    u8::<&str, VerboseError<&str>>(
                        String::from(c)
                            .as_str())
                            .unwrap()
                            .1
                )
            )
        ), 
        opt(alpha0), 
    );
    parser.parse(source).map(|(tail, data)| (tail, data.into_iter().flatten().collect()))
}

/// Computes the answer to question 1 by taking input from stdin
fn get_q1_result() -> anyhow::Result<usize> {
    let lines = io::stdin().lines();
    let mut acc: usize = 0;

    for line in lines {
        let input = line.unwrap();
        let digits = parse_literal_digits(&input).unwrap().1;
        eprintln!("{:?}, {:?}", input, digits);
        let first = *digits.first().unwrap() as usize;
        let last = *digits.last().unwrap() as usize;
        acc += (first * 10) + last;
    }

    Ok(acc)
}

fn main() {
    let res = get_q1_result().unwrap();
    println!("{:?}", res);
}

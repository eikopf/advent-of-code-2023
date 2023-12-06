use argh::FromArgs;
use std::{str::FromStr, fmt::Display};
use thiserror::Error;

#[derive(Error, Debug)]
pub struct QuestionParseError(String);

impl Display for QuestionParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Expected --question [0|1] or -q [0|1]; got {}", self.0)
    }
}

pub enum Question {
    One,
    Two,
}

impl FromStr for Question {
    type Err = QuestionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Self::One),
            "2" => Ok(Self::Two),
            other @ _ => Err(QuestionParseError(String::from(other))),
        }
    }
}

#[derive(FromArgs)]
/// A solution to an AOC2023 day.
pub struct Solution {
    #[argh(option, short = 'q')]
    /// the question to run
    pub question: Question, 
}

impl Solution {
    pub fn new() -> Self {
        argh::from_env()
    }
}

pub fn read_stdin_to_string() -> String {
    std::io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>()
        .join("\n")
}

pub fn read_stdin_by_line() -> std::io::Lines<std::io::StdinLock<'static>> {
    std::io::stdin().lines()
}

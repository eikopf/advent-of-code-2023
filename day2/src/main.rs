use std::io;

use aoc::{Solution, Question};
use nom::{
    IResult, 
    multi::{fold_many_m_n, many1}, 
    character::complete::u8, 
    Parser, 
    bytes::complete::tag, 
    sequence::{separated_pair, delimited, terminated},
    branch::alt, combinator::{opt, map},
};

/// Represents an individual set unveiled during a game.
#[derive(Debug, Clone, Copy)]
struct CubeSet {
    red: usize,
    green: usize,
    blue: usize,
}

/// Represents a full game, with its index and cube sets.
#[derive(Debug, Clone)]
struct Game {
    index: usize,
    sets: Vec<CubeSet>,
}

fn parse_cube_set(source: &str) -> IResult<&str, CubeSet> {
    let mut parser = fold_many_m_n(
        1, 
        3, 
        terminated(
            separated_pair(u8, tag(" "), alt((tag("green"), tag("blue"), tag("red")))),
            opt(tag(", "))
        ),
        || {CubeSet{red: 0, green: 0, blue: 0}}, 
        |set, (count, color)| match color {
            "red" => CubeSet {red: set.red + (count as usize), ..set},
            "blue" => CubeSet {blue: set.blue + (count as usize), ..set},
            "green" => CubeSet {green: set.green + (count as usize), ..set},
            _ => unreachable!(),
        }
    );

    parser.parse(source)
}

fn parse_game_string(source: &str) -> IResult<&str, Game> {
    let index = delimited(tag("Game "), u8, tag(": "));
    let sets = terminated(many1(parse_cube_set), opt(tag("; ")));
    let mut parser = map(index.and(many1(sets)), |(i, s)| Game {
        index: i as usize, 
        sets: s.into_iter().flatten().collect(),
    });

    parser.parse(source)
}

fn get_q1_result() -> anyhow::Result<usize> {
    let lines = io::stdin().lines();
    let mut possible_sum = 0usize;

    for line in lines {
        let input = line.unwrap();
        let game = parse_game_string(&input).unwrap().1;
        let mut game_is_possible = true;

        for set in &game.sets {
            if set.red > 12 || set.green > 13 || set.blue > 14 {
                game_is_possible = false;
            }
        }

        if game_is_possible {
            possible_sum += game.index;
        }
    }

    Ok(possible_sum)
}

fn get_q2_result() -> anyhow::Result<usize> {
    let lines = io::stdin().lines();
    let mut minimum_sets: Vec<CubeSet> = Vec::new();

    for line in lines {
        let input = line.unwrap();
        let game = parse_game_string(&input).unwrap().1;
        let mut minimum_set = CubeSet { 
            red: 0usize, 
            green: 0usize, 
            blue: 0usize,
        };

        for set in game.sets {
            if minimum_set.red < set.red {
                minimum_set.red = set.red;
            }

            if minimum_set.green < set.green {
                minimum_set.green = set.green;
            }

            if minimum_set.blue < set.blue {
                minimum_set.blue = set.blue;
            }
        }

        minimum_sets.push(minimum_set);
    }

    Ok(minimum_sets
        .into_iter()
        .fold(0, 
            |sum, set| 
            sum + (set.red * set.green * set.blue)
        )
    )
}

fn main() {
    let cli: Solution = argh::from_env();
    let res = match cli.question {
        Question::One => get_q1_result(),
        Question::Two => get_q2_result(),
    }.unwrap();
    println!("{}", res);
}

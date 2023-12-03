use aoc::{Solution, Question};

use std::{io, collections::HashMap};

use nom::{
    IResult, 
    character::complete::{digit1, u64, anychar}, 
    Parser, 
    combinator::{not, map}, 
    bytes::complete::tag, 
    multi::many1, 
    branch::alt,
};

/// Represents the elements which can appear in an engine schematic.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum SchematicItem {
    Number {value: u64, length: usize},
    Symbol(char),
}

type Schematic = Vec<Vec<Option<SchematicItem>>>;

/// Parses a number as it would appear in a schematic line.
fn number(source: &str) -> IResult<&str, SchematicItem> {
    let (tail, digits) = digit1.parse(source)?;
    let length = digits.len();
    // this only chokes if it gets an input
    // that evaluates to be greater than u64.MAX,
    // and will never have any tail
    let (_, value) = u64.parse(digits)?;

    Ok((tail, SchematicItem::Number { value, length }))
}

/// Parses a symbol (i.e. not a period or a number) in a schematic line.
fn symbol(source: &str) -> IResult<&str, SchematicItem> {
    let mut parser = map(
        not(number)
        .and(not(tag(".")))
        .and(anychar), 
        |(_, c)| SchematicItem::Symbol(c));

    parser.parse(source)
}

/// Parses a full schematic line
fn parse_schematic_line(source: &str) -> IResult<&str, Vec<Option<SchematicItem>>> {
    let mut parser = many1(
        alt((
            map(number, |n| Some(n)),
            map(symbol, |s| Some(s)),
            map(tag("."), |_| None),
        ))
    );

    parser.parse(source)
}

/// Reads all available lines from stdin and constructs a representation of the input.
fn get_full_input_from_stdin() -> anyhow::Result<Schematic> {
    Ok(io::stdin()
        .lines()
        .into_iter()
        .map(|line| 
            parse_schematic_line(&line.unwrap())
                .unwrap()
                .1
                .into_iter()
                .map(|item| match item {
                    Some(SchematicItem::Number { value, length }) => {
                        let mut vec = Vec::with_capacity(length + 1);
                        vec.push(Some(SchematicItem::Number { value, length }));
                        // pad additional length with None
                        for _ in 0..(length - 1) {
                            vec.push(None);
                        }
                        vec
                    }
                    sym @ Some(SchematicItem::Symbol(_)) => vec![sym],
                    None => vec![None],
                }).flatten().collect())
        .collect()
    )
}

fn get_adjacent_symbols(
    (i, j): (usize, usize), 
    schematic: &Schematic
) -> Option<Vec<SchematicItem>> {
    let Some(SchematicItem::Number { length, .. }) = schematic[i][j] else {
        return None;
    };

    let mut adjacent_items = Vec::with_capacity(2 * length + 6);

    // left edge
    if j > 0 {
        adjacent_items.push(schematic[i][j - 1]);

        if i > 0 {
            adjacent_items.push(schematic[i - 1][j - 1]);
        }

        if i < schematic.len() - 1 {
            adjacent_items.push(schematic[i + 1][j - 1]);
        }
    }

    // top and bottom edges
    for offset in 0..length {
        if i > 0 {
            adjacent_items.push(schematic[i - 1][j + offset]);
        }

        if i < schematic.len() - 1 {
            adjacent_items.push(schematic[i + 1][j + offset]);
        }
    }

    // right edge
    if j + length < schematic[i].len() - 1 {
        adjacent_items.push(schematic[i][j + length]);

        if i > 0 {
            adjacent_items.push(schematic[i - 1][j + length]);
        }

        if i < schematic.len() - 1 {
            adjacent_items.push(schematic[i + 1][j + length]);
        }
    }

    Some(
        adjacent_items
        .into_iter()
        .filter_map(|item| match item {
            sym @ Some(SchematicItem::Symbol(_)) => sym,
            _ => None
        })
        .collect()
    )
}

fn get_adjacent_indices(
    (i, j): (usize, usize), 
    schematic: &Schematic
) -> Option<Vec<(usize, usize)>> {
    let Some(SchematicItem::Number { length, .. }) = schematic[i][j] else {return None;};
    let mut adjacent_indices = Vec::with_capacity(2 * length + 6);

    if j > 0 {
        adjacent_indices.push((i, j - 1));

        if i > 0 {
            adjacent_indices.push((i - 1, j - 1));
        }

        if i < schematic.len() - 1 {
            adjacent_indices.push((i + 1, j - 1));
        }
    }

    for offset in 0..length {
        if i > 0 {
            adjacent_indices.push((i - 1, j + offset));
        }

        if i < schematic.len() - 1 {
            adjacent_indices.push((i + 1, j + offset));
        }
    }

    if j + length < schematic[i].len() - 1 {
        adjacent_indices.push((i, j + length));

        if i > 0 {
            adjacent_indices.push((i - 1, j + length));
        }

        if i < schematic.len() - 1 {
            adjacent_indices.push((i + 1, j + length));
        }
    }

    Some(adjacent_indices)
}

fn get_q1_result() -> anyhow::Result<usize> {
    let schematic = get_full_input_from_stdin()?;
    let mut part_number_sum = 0usize;

    for (i, line) in schematic.clone().into_iter().enumerate() {
        for (j, elem) in line.clone().into_iter().enumerate() {
            if let Some(SchematicItem::Number { value, .. }) = elem {
                if get_adjacent_symbols((i, j), &schematic).unwrap().len() > 0 { 
                    part_number_sum += value as usize; 
                }
            }
        }
    }
    
    Ok(part_number_sum)
}

fn get_q2_result() -> anyhow::Result<usize> {
    let schematic = get_full_input_from_stdin()?;
    let mut gear_candidates: HashMap<(usize, usize), Vec<usize>> = HashMap::with_capacity(1000);

    for (i, line) in schematic.clone().into_iter().enumerate() {
        for (j, elem) in line.into_iter().enumerate() {
            let Some(SchematicItem::Number { value, .. }) = elem else {continue;};
            let indices = get_adjacent_indices((i, j), &schematic).unwrap();

            for (x, y) in indices {
                if let Some(SchematicItem::Symbol('*')) = schematic[x][y] {
                    if let Some(vec) = gear_candidates.get(&(x, y)) {
                        let mut vec = vec.clone();
                        vec.push(value as usize);
                        gear_candidates.insert((i, j), vec);
                    } else {
                        gear_candidates.insert((x, y), vec![value as usize]);
                    }
                }
            }
        }
    }

    Ok(gear_candidates
        .into_iter()
        .filter_map(|(_, nums)| match nums.len() {
            2 => Some(nums[0] * nums[1]),
            _ => None,
        })
        .fold(0, |a, b| a + b)
    )
}

fn main() {
    let cli: Solution = argh::from_env();
    let res = match cli.question {
        Question::One => get_q1_result(),
        Question::Two => get_q2_result(),
    }.unwrap();
    eprintln!("{}", res);
}

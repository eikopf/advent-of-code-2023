use std::io;

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
    Symbol,
}

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
    let mut parser = not(number)
        .and(not(tag(".")))
        .and(anychar);

    // we don't actually care what symbol it was
    let (tail, _) = parser.parse(source)?;
    Ok((tail, SchematicItem::Symbol))
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
fn get_full_input_from_stdin() -> anyhow::Result<Vec<Vec<Option<SchematicItem>>>> {
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
                    Some(SchematicItem::Symbol) => vec![Some(SchematicItem::Symbol)],
                    None => vec![None],
                }).flatten().collect())
        .collect()
    )
}

fn get_q1_result() -> anyhow::Result<usize> {
    let schematic = get_full_input_from_stdin()?;
    let mut part_number_sum = 0usize;

    for (i, line) in schematic.clone().into_iter().enumerate() {
        for (j, elem) in line.clone().into_iter().enumerate() {
            if let Some(SchematicItem::Number { value, length }) = elem {
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

                if adjacent_items.contains(&Some(SchematicItem::Symbol)) { 
                    part_number_sum += value as usize; 
                }
            }
        }
    }
    
    Ok(part_number_sum)
}

fn main() {
    let res = get_q1_result().unwrap();
    eprintln!("{}", res);
}

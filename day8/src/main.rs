use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, multispace1, one_of},
    combinator::{map, map_res},
    error::Error,
    multi::{many1, separated_list1},
    sequence::{terminated, Tuple},
    Finish, IResult, Parser,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Side {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Node([char; 3]);

#[derive(Debug, Clone)]
struct Network(HashMap<Node, (Node, Node)>);

impl Network {
    /// Traverses the network according to the given path and
    /// returns the number of steps to get from AAA to ZZZ.
    fn traverse(&self, path: &mut impl Iterator<Item = Side>) -> usize {
        let mut count = 0;
        let mut current_node = Node(['A', 'A', 'A']);

        loop {
            if let Some(side) = path.next() {
                current_node = match side {
                    Side::Left => self.0.get(&current_node).unwrap().0,
                    Side::Right => self.0.get(&current_node).unwrap().1,
                };

                count += 1;
            } else {
                panic!("ran out of directions");
            }

            if current_node == Node(['Z', 'Z', 'Z']) {
                break;
            }
        }

        count
    }

    /// Traverses the network in lockstep, starting from all nodes which
    /// end with A, and terminating when the path produces a set of nodes
    /// which all end with Z; then the function returns the length of the
    /// traversal.
    fn ghost_traverse(&self, path: &mut impl Iterator<Item = Side>) -> usize {
        let mut count = 0;
        let mut current_nodes: Vec<Node> = self
            .0
            .keys()
            .filter(|&node| node.0[2] == 'A')
            .map(|&node| node)
            .collect();

        loop {
            if let Some(side) = path.next() {
                count += 1;
                current_nodes = current_nodes.into_iter().map(|node| match side {
                    Side::Left => self.0.get(&node).unwrap().0,
                    Side::Right => self.0.get(&node).unwrap().1,
                }).collect();
            } else {
                panic!("ran out of directions");
            }
            
            if current_nodes.iter().filter(|node| node.0[2] == 'Z').count() == current_nodes.len() { break; }
        }

        count
    }
}

/// Parses the first line of the source data.
fn path(source: &str) -> IResult<&str, Vec<Side>> {
    let mut parser = many1(map(one_of("RL"), |side| match side {
        'R' => Side::Right,
        'L' => Side::Left,
        _ => unreachable!(),
    }));

    parser.parse(source)
}

/// Parses a line from the "network" section of the source data and inserts
/// the result into the provided network.
fn mapping(source: &str) -> IResult<&str, (Node, (Node, Node))> {
    let mut parser = (
        map_res(terminated(alpha1, tag(" = (")), |chars: &str| {
            chars.chars().collect::<Vec<_>>().as_slice().try_into()
        }),
        map_res(terminated(alpha1, tag(", ")), |chars: &str| {
            chars.chars().collect::<Vec<_>>().as_slice().try_into()
        }),
        map_res(terminated(alpha1, tag(")")), |chars: &str| {
            chars.chars().collect::<Vec<_>>().as_slice().try_into()
        }),
    );
    let (tail, (source, left, right)) = parser.parse(source)?;
    Ok((tail, (Node(source), (Node(left), Node(right)))))
}

fn parse_input_data() -> anyhow::Result<(Vec<Side>, Network)> {
    let source = aoc::read_stdin_to_string();
    let mut parser = terminated(path, multispace1).and(separated_list1(multispace1, mapping));
    let (_, (path, mappings)) = parser.parse(&source).finish().map_err(|err| Error {
        input: err.input.to_string(),
        code: err.code,
    })?;

    let mut network = Network(HashMap::new());
    for (source, target) in mappings {
        network.0.insert(source, target);
    }

    Ok((path, network))
}

fn get_q1_result() -> anyhow::Result<usize> {
    let (path, network) = parse_input_data()?;
    let mut path_loop = path.into_iter().cycle();
    Ok(network.traverse(&mut path_loop))
}

fn get_q2_result() -> anyhow::Result<usize> {
    let (path, network) = parse_input_data()?;
    let mut path_loop = path.into_iter().cycle();
    Ok(network.ghost_traverse(&mut path_loop))
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

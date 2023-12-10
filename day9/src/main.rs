use nom::{
    character::complete::{multispace1, i64},
    combinator::map_res,
    multi::separated_list1,
    IResult, Parser,
};

/// Represents a single line from the source data.
struct History {
    /// The literal values of a source data line.
    sequence: Vec<isize>,
}

impl History {
    /// Returns the trailing edge of the difference stack,
    /// such that the sum of the resulting vector is the
    /// extrapolated value as described in question 1.
    fn get_diff_stack_trailing_edge(&self) -> Vec<isize> {
        let mut edge = Vec::new();
        let mut derivative = self.sequence.clone();

        while derivative.iter().filter(|&&x| x != 0).count() != 0 {
            edge.push(*derivative.last().unwrap());
            derivative = diff(derivative);
        }

        edge
    }
}

/// Computes the first difference of the given vector.
fn diff(vec: Vec<isize>) -> Vec<isize> {
    vec.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>()
}

/// Parses a single line from the source data.
fn parse_source_line(source: &str) -> IResult<&str, Vec<isize>> {
    let mut parser = separated_list1(multispace1, map_res(i64, |x| isize::try_from(x)));

    parser.parse(source)
}

fn get_q1_result() -> anyhow::Result<isize> {
    Ok(aoc::read_stdin_by_line()
        .map(|line| parse_source_line(&line.unwrap()).unwrap().1)
        .map(|sequence| History { sequence })
        .map(|history| history.get_diff_stack_trailing_edge().into_iter().sum::<isize>())
        .sum())
}

fn get_q2_result() -> anyhow::Result<isize> {
    todo!()
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

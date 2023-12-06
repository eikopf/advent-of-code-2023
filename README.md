# Advent of Code 2023
> In which I write stupid code to complete easy problems in unnecessarily complex ways.

## Usage
All the solutions use the same `argh` interface (defined in the `aoc` crate), so running a solution just looks like this:

```sh
# run the solution to day 7 question 2
cat data/day7.txt | cargo run -p day7 -- --question 2

# run the solution to day 3 question 1
cat data/day3.txt | cargo run -p day3 -- -q 1
```

If you have [`just`](https://github.com/casey/just) installed, the following commands also work:

```sh
# run all the questions from all the days sequentially
just run-all

# run both questions from a particular day
just run-day

# run a particular question from a particular day
# (ordered as day then question)
just run 6 2

# with hyperfine installed, you can also time a particular solution
just time 4 1
```

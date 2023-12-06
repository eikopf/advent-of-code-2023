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

## Notes
> Days 1, 2, and 3 are ugly as sin; just trust me that they're not worth writing about.

### Day 4
`TODO`

### Day 5
#### Question 1
For question 1, an interesting point is that even though each `CategoryMap` is composed of disjoint mappings (represented by `RangeMap`s), an issue can arise where sequentially applying them accidentally applies several transformations to an individual seed.

> This is caused by mappings whose targets (destinations) overlap with the domains (sources) of later mappings.

The solution for this was a `TransformLock` enum, defined as follows:
```rust
enum TransformLock<T> {
    Locked(T),
    Unlocked(T),
}
```
This basically serves to enforce idempotency on a per-category level: the inputs are first wrapped in the unlocked variant before the transformation, and then they are locked if they are transformed. The `RangeMap`'s `apply` method respects this lock, and the `CategoryMap`'s `apply` method wraps the given input elementwise when called, and then strips the locks before returning.

#### Question 2
It's important to note that the image of a range under a `RangeMap` (that is, the set of the images of all the points in the range) will be a collection of ranges; in particular the resulting ranges will form a collective disjoint subset of the original range.

A range $[a, b)$ "splits" into multiple ranges if it is partially covered by the source range $[c, d)$ of a `RangeMap` whose mapping is a function $f:[c, d)\to[e, f)$: consider the following three cases:

1. $`c\in[a, b)`$, which generates under $f$ the ranges $[a, c)$ and $\{f(x):x\in[c, b)\}$;
2. $d\in[a, b)$, which generates under $f$ the ranges $\{f(x):x\in[a, d)\}$ and $[d, b)$;
3. $c,d\in[a, b)$, which generates under $f$ the ranges $[a, c)$, $[d, b)$, and $`\{f(x):x\in[c, d)\}`$.

Otherwise if $[c, d)$ totally covers $[a, b)$ a single range is generated (the image of $[a, b)$ under $f$), and if the two ranges are disjoint then $[a, b)$ is unchanged.

This procedure can be iterated for each `RangeMap`, and then for each `CategoryMap`, to subdivide the initial set of seed-ranges (as defined in question 2) into a larger set of ranges. At the end of the process, the set of ranges (as a `Vec<Range>`) will be a list of the locations which have corresponding seeds; the solution is then just to find the minimum among their lower bounds.

### Day 6
> This day was shockingly easy compared to my 18 hours of suffering on day 5.
#### Question 1
The question ultimately becomes something like: "Here is a list of numbers; how many pairs of natural numbers which sum to each number have a corresponding product greater than the given threshold?"

The one caveat to this representation is that symmetries are counted as different solutions, i.e. $(3, 4)$ and $(4, 3)$ are both valid solutions.

This was pretty trivial to implement as an iterator over ranges, and the final solution runs in about 1.3 μs.

#### Question 2
The dumb thing worked: I just rewrote the parser to produce a single `Race` struct, and then filtered over `(0..=duration).zip((0..=duration).rev())`. That came out to about 130 μs, and since `rayon` only lets you call `zip` on `Range<u16>` or smaller, that's the best I'm going to do for today.

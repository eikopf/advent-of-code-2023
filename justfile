default:
  @just --list --list-prefix "> "

run-all:
  cat data/day1.txt | cargo run -p day1 -- --question 1
  cat data/day1.txt | cargo run -p day1 -- --question 2
  cat data/day2.txt | cargo run -p day2 -- --question 1
  cat data/day2.txt | cargo run -p day2 -- --question 2
  cat data/day3.txt | cargo run -p day3 -- --question 1
  cat data/day3.txt | cargo run -p day3 -- --question 2
  cat data/day4.txt | cargo run -p day4 -- --question 1
  cat data/day4.txt | cargo run -p day4 -- --question 2
  cat data/day5.txt | cargo run -p day5 -- --question 1
  @echo //////// skipping day 5 question 2 ////////
  @#cat data/day5.txt | cargo run -p day5 -- --question 2
  cat data/day6.txt | cargo run -p day6 -- --question 1
  cat data/day6.txt | cargo run -p day6 -- --question 2
  cat data/day7.txt | cargo run -p day7 -- --question 1
  cat data/day7.txt | cargo run -p day7 -- --question 2

run-day day:
  cat data/day{{day}}.txt | cargo run --bin day{{day}} -- --question 1
  cat data/day{{day}}.txt | cargo run --bin day{{day}} -- --question 2

run day question:
  cat data/day{{day}}.txt | cargo run --bin day{{day}} -- --question {{question}}

time day question:
  hyperfine -w 5 "cat data/day{{day}}.txt | cargo run --bin day{{day}} -- -q {{question}}"

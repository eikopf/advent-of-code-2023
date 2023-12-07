use std::collections::HashMap;

use nom::{
    bytes::complete::take,
    character::complete::{multispace1, u32},
    combinator::{map, map_res},
    sequence::terminated,
    IResult, Parser,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(PartialEq, Eq)]
struct Hand {
    cards: [Card; 5],
    hand_type: HandType,
}

impl From<[Card; 5]> for Hand {
    fn from(value: [Card; 5]) -> Self {
        let mut card_counts = HashMap::new();

        for card in value {
            if let Some(count) = card_counts.get(&card) {
                card_counts.insert(card, count + 1);
            } else {
                card_counts.insert(card, 1);
            }
        };

        // matching on products is the dumbest possible way
        // to handle cases with the same number of unique cards,
        // but it does work.
        let hand_type = match card_counts.len() {
            // direct cases
            5 => HandType::HighCard,
            4 => HandType::OnePair,
            1 => HandType::FiveOfAKind,
            // two pair or three of a kind
            3 => match card_counts.values().into_iter().product() {
                3 => HandType::ThreeOfAKind,
                4 => HandType::TwoPair,
                _ => panic!("invalid hand"),
            }
            // full house or four of a kind
            2 => match card_counts.values().into_iter().product() {
                6 => HandType::FullHouse,
                4 => HandType::FourOfAKind,
                _ => panic!("invalid hand"),
            }
            _ => panic!("not enough cards"),
        };

        Self { cards: value, hand_type }
    }
}

/// This trait implements the type-based partial ordering.
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self.hand_type, other.hand_type) {
            (a, b) if a > b => Some(std::cmp::Ordering::Greater),
            (a, b) if b > a => Some(std::cmp::Ordering::Less),
            _ => None,
        }
    }
}

/// This trait implements the tie-breaker ordering.
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.hand_type == other.hand_type {
            self.cards
                .into_iter()
                .zip(other.cards.into_iter())
                .filter_map(|(left, right)| match Card::cmp(&left, &right) {
                    std::cmp::Ordering::Equal => None,
                    ord => Some(ord),
                })
                .nth(0)
                .unwrap_or(std::cmp::Ordering::Equal)
        } else {
            Hand::partial_cmp(&self, other).unwrap()
        }
    }
}

/// Parses a single line from the input and returns it.
fn parse_hand_and_bid(source: &str) -> IResult<&str, ([Card; 5], usize)> {
    let mut parser = terminated(
        map(map(take(5usize), |s: &str| s.chars()), |chars| {
            chars.map(|c| match c {
                '2' => Card::Two,
                '3' => Card::Three,
                '4' => Card::Four,
                '5' => Card::Five,
                '6' => Card::Six,
                '7' => Card::Seven,
                '8' => Card::Eight,
                '9' => Card::Nine,
                'T' => Card::Ten,
                'J' => Card::Jack,
                'Q' => Card::Queen,
                'K' => Card::King,
                'A' => Card::Ace,
                _ => panic!("got an invalid character"),
            })
        }),
        multispace1,
    )
    .and(map_res(u32, |x| usize::try_from(x)));

    parser.parse(source).map(|(tail, (cards, bid))| {
        (
            tail,
            (
                cards.collect::<Vec<_>>().as_slice().try_into().unwrap(),
                bid,
            ),
        )
    })
}

fn get_q1_result() -> anyhow::Result<usize> {
    let lines = aoc::read_stdin_by_line();
    let mut hands = lines
        .into_iter()
        .map(|line| parse_hand_and_bid(&line.unwrap()).unwrap().1)
        .map(|(cards, bid)| (Hand::from(cards), bid))
        .collect::<Vec<_>>();

    hands.sort_by(|(a, _), (b, _)| Hand::cmp(a, b));

    Ok(hands
        .into_iter()
        .map(|(_, bid)| bid)
        .enumerate()
        .map(|(rank, bid)| (rank + 1) * bid)
        .sum())
}

fn get_q2_result() -> anyhow::Result<usize> {
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

use std::{collections::HashMap, str::Chars};

use nom::{
    bytes::complete::take,
    character::complete::{multispace1, u32},
    combinator::{map, map_res},
    sequence::terminated,
    IResult, Parser,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum StandardCard {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum JokerCard {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
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
struct Hand<Card> {
    cards: [Card; 5],
    hand_type: HandType,
}

impl From<[StandardCard; 5]> for Hand<StandardCard> {
    fn from(value: [StandardCard; 5]) -> Self {
        let mut card_counts = HashMap::new();

        for card in value {
            if let Some(count) = card_counts.get(&card) {
                card_counts.insert(card, count + 1);
            } else {
                card_counts.insert(card, 1);
            }
        }

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
            },
            // full house or four of a kind
            2 => match card_counts.values().into_iter().product() {
                6 => HandType::FullHouse,
                4 => HandType::FourOfAKind,
                _ => panic!("invalid hand"),
            },
            _ => panic!("not enough cards"),
        };

        Self {
            cards: value,
            hand_type,
        }
    }
}

impl From<[JokerCard; 5]> for Hand<JokerCard> {
    fn from(value: [JokerCard; 5]) -> Self {
        let mut card_counts = HashMap::new();

        for card in value {
            if let Some(count) = card_counts.get(&card) {
                card_counts.insert(card, count + 1);
            } else {
                card_counts.insert(card, 1);
            }
        }

        // if we have any jokers, set them as whatever
        // card we have the most of.
        if let Some(joker_count) = card_counts.remove(&JokerCard::Joker) {
            if card_counts.is_empty() {
                // this handles the case where the map
                // only contains jokers, i.e. where the
                // input is JJJJJ
                card_counts.insert(JokerCard::Joker, 0);
            }

            // get a key corresponding to the max value
            let max_key = card_counts
                .iter()
                .max_by(|(_, &a), (_, &b)| a.cmp(&b))
                .map(|(key, _)| key)
                .expect("at least one card");

            let max_count = card_counts.get(max_key).unwrap();
            card_counts.insert(*max_key, max_count + joker_count);
        }

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
            },
            // full house or four of a kind
            2 => match card_counts.values().into_iter().product() {
                6 => HandType::FullHouse,
                4 => HandType::FourOfAKind,
                _ => panic!("invalid hand"),
            },
            _ => panic!("not enough cards"),
        };

        Self {
            cards: value,
            hand_type,
        }
    }
}

/// This trait implements the type-based partial ordering.
impl<T: PartialOrd> PartialOrd for Hand<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self.hand_type, other.hand_type) {
            (a, b) if a > b => Some(std::cmp::Ordering::Greater),
            (a, b) if b > a => Some(std::cmp::Ordering::Less),
            _ => None,
        }
    }
}

/// This trait implements the tie-breaker ordering.
impl<T: Eq + Ord + Copy> Ord for Hand<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.hand_type == other.hand_type {
            self.cards
                .into_iter()
                .zip(other.cards.into_iter())
                .filter_map(|(left, right)| match T::cmp(&left, &right) {
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

/// Parses a single line from the input and returns it, using the provided
/// closure to map the parsed characters in the "hand" section of the data
/// to a [Vec<T>].
fn parse_hand_and_bid<T, F>(source: &str, f: F) -> IResult<&str, ([T; 5], usize)>
where
    F: for<'a> Fn(Chars<'a>) -> Vec<T>,
    [T; 5]: for<'a> TryFrom<&'a [T]> + std::fmt::Debug,
    for<'a> <[T; 5] as TryFrom<&'a [T]>>::Error: std::fmt::Debug,
{
    let mut parser = terminated(
        map(map(take(5usize), |s: &str| s.chars()), f),
        multispace1,
    )
    .and(map_res(u32, |x| usize::try_from(x)));

    parser.parse(source).map(|(tail, (cards, bid))| {
        (
            tail,
            (
                cards.as_slice().try_into().unwrap(),
                bid,
            ),
        )
    })
}

fn get_q1_result() -> anyhow::Result<usize> {
    let lines = aoc::read_stdin_by_line();
    let mut hands = lines
        .into_iter()
        .map(|line| parse_hand_and_bid(&line.unwrap(), 
            |chars| chars.map(|c| match c {
                '2' => StandardCard::Two,
                '3' => StandardCard::Three,
                '4' => StandardCard::Four,
                '5' => StandardCard::Five,
                '6' => StandardCard::Six,
                '7' => StandardCard::Seven,
                '8' => StandardCard::Eight,
                '9' => StandardCard::Nine,
                'T' => StandardCard::Ten,
                'J' => StandardCard::Jack,
                'Q' => StandardCard::Queen,
                'K' => StandardCard::King,
                'A' => StandardCard::Ace,
                _ => panic!("got an invalid character"),
            }).collect()).unwrap().1)
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
    let lines = aoc::read_stdin_by_line();
    let mut hands = lines
        .into_iter()
        .map(|line| parse_hand_and_bid(&line.unwrap(), 
            |chars| chars.map(|c| match c {
                'J' => JokerCard::Joker,
                '2' => JokerCard::Two,
                '3' => JokerCard::Three,
                '4' => JokerCard::Four,
                '5' => JokerCard::Five,
                '6' => JokerCard::Six,
                '7' => JokerCard::Seven,
                '8' => JokerCard::Eight,
                '9' => JokerCard::Nine,
                'T' => JokerCard::Ten,
                'Q' => JokerCard::Queen,
                'K' => JokerCard::King,
                'A' => JokerCard::Ace,
                _ => panic!("got an invalid character"),
            }).collect()).unwrap().1)
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

fn main() {
    let cli = aoc::Solution::new();
    let res = match cli.question {
        aoc::Question::One => get_q1_result(),
        aoc::Question::Two => get_q2_result(),
    }
    .unwrap();
    println!("{}", res);
}

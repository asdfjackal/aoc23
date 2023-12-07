use itertools::Itertools;
use std::cmp::Ordering;

const CARDS: &str = "J23456789TQKA";

fn get_card_value(card: char) -> usize {
    CARDS.find(card).unwrap()
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn get_type(input: &str) -> HandType {
    use HandType::*;

    let mut card_map = input.chars().counts();
    let jokers: usize = *card_map.get(&'J').unwrap_or(&0usize);
    if jokers == 5 {
        return FiveOfAKind;
    }
    card_map.remove(&'J');
    let mut counts = card_map.values().collect_vec();
    counts.sort();
    let counts_string = counts
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("");
    let counts_string = (counts_string.parse::<u64>().unwrap() + jokers as u64).to_string();
    match counts_string.as_str() {
        "5" => FiveOfAKind,
        "14" => FourOfAKind,
        "23" => FullHouse,
        "113" => ThreeOfAKind,
        "122" => TwoPair,
        "1112" => OnePair,
        _ => HighCard,
    }
}

#[derive(Eq, PartialEq, Debug)]
struct Hand {
    cards: String,
    hand_type: HandType,
    bid: u64,
}

impl Hand {
    fn from_slice(input: &str, bid: u64) -> Self {
        Hand {
            cards: input.to_string(),
            hand_type: get_type(input),
            bid,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand_type > other.hand_type {
            return Ordering::Less;
        }
        if self.hand_type < other.hand_type {
            return Ordering::Greater;
        }
        for i in 0..5 {
            if get_card_value(self.cards.as_bytes()[i] as char)
                > get_card_value(other.cards.as_bytes()[i] as char)
            {
                return Ordering::Less;
            }
            if get_card_value(self.cards.as_bytes()[i] as char)
                < get_card_value(other.cards.as_bytes()[i] as char)
            {
                return Ordering::Greater;
            }
        }
        Ordering::Equal
    }
}
fn main() {
    let input = include_str!("../input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let mut hands: Vec<Hand> = input
        .trim()
        .lines()
        .map(|x| {
            let mut row = x.trim().split(' ');
            let cards = row.next().unwrap();
            let bid: u64 = row.next().unwrap().parse().unwrap();
            Hand::from_slice(cards, bid)
        })
        .collect();
    hands.sort();
    hands.reverse();
    let sum: u64 = hands
        .iter()
        .enumerate()
        .map(|(i, hand)| hand.bid * (i as u64 + 1))
        .sum();
    sum.to_string()
}

#[cfg(test)]
mod tests_2 {
    use super::*;

    #[test]
    fn part_2() {
        let input = include_str!("../test1.txt");
        assert_eq!("5905", part1(input));
    }
}

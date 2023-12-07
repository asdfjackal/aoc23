use std::cmp::Ordering;
use std::collections::HashMap;

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

impl HandType {
    fn get_type(input: &[char; 5]) -> HandType {
        let mut card_map = input.iter().fold(HashMap::new(), |mut map, x| {
            *map.entry(x).or_insert(0) += 1;
            map
        });
        let jokers: u64 = *card_map.get(&'J').unwrap_or(&0);
        card_map.remove(&'J');
        let (_, mut counts): (Vec<&char>, Vec<&u64>) = card_map.iter().unzip();
        counts.sort();
        counts.reverse();
        let counts_string = counts
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("");
        if jokers >= 4 {
            return HandType::FiveOfAKind;
        }

        if jokers == 3 {
            if counts_string.starts_with('2') {
                return HandType::FiveOfAKind;
            }
            return HandType::FourOfAKind;
        }

        if jokers == 2 {
            if counts_string.starts_with('3') {
                return HandType::FiveOfAKind;
            }
            if counts_string.starts_with('2') {
                return HandType::FourOfAKind;
            }
            return HandType::ThreeOfAKind;
        }

        if jokers == 1 {
            if counts_string.starts_with('4') {
                return HandType::FiveOfAKind;
            }
            if counts_string.starts_with('3') {
                return HandType::FourOfAKind;
            }
            if counts_string.starts_with("22") {
                return HandType::FullHouse;
            }
            if counts_string.starts_with('2') {
                return HandType::ThreeOfAKind;
            }
            return HandType::OnePair;
        }

        if counts_string.starts_with('5') {
            return HandType::FiveOfAKind;
        }
        if counts_string.starts_with('4') {
            return HandType::FourOfAKind;
        }
        if counts_string.starts_with("32") {
            return HandType::FullHouse;
        }
        if counts_string.starts_with('3') {
            return HandType::ThreeOfAKind;
        }
        if counts_string.starts_with("22") {
            return HandType::TwoPair;
        }
        if counts_string.starts_with('2') {
            return HandType::OnePair;
        }
        HandType::HighCard
    }
}

#[derive(Eq, PartialEq, Debug)]
struct Hand {
    cards: [char; 5],
    hand_type: HandType,
    bid: u64,
}

impl Hand {
    fn from_slice(input: &[char; 5], bid: u64) -> Self {
        Hand {
            cards: *input,
            hand_type: HandType::get_type(input),
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
            if get_card_value(self.cards[i]) > get_card_value(other.cards[i]) {
                return Ordering::Less;
            }
            if get_card_value(self.cards[i]) < get_card_value(other.cards[i]) {
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
            let mut cards = [' '; 5];
            row.next()
                .unwrap()
                .chars()
                .zip(cards.iter_mut())
                .for_each(|(c, ptr)| *ptr = c);
            let bid: u64 = row.next().unwrap().parse().unwrap();
            Hand::from_slice(&cards, bid)
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

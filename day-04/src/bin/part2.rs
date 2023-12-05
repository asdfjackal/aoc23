use std::collections::HashSet;

fn main() {
    let input = include_str!("../input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn extract_nums(input: &str) -> HashSet<u32> {
    let mut output = HashSet::new();
    input
        .trim()
        .split_ascii_whitespace()
        .map(|x| x.trim().parse().unwrap())
        .for_each(|x| {
            output.insert(x);
        });
    output
}

fn part1(input: &str) -> String {
    let mut total: u32 = 0;
    let mut extra_cards: Vec<u32> = Vec::new();
    input.trim().lines().for_each(|line| {
        let mut num_cards = 1;
        if !extra_cards.is_empty() {
            num_cards += extra_cards.remove(0);
        }
        let raw_card = line.split_once(':').unwrap().1.trim();
        let (raw_picks, raw_winners) = raw_card.split_once('|').unwrap();
        let picks = extract_nums(raw_picks);
        let winners = extract_nums(raw_winners);
        let winning_picks: Vec<u32> = picks.intersection(&winners).cloned().collect();
        let num_winners: u32 = winning_picks.len().try_into().unwrap();
        for i in 0..num_winners {
            if extra_cards.len() as u32 > i {
                extra_cards[i as usize] += num_cards;
            } else {
                extra_cards.push(num_cards);
            }
        }
        total += num_cards;
    });
    total.to_string()
}

#[cfg(test)]
mod tests_2 {
    use super::*;

    #[test]
    fn part_2() {
        let input = include_str!("../test1.txt");
        assert_eq!("30", part1(input));
    }
}

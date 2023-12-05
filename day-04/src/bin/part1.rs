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
    let sum: u32 = input
        .trim()
        .lines()
        .map(|line| {
            println!("{}", line);
            let raw_card = line.split_once(':').unwrap().1.trim();
            let (raw_picks, raw_winners) = raw_card.split_once('|').unwrap();
            let picks = extract_nums(raw_picks);
            let winners = extract_nums(raw_winners);
            let winning_picks: Vec<u32> = picks.intersection(&winners).cloned().collect();
            if winning_picks.is_empty() {
                return 0;
            }
            let num_winners: u32 = winning_picks.len().try_into().unwrap();
            u32::pow(2, num_winners - 1)
        })
        .sum();
    sum.to_string()
}

#[cfg(test)]
mod tests_1 {
    use super::*;

    #[test]
    fn part_1() {
        let input = include_str!("../test1.txt");
        assert_eq!("13", part1(input));
    }
}

struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn winning_scenarios(&self) -> u64 {
        let options: Vec<u64> = (1..self.time).collect();
        let winning_options: Vec<u64> = options
            .iter()
            .map(|x| x * (self.time - x))
            .filter(|x| x > &self.distance)
            .collect();
        winning_options.len() as u64
    }
}

fn main() {
    let input = include_str!("../input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let mut lines = input.trim().lines();
    let raw_times = lines.next().unwrap();
    let raw_distances = lines.next().unwrap();
    let time: u64 = raw_times
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .split_ascii_whitespace()
        .collect::<String>()
        .parse()
        .unwrap();
    let distance: u64 = raw_distances
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .split_ascii_whitespace()
        .collect::<String>()
        .parse()
        .unwrap();
    let race = Race { time, distance };
    race.winning_scenarios().to_string()
}

#[cfg(test)]
mod tests_2 {
    use super::*;

    #[test]
    fn part_2() {
        let input = include_str!("../test1.txt");
        assert_eq!("71503", part1(input));
    }
}

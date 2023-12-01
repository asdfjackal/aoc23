const WORD_DIGITS: &[&str] = &[
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    let input = include_str!("../input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    let sum: u32 = input.trim().split('\n').map(parse_line).sum();
    sum.to_string()
}

fn parse_line(input: &str) -> u32 {
    let mut raw = input;
    let mut digits: Vec<u32> = Vec::new();
    'outer: while !raw.is_empty() {
        let char = raw.chars().next().unwrap();
        if char.is_ascii_digit() {
            digits.push(char.to_digit(10).unwrap());
            raw = &raw[1..];
            continue;
        }
        for (i, word) in WORD_DIGITS.iter().enumerate() {
            if raw.starts_with(word) {
                digits.push(u32::try_from(i + 1).unwrap());
                raw = &raw[1..];
                continue 'outer;
            }
        }
        raw = &raw[1..]
    }
    (digits.first().unwrap() * 10) + digits.last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_2() {
        let input = include_str!("../test2.txt");
        assert_eq!(part2(input), "281");
    }
}

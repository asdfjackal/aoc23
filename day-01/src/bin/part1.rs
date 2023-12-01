fn main() {
    let input = include_str!("../input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let sum: u32 = input.trim().split('\n').map(parse_line).sum();
    sum.to_string()
}

fn parse_line(input: &str) -> u32 {
    let all_chars: Vec<char> = input.chars().collect();
    let num_chars: Vec<u32> = all_chars
        .iter()
        .filter(|x| x.is_ascii_digit())
        .map(|x| x.to_digit(10).unwrap())
        .collect();
    (num_chars.first().unwrap() * 10) + num_chars.last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = include_str!("../test1.txt");
        assert_eq!(part1(input), "142");
    }
}

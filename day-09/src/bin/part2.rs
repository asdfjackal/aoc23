fn main() {
    let input = include_str!("../input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn get_difs(input: &[i32]) -> Vec<i32> {
    let first: Vec<i32> = input[0..input.len() - 1].into();
    let last: Vec<i32> = input[1..].into();
    first.iter().zip(last.iter()).map(|(f, l)| l - f).collect()
}

fn process_line(input: Vec<i32>) -> i32 {
    if input.iter().copied().all(|x| x == 0) {
        return 0;
    }
    let difs = get_difs(input.as_slice());
    let interp = input.first().unwrap() - process_line(difs);
    interp
}

fn part2(input: &str) -> String {
    input
        .trim()
        .lines()
        .map(|line| {
            process_line(
                line.split_ascii_whitespace()
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect(),
            )
        })
        .sum::<i32>()
        .to_string()
}

#[cfg(test)]
mod tests_2 {
    use super::*;

    #[test]
    fn test_get_difs() {
        let input = vec![10, 13, 16, 21, 30, 45, 68];
        assert_eq!(vec![3, 3, 5, 9, 15, 23], get_difs(input.as_slice()));
    }

    #[test]
    fn test_process_line() {
        let input = vec![10, 13, 16, 21, 30, 45];
        assert_eq!(5, process_line(input));
    }

    #[test]
    fn part_2() {
        let input = include_str!("../test1.txt");
        assert_eq!("2", part2(input));
    }
}

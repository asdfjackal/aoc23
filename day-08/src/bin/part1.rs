use std::collections::HashMap;

struct Branch {
    left: String,
    right: String,
}

fn main() {
    let input = include_str!("../input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let (sequence, rest) = input.trim().split_once("\n\n").unwrap();
    let branches: HashMap<String, Branch> = rest
        .trim()
        .lines()
        .map(|x| {
            let (raw_key, raw_branch) = x.trim().split_once(" = ").unwrap();
            let (raw_left, raw_right) = raw_branch
                .strip_prefix('(')
                .unwrap()
                .strip_suffix(')')
                .unwrap()
                .split_once(", ")
                .unwrap();

            (
                raw_key.to_string(),
                Branch {
                    left: raw_left.to_string(),
                    right: raw_right.to_string(),
                },
            )
        })
        .collect();
    let mut steps = 0;
    let mut current: String = "AAA".to_string();
    loop {
        let direction = sequence.chars().nth(steps % sequence.len()).unwrap();
        current = match direction {
            'L' => branches.get(&current).unwrap().left.clone(),
            'R' => branches.get(&current).unwrap().right.clone(),
            _ => panic!(),
        };
        steps += 1;
        if current == "ZZZ" {
            break;
        }
    }
    steps.to_string()
}

#[cfg(test)]
mod tests_1 {
    use super::*;

    #[test]
    fn input_1() {
        let input = include_str!("../test1.txt");
        assert_eq!("2", part1(input));
    }
    #[test]
    fn input_2() {
        let input = include_str!("../test2.txt");
        assert_eq!("6", part1(input));
    }
}

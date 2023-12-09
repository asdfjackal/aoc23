use num::integer::lcm;
use std::collections::HashMap;

struct Branch {
    left: String,
    right: String,
}

fn main() {
    let input = include_str!("../input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
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
    let mut steps: u64 = 0;
    let mut current: Vec<String> = branches
        .keys()
        .filter(|x| x.ends_with('A'))
        .cloned()
        .collect();
    let mut cycle_steps = Vec::new();
    loop {
        let direction_index: usize = (steps % sequence.len() as u64) as usize;
        let direction = sequence.chars().nth(direction_index).unwrap();
        current = current
            .iter()
            .map(|x| match direction {
                'L' => branches.get(x).unwrap().left.clone(),
                'R' => branches.get(x).unwrap().right.clone(),
                _ => panic!(),
            })
            .collect();
        steps += 1;
        let (finished, not_finished): (Vec<String>, Vec<String>) =
            current.iter().cloned().partition(|x| x.ends_with('Z'));
        if !finished.is_empty() {
            cycle_steps.push(steps);
        }
        current = not_finished;
        if current.is_empty() {
            break;
        }
    }
    cycle_steps
        .iter()
        .fold(1, |acc, e| lcm(acc, *e))
        .to_string()
}

#[cfg(test)]
mod tests_2 {
    use super::*;

    #[test]
    fn input_3() {
        let input = include_str!("../test3.txt");
        assert_eq!("6", part2(input));
    }
}

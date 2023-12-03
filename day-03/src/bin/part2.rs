use regex::Regex;

struct EngineNumber {
    value: i32,
    row: i32,
    start: i32,
    end: i32,
}

struct EngineGear {
    row: i32,
    column: i32,
}

impl EngineGear {
    fn ratio(&self, numbers: &[EngineNumber]) -> i32 {
        let adjacent: Vec<&EngineNumber> = numbers
            .iter()
            .filter(|x| {
                self.row <= x.row + 1
                    && self.row >= x.row - 1
                    && self.column <= x.end + 1
                    && self.column >= x.start - 1
            })
            .collect();
        if adjacent.len() == 2 {
            return adjacent.iter().map(|x| x.value).product();
        }
        0
    }
}

fn main() {
    let input = include_str!("../input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let mut numbers = Vec::new();
    let mut gears = Vec::new();
    let number_regex = Regex::new(r"\d+").unwrap();
    let symbol_regex = Regex::new(r"\*").unwrap();
    for (i, row) in input.trim().split('\n').enumerate() {
        let row_num = i32::try_from(i).unwrap();
        number_regex.find_iter(row).for_each(|x| {
            numbers.push(EngineNumber {
                row: row_num,
                start: i32::try_from(x.start()).unwrap(),
                end: i32::try_from(x.end()).unwrap() - 1,
                value: x.as_str().parse().unwrap(),
            });
        });
        symbol_regex.find_iter(row).for_each(|x| {
            gears.push(EngineGear {
                row: row_num,
                column: i32::try_from(x.start()).unwrap(),
            })
        })
    }
    let sum: i32 = gears.iter().map(|x| x.ratio(numbers.as_slice())).sum();
    sum.to_string()
}

#[cfg(test)]
mod tests_2 {
    use super::*;

    #[test]
    fn part_2() {
        let input = include_str!("../test1.txt");
        assert_eq!("467835", part1(input));
    }
}

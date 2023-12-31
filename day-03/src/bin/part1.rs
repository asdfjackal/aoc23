use regex::Regex;

struct EngineNumber {
    value: i32,
    row: i32,
    start: i32,
    end: i32,
}

impl EngineNumber {
    fn is_adjacent(&self, symbols: &[EngineSymbol]) -> bool {
        symbols.iter().any(|x| {
            x.row <= self.row + 1
                && x.row >= self.row - 1
                && x.column <= self.end + 1
                && x.column >= self.start - 1
        })
    }
}

struct EngineSymbol {
    row: i32,
    column: i32,
}

fn main() {
    let input = include_str!("../input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let mut numbers = Vec::new();
    let mut symbols = Vec::new();
    let number_regex = Regex::new(r"\d+").unwrap();
    let symbol_regex = Regex::new(r"[^\d\.]").unwrap();
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
            symbols.push(EngineSymbol {
                row: row_num,
                column: i32::try_from(x.start()).unwrap(),
            })
        })
    }
    let sum: i32 = numbers
        .iter()
        .filter(|x| x.is_adjacent(symbols.as_slice()))
        .map(|x| x.value)
        .sum();
    sum.to_string()
}

#[cfg(test)]
mod tests_1 {
    use super::*;

    #[test]
    fn part_1() {
        let input = include_str!("../test1.txt");
        assert_eq!("4361", part1(input));
    }

    #[test]
    fn is_adjacent() {
        let number = EngineNumber {
            value: 123,
            start: 2,
            end: 4,
            row: 2,
        };
        let symbol = EngineSymbol { row: 3, column: 5 };
        assert!(number.is_adjacent(vec![symbol].as_slice()))
    }
}

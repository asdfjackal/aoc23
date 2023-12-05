struct SeedMap {
    start: i64,
    end: i64,
    offset: i64,
}

impl SeedMap {
    fn from_string(input: &str) -> Self {
        let mut split = input.trim().split(' ');
        let destination: i64 = split.next().unwrap().parse().unwrap();
        let source: i64 = split.next().unwrap().parse().unwrap();
        let range: i64 = split.next().unwrap().parse().unwrap();
        SeedMap {
            start: source,
            end: source + (range - 1),
            offset: destination - source,
        }
    }

    fn transform(&self, input: i64) -> i64 {
        if input >= self.start && input <= self.end {
            return input + self.offset;
        }
        input
    }
}

fn main() {
    let input = include_str!("../input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let mut inputs = input.trim().split("\n\n");
    let seeds: Vec<i64> = inputs
        .next()
        .unwrap()
        .trim()
        .split_once(": ")
        .unwrap()
        .1
        .trim()
        .split(' ')
        .map(|x| x.parse().unwrap())
        .collect();

    let transformers: Vec<Vec<SeedMap>> = inputs
        .map(|x| {
            let mut lines = x.trim().lines();
            lines.next();
            lines.map(SeedMap::from_string).collect()
        })
        .collect();
    let min: i64 = seeds
        .iter()
        .map(|x| {
            let mut output = *x;
            for maps in transformers.as_slice() {
                for map in maps {
                    let result = map.transform(output);
                    if result != output {
                        output = result;
                        break;
                    }
                }
            }
            output
        })
        .min()
        .unwrap();
    min.to_string()
}

#[cfg(test)]
mod tests_1 {
    use super::*;

    #[test]
    fn part_1() {
        let input = include_str!("../test1.txt");
        assert_eq!("35", part1(input));
    }
}

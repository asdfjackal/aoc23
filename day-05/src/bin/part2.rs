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

#[derive(Copy, Clone)]
struct Seed {
    start: i64,
    range: i64,
}

fn prune_seeds(seeds: &[Seed]) -> Vec<Seed> {
    let mut output: Vec<Seed> = Vec::new();
    let mut end = -1;
    for seed in seeds {
        if seed.start + seed.range - 1 <= end {
            continue;
        }
        if seed.start > end {
            output.push(Seed {
                start: seed.start,
                range: seed.range,
            });
            end = seed.start + seed.range - 1;
            continue;
        }
        output.push(Seed {
            start: end + 1,
            range: seed.start + seed.range - end,
        });
        end = seed.start + seed.range - 1;
    }
    output
}

fn main() {
    let input = include_str!("../input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    let mut inputs = input.trim().split("\n\n");
    let mut seeds = Vec::new();
    let mut raw_seeds = inputs
        .next()
        .unwrap()
        .trim()
        .split_once(": ")
        .unwrap()
        .1
        .trim()
        .split(' ');
    while let Some(start_seed) = raw_seeds.next() {
        let start: i64 = start_seed.parse().unwrap();
        let range: i64 = raw_seeds.next().unwrap().parse().unwrap();
        seeds.push(Seed { start, range });
    }
    seeds.sort_by_key(|x| x.start);
    seeds = prune_seeds(&seeds);

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
            let mut min = i64::MAX;
            for i in x.start..(x.start + x.range - 1) {
                let mut output = i;
                for maps in transformers.iter() {
                    for map in maps {
                        let result = map.transform(output);
                        if result != output {
                            output = result;
                            break;
                        }
                    }
                }
                if output < min {
                    min = output;
                }
            }
            println!(
                "Finished Seed starting at {} with range {} and minimum value {}",
                x.start, x.range, min
            );
            min
        })
        .min()
        .unwrap();
    min.to_string()
}

#[cfg(test)]
mod tests_2 {
    use super::*;

    #[test]
    fn part_1() {
        let input = include_str!("../test1.txt");
        assert_eq!("46", part2(input));
    }
}

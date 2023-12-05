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

    fn range(&self) -> i64 {
        self.end - self.start + 1
    }

    fn transform_seed(&self, seed: &Seed) -> (Vec<Seed>, Vec<Seed>) {
        if seed.start > self.end || seed.end() < self.start {
            return (vec![], vec![*seed]);
        }
        if seed.start >= self.start && seed.end() <= self.end {
            return (
                vec![Seed {
                    start: seed.start + self.offset,
                    range: seed.range,
                }],
                vec![],
            );
        }
        if seed.start < self.start && seed.end() > self.end {
            return (
                vec![Seed {
                    start: self.start + self.offset,
                    range: self.range(),
                }],
                vec![
                    Seed {
                        start: seed.start,
                        range: self.start - seed.start,
                    },
                    Seed {
                        start: self.end + 1,
                        range: seed.end() - self.end,
                    },
                ],
            );
        }
        if seed.start >= self.start && seed.start <= self.end {
            return (
                vec![Seed {
                    start: seed.start + self.offset,
                    range: self.end - seed.start + 1,
                }],
                vec![Seed {
                    start: self.end + 1,
                    range: seed.end() - self.end,
                }],
            );
        }
        // if seed.end() >= self.start && seed.end() <= self.end {
        (
            vec![Seed {
                start: self.start + self.offset,
                range: seed.end() - self.start + 1,
            }],
            vec![Seed {
                start: seed.start,
                range: self.start - seed.start,
            }],
        )
    }
}

#[derive(Copy, Clone, Debug)]
struct Seed {
    start: i64,
    range: i64,
}

impl Seed {
    fn end(&self) -> i64 {
        self.start + self.range - 1
    }
}

fn prune_seeds(seeds: &[Seed]) -> Vec<Seed> {
    let mut output: Vec<Seed> = Vec::new();
    let mut end = -1;
    for seed in seeds {
        if seed.start + seed.range - 1 <= end {
            continue;
        }
        if seed.start > end {
            output.push(*seed);
            end = seed.end();
            continue;
        }
        output.push(Seed {
            start: end + 1,
            range: seed.end() - end + 1,
        });
        end = seed.end();
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
    for step in transformers {
        let mut output: Vec<Seed> = Vec::new();
        let mut untransformed: Vec<Seed> = seeds.to_vec();
        for map in step.iter() {
            let (transformed, raw): (Vec<Vec<Seed>>, Vec<Vec<Seed>>) =
                untransformed.iter().map(|x| map.transform_seed(x)).unzip();
            let flat_raw: Vec<Seed> = raw.iter().flatten().copied().collect();
            let mut flat_transformed: Vec<Seed> = transformed.iter().flatten().copied().collect();
            untransformed = flat_raw;
            output.append(&mut flat_transformed);
        }
        output.append(&mut untransformed);
        seeds = output;
    }
    seeds.sort_by_key(|x| x.start);
    dbg!(&seeds);
    seeds.first().unwrap().start.to_string()
}

#[cfg(test)]
mod tests_2_fast {
    use super::*;

    #[test]
    fn part_2() {
        let input = include_str!("../test1.txt");
        assert_eq!("46", part2(input));
    }
}

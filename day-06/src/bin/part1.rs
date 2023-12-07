struct Race {
    time: u32,
    distance: u32,
}

impl Race {
    fn winning_scenarios(&self) -> u32 {
        let options: Vec<u32> = (1..self.time).collect();
        let winning_options: Vec<u32> = options
            .iter()
            .map(|x| x * (self.time - x))
            .filter(|x| x > &self.distance)
            .collect();
        winning_options.len() as u32
    }
}

fn main() {
    let input = include_str!("../input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let mut lines = input.trim().lines();
    let raw_times = lines.next().unwrap();
    let raw_distances = lines.next().unwrap();
    let times: Vec<u32> = raw_times
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let distances: Vec<u32> = raw_distances
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let races: Vec<Race> = times
        .iter()
        .zip(distances)
        .map(|(time, distance)| Race {
            time: *time,
            distance,
        })
        .collect();
    races
        .iter()
        .map(|x| x.winning_scenarios())
        .product::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = include_str!("../test1.txt");
        assert_eq!("288", part1(input));
    }
}

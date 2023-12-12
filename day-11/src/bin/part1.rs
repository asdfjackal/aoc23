struct Galaxy {
    x: i32,
    y: i32,
}
fn main() {
    let input = include_str!("../input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn parse_universe(input: &str) -> Vec<Vec<char>> {
    input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn transpose_universe(input: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let len = input[0].len();
    (0..len)
        .map(|i| input.iter().map(|row| row[i]).collect())
        .collect()
}

fn expand_y_axis(input: Vec<Vec<char>>) -> Vec<Vec<char>> {
    input
        .iter()
        .flat_map(|line| {
            if line.iter().all(|x| x == &'.') {
                return vec![line, line];
            }
            vec![line]
        })
        .cloned()
        .collect()
}

fn expand_universe(input: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let intermediate = expand_y_axis(input);
    let transposed = transpose_universe(intermediate);
    expand_y_axis(transposed)
}

fn get_galaxies(input: Vec<Vec<char>>) -> Vec<Galaxy> {
    let mut output = Vec::new();
    for (y, row) in input.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if cell == &'#' {
                output.push(Galaxy {
                    x: x as i32,
                    y: y as i32,
                });
            }
        }
    }
    output
}

fn get_total_distance(galaxies: Vec<Galaxy>) -> i32 {
    let mut total = 0;
    galaxies.iter().enumerate().for_each(|(i, first)| {
        let others = galaxies.iter().skip(i + 1);
        others.for_each(|other| total += get_distance(first, other));
    });
    total
}

fn get_distance(first: &Galaxy, second: &Galaxy) -> i32 {
    (first.x - second.x).abs() + (first.y - second.y).abs()
}

fn part1(input: &str) -> String {
    let universe = parse_universe(input);
    let expanded = expand_universe(universe);
    let galaxies = get_galaxies(expanded);
    let total = get_total_distance(galaxies);

    total.to_string()
}

#[cfg(test)]
mod tests_1 {
    use super::*;

    #[test]
    fn part_1() {
        let input = include_str!("../test1.txt");
        assert_eq!("374", part1(input));
    }
}

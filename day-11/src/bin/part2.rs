struct Galaxy {
    x: i64,
    y: i64,
}
fn main() {
    let input = include_str!("../input1.txt");
    let output = part2(input, 1_000_000);
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

fn get_y_voids(input: &[Vec<char>]) -> Vec<i64> {
    let mut voids = Vec::new();
    input.iter().enumerate().for_each(|(i, line)| {
        if line.iter().all(|x| x == &'.') {
            voids.push(i as i64);
        }
    });
    voids
}

fn get_voids(input: Vec<Vec<char>>) -> (Vec<i64>, Vec<i64>) {
    let y_voids = get_y_voids(&input);
    let transposed = transpose_universe(input);
    let x_voids = get_y_voids(&transposed);
    (x_voids, y_voids)
}

fn get_galaxies(input: &[Vec<char>]) -> Vec<Galaxy> {
    let mut output = Vec::new();
    for (y, row) in input.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if cell == &'#' {
                output.push(Galaxy {
                    x: x as i64,
                    y: y as i64,
                });
            }
        }
    }
    output
}

fn get_total_distance(galaxies: Vec<Galaxy>) -> i64 {
    let mut total = 0;
    galaxies.iter().enumerate().for_each(|(i, first)| {
        let others = galaxies.iter().skip(i + 1);
        others.for_each(|other| total += get_distance(first, other));
    });
    total
}

fn get_distance(first: &Galaxy, second: &Galaxy) -> i64 {
    (first.x - second.x).abs() + (first.y - second.y).abs()
}

fn expand_galaxies(
    galaxies: Vec<Galaxy>,
    x_voids: Vec<i64>,
    y_voids: Vec<i64>,
    expansion: i64,
) -> Vec<Galaxy> {
    galaxies
        .iter()
        .map(|galaxy| {
            let x_void_count = x_voids.iter().copied().filter(|x| x < &galaxy.x).count() as i64;
            let y_void_count = y_voids.iter().copied().filter(|y| y < &galaxy.y).count() as i64;
            Galaxy {
                x: (galaxy.x + (x_void_count * (expansion - 1))),
                y: (galaxy.y + (y_void_count * (expansion - 1))),
            }
        })
        .collect()
}

fn part2(input: &str, expansion: i64) -> String {
    let universe = parse_universe(input);
    let galaxies = get_galaxies(&universe);
    let (x_voids, y_voids) = get_voids(universe);
    let expanded = expand_galaxies(galaxies, x_voids, y_voids, expansion);
    let total = get_total_distance(expanded);

    total.to_string()
}

#[cfg(test)]
mod tests_2 {
    use super::*;

    #[test]
    fn part_2() {
        let input = include_str!("../test1.txt");
        assert_eq!("8410", part2(input, 100));
    }
}

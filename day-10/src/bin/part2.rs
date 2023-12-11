#[derive(Copy, Clone)]
struct Point {
    x: usize,
    y: usize,
}

enum Direction {
    North,
    South,
    East,
    West,
}

fn main() {
    let input = include_str!("../input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn determine_starting_char(board: &[Vec<char>], start: &Point) -> char {
    let (mut north, mut south, mut east, mut west) = (false, false, false, false);
    if start.x != 0 {
        west = matches!(
            board.get(start.y).unwrap().get(start.x - 1).unwrap(),
            'F' | 'L' | '-'
        )
    }
    if start.x != board.get(0).unwrap().len() - 1 {
        east = matches!(
            board.get(start.y).unwrap().get(start.x + 1).unwrap(),
            '7' | 'J' | '-'
        )
    }
    if start.y != 0 {
        north = matches!(
            board.get(start.y - 1).unwrap().get(start.x).unwrap(),
            '7' | 'F' | '|'
        )
    }
    if start.y != board.len() - 1 {
        south = matches!(
            board.get(start.y + 1).unwrap().get(start.x).unwrap(),
            'J' | 'L' | '|'
        )
    }

    match (north, south, east, west) {
        (true, true, false, false) => '|',
        (false, false, false, false) => '-',
        (true, false, true, false) => 'L',
        (true, false, false, true) => 'J',
        (false, true, true, false) => 'F',
        (false, true, false, true) => '7',
        _ => panic!(),
    }
}

fn get_exit(pipe: &char, dir_in: &Direction) -> Direction {
    use Direction::*;

    match (pipe, dir_in) {
        ('|', South) => South,
        ('|', North) => North,
        ('-', East) => East,
        ('-', West) => West,
        ('L', South) => East,
        ('L', West) => North,
        ('J', South) => West,
        ('J', East) => North,
        ('F', West) => South,
        ('F', North) => East,
        ('7', East) => South,
        ('7', North) => West,
        _ => panic!(),
    }
}

fn get_at_point(board: &[Vec<char>], point: &Point) -> char {
    *board.get(point.y).unwrap().get(point.x).unwrap()
}

fn get_points_inside(row: &[usize], width: usize, y: usize, board: &[Vec<char>]) -> u32 {
    let mut total_inside = 0;
    let mut inside = false;
    let mut last_turn = ' ';
    for i in 0..width {
        if row.contains(&i) {
            let point = Point { x: i, y };
            let mut path_char = get_at_point(board, &point);
            if path_char == 'S' {
                path_char = determine_starting_char(board, &point)
            }
            if path_char == 'F' || path_char == 'L' {
                last_turn = path_char;
            }
            match path_char {
                '|' => inside = !inside,
                '7' => {
                    if last_turn == 'L' {
                        inside = !inside
                    }
                }
                'J' => {
                    if last_turn == 'F' {
                        inside = !inside
                    }
                }
                _ => (),
            }
            continue;
        }
        if inside {
            total_inside += 1;
        }
    }
    total_inside
}

fn part2(input: &str) -> String {
    use Direction::*;
    let mut board: Vec<Vec<char>> = Vec::new();
    input
        .trim()
        .lines()
        .for_each(|line| board.push(line.chars().collect()));
    let mut start = Point { x: 0, y: 0 };
    'outer: for (y, line) in board.iter().enumerate() {
        for (x, char) in line.iter().enumerate() {
            if *char == 'S' {
                start.x = x;
                start.y = y;
                break 'outer;
            }
        }
    }
    let mut path = vec![start];
    let mut current_char = determine_starting_char(&board, &start);
    let mut current_point = start;
    let mut dir_in = match current_char {
        '|' | '7' | 'F' => North,
        '-' | 'J' => East,
        _ => West,
    };
    loop {
        dir_in = get_exit(&current_char, &dir_in);
        match dir_in {
            North => {
                current_point.y -= 1;
            }
            South => {
                current_point.y += 1;
            }
            East => {
                current_point.x += 1;
            }
            West => {
                current_point.x -= 1;
            }
        }
        current_char = get_at_point(board.as_slice(), &current_point);
        path.push(current_point);
        if current_char == 'S' {
            break;
        }
    }

    let height = board.len();
    let width = board.get(0).unwrap().len();

    let mut insides = Vec::new();

    for j in 0..height {
        let mut path_intersections: Vec<usize> =
            path.iter().filter(|p| p.y == j).map(|x| x.x).collect();
        path_intersections.sort();
        let points_inside = get_points_inside(path_intersections.as_slice(), width, j, &board);
        insides.push(points_inside);
    }

    dbg!(&insides);

    insides.iter().sum::<u32>().to_string()
}

#[cfg(test)]
mod tests_2 {
    use super::*;

    #[test]
    fn test_input_3() {
        let input = include_str!("../test3.txt");
        assert_eq!("4", part2(input));
    }

    #[test]
    fn test_input_4() {
        let input = include_str!("../test4.txt");
        assert_eq!("4", part2(input));
    }

    #[test]
    fn test_input_5() {
        let input = include_str!("../test5.txt");
        assert_eq!("8", part2(input));
    }

    #[test]
    fn test_input_6() {
        let input = include_str!("../test6.txt");
        assert_eq!("10", part2(input));
    }
}

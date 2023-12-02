use std::collections::HashMap;

fn get_value_or_zero(map: &HashMap<&str, i32>, key: &str) -> i32 {
    match map.get(key) {
        None => 0,
        Some(i) => *i,
    }
}

#[derive(PartialEq, Debug, Clone)]
struct Turn {
    red: i32,
    blue: i32,
    green: i32,
}

impl Turn {
    fn from_string(input: &str) -> Self {
        let mut raw_turn = HashMap::new();
        input.trim().split(',').for_each(|x| {
            let raw_color = x.trim().split_once(' ').unwrap();
            let count: i32 = raw_color.0.parse().unwrap();
            let color = raw_color.1;
            raw_turn.insert(color, count);
        });
        Turn {
            red: get_value_or_zero(&raw_turn, "red"),
            blue: get_value_or_zero(&raw_turn, "blue"),
            green: get_value_or_zero(&raw_turn, "green"),
        }
    }

    fn power(&self) -> i32 {
        self.red * self.green * self.blue
    }
}

#[derive(PartialEq, Debug)]
struct Game {
    id: i32,
    turns: Vec<Turn>,
}

impl Game {
    fn from_string(input: &str) -> Self {
        let raw_game = input.trim().split_once(':').unwrap();
        let game_info = raw_game.0.trim().split_once(' ').unwrap();
        let raw_turns = raw_game.1.trim();
        Game {
            id: game_info.1.parse().unwrap(),
            turns: raw_turns.split(';').map(Turn::from_string).collect(),
        }
    }

    fn minimum(&self) -> Turn {
        let mut min = Turn {
            red: 0,
            blue: 0,
            green: 0,
        };
        for turn in self.turns.iter() {
            if min.red < turn.red {
                min.red = turn.red
            };
            if min.green < turn.green {
                min.green = turn.green
            };
            if min.blue < turn.blue {
                min.blue = turn.blue
            };
        }
        min
    }
}

fn main() {
    let input = include_str!("../input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let games: Vec<Game> = input.trim().split('\n').map(Game::from_string).collect();
    let sum: i32 = games.iter().map(|x| x.minimum().power()).sum();
    sum.to_string()
}

#[cfg(test)]
mod tests_2 {
    use super::*;
    use rstest::rstest;

    #[test]
    fn part_2() {
        let input = include_str!("../test1.txt");
        assert_eq!("2286", part1(input));
    }

    #[rstest]
    #[case("3 blue, 4 red", Turn{blue: 3, red: 4, green: 0})]
    #[case(" 3 green, 4 blue, 1 red", Turn{blue: 4, red: 1, green: 3})]
    fn turn_from_string(#[case] input: &str, #[case] output: Turn) {
        assert_eq!(output, Turn::from_string(input))
    }

    #[rstest]
    #[case(
        "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
        Game {
            id: 2,
            turns: Vec::from([
                Turn {
                    red: 0,
                    blue: 1,
                    green: 2
                },
                Turn {
                    red: 1,
                    blue: 4,
                    green: 3
                },
                Turn {
                    red: 0,
                    blue: 1,
                    green: 1
                }
            ])
        }
    )]
    fn game_from_string(#[case] input: &str, #[case] output: Game) {
        assert_eq!(output, Game::from_string(input,))
    }
}

use std::path::PathBuf;
use crate::read_lines;

#[derive(Debug)]
struct Game {
    number: i32,
    sets: Vec<Set>
}

impl PartialEq for Game {
    fn eq(&self, other: &Self) -> bool {
        self.number == other.number &&
        self.sets == other.sets
    }
}

#[derive(Debug)]
struct Set {
    red: i32,
    green: i32,
    blue: i32
}

impl PartialEq for Set {
    fn eq(&self, other: &Self) -> bool {
        self.red == other.red &&
        self.green == other.green &&
        self.blue == other.blue
    }
}

fn day2(part2: bool, path: &PathBuf) -> i32 {
    let red_limit = 12;
    let green_limit = 13;
    let blue_limit = 14;

    let mut sum: i32 = 0;

    // File input.txt must exist in the current path
    if let Ok(lines) = read_lines(path) {
        let games: Vec<Game> = lines.map(|line| {
            let unwrapped_line = line.as_ref().unwrap();
            println!("{}", unwrapped_line);
            let game: Game = parse_game(unwrapped_line);
            let sets: &Vec<Set> = &game.sets;
            let mut valid: bool = true;
            &sets.iter().for_each(|set| {
                if set.red > red_limit || set.green > green_limit || set.blue > blue_limit {
                    valid = false;
                }
            });
            if valid {
                sum = sum + &game.number;
            }
            return game;
        }).collect::<Vec<Game>>();
    }
    return sum;
}

fn parse_game(line: &str) -> Game {
    //Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    let game_str: Vec<&str> = line.split(":").collect();
    let number = game_str.get(0).unwrap().split(" ").last().unwrap().parse().unwrap();
    let sets_string: Vec<&str> = game_str.get(1).unwrap().split(";").collect();
    let sets: Vec<Set> = sets_string.iter().map(|str| {
        let mut red_number: i32 = 0;
        let mut green_number: i32 = 0;
        let mut blue_number: i32 = 0;
        let cubes: Vec<&str> = str.split(",").collect();
        let _: Vec<i32> = cubes.iter().map(|cube_string| {
            let cube: Vec<&str> = cube_string.split(" ").collect();
            if cube.get(2).unwrap().contains("red") {
                red_number = cube.get(1).unwrap().parse().unwrap();
            } else if cube.get(2).unwrap().contains("green") {
                green_number = cube.get(1).unwrap().parse().unwrap();
            } else if cube.get(2).unwrap().contains("blue") {
                blue_number = cube.get(1).unwrap().parse().unwrap();
            }
            return 1;
        }).collect();
        return Set {
            red: red_number,
            green: green_number,
            blue: blue_number,
        }
    }).collect::<Vec<Set>>();
    return Game {
        number,
        sets,
    };
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::day2::{day2, Game, parse_game, Set};

    #[test]
    fn test_parse_line() {
        let set1: Set = Set {
            red: 1,
            green: 2,
            blue: 3,
        };
        let set2: Set = Set {
            red: 0,
            green: 1,
            blue: 0,
        };
        let expected_game: Game = Game {
            number: 1,
            sets: vec![set1, set2],
        };

        let parsed_game: Game = parse_game("Game 1: 3 blue, 1 red, 2 green; 1 green");
        assert_eq!(parsed_game, expected_game);
    }

    #[test]
    fn test_part1() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/day2/test/input.txt");
        assert_eq!(day2(false, &d), 8);
    }

    #[test]
    fn part1() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/day2/input.txt");
        assert_eq!(day2(false, &d), 2278);
    }

    #[test]
    fn test_part2() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/day2/test/input2.txt");
        assert_eq!(day2(true, &d), 281);
    }

}
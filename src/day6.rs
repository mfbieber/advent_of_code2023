use std::path::PathBuf;
use crate::read_lines;

struct Race {
    time: i64,
    win_distance: i64,
    speed: i64
}

impl Race {
    fn hold_one_ms(&mut self) {
        self.speed = self.speed + 1;
        self.time = self.time - 1;
    }

    fn get_distance(&self) -> i64 {
        return self.time * self.speed;
    }

    fn would_win(&self) -> bool {
        return self.get_distance() > self.win_distance;
    }
}

fn day6_part1(races: &mut Vec<Race>) -> i64 {
    let mut multiplied: i64 = 1;
    for race in races {
        let mut i: i64 = 1;
        let mut wins: i64 = 0;
        while i < race.time {
            race.hold_one_ms();
            if race.would_win() {
                wins += 1;
            }
        }
        multiplied = multiplied * wins;
    }
    println!("{}", multiplied);
    return multiplied;
}

fn day6_part2(race: &mut Race) -> i64 {
    let mut i: i64 = 1;
    let mut wins: i64 = 0;
    while i < race.time {
        race.hold_one_ms();
        if race.would_win() {
            wins += 1;
        }
    }
    println!("{}", wins);
    return wins;
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::day6::{day6_part1, day6_part2, Race};

    #[test]
    fn test_would_not_win() {
        let mut race: Race = Race {
            time: 6,
            win_distance: 9,
            speed: 1,
        };
        assert_eq!(race.would_win(), false);
        assert_eq!(race.get_distance(), 6);
    }

    #[test]
    fn test_would_win() {
        let mut race: Race = Race {
            time: 7,
            win_distance: 9,
            speed: 0,
        };
        race.hold_one_ms();
        race.hold_one_ms();
        race.hold_one_ms();
        race.hold_one_ms();
        assert_eq!(race.would_win(), true);
    }

    #[test]
    fn test_hold_one_ms() {
        let mut race: Race = Race {
            time: 7,
            win_distance: 9,
            speed: 0,
        };
        race.hold_one_ms();
        race.hold_one_ms();
        assert_eq!(race.speed, 2);
        assert_eq!(race.time, 5);
    }

    #[test]
    fn test_part1() {
        let mut race1: Race = Race {
            time: 7,
            win_distance: 9,
            speed: 0,
        };
        let mut race2: Race = Race {
            time: 15,
            win_distance: 40,
            speed: 0,
        };
        let mut race3: Race = Race {
            time: 30,
            win_distance: 200,
            speed: 0,
        };
        let mut races: Vec<Race> = vec![race1, race2, race3];
        assert_eq!(day6_part1(&mut races), 288);
    }

    #[test]
    fn part1() {
        let mut race1: Race = Race {
            time: 41,
            win_distance: 214,
            speed: 0,
        };
        let mut race2: Race = Race {
            time: 96,
            win_distance: 1789,
            speed: 0,
        };
        let mut race3: Race = Race {
            time: 88,
            win_distance: 1127,
            speed: 0,
        };
        let mut race4: Race = Race {
            time: 94,
            win_distance: 1055,
            speed: 0,
        };
        let mut races: Vec<Race> = vec![race1, race2, race3, race4];
        assert_eq!(day6_part1(&mut races), 4811940);
    }

    #[test]
    fn test_part2() {
        let mut race: Race = Race {
            time: 71530,
            win_distance: 940200,
            speed: 0,
        };
        assert_eq!(day6_part2(&mut race), 71503);
    }

    #[test]
    fn part2() {
        let mut race: Race = Race {
            time: 41968894,
            win_distance: 214178911271055,
            speed: 0,
        };
        assert_eq!(day6_part2(&mut race), 30077773);
    }

}
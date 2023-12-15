use std::path::PathBuf;
use crate::read_lines;

struct History {
    values: Vec<i32>,
    differences: Vec<Vec<i32>>
}

impl History {
    fn new(string: String) -> History {
        let values: Vec<i32> = string.split(" ")
            .map(|n| {
                let i: i32 = n.parse().unwrap();
                return i;
            }).collect();
        return History { values, differences: vec![] };
    }

    fn calculate_differences(&mut self) {
        let mut all_differences: Vec<Vec<i32>> = vec![];
        let mut differences: Vec<i32> = self.values.clone();
        let mut end: bool = false;
        while !end {
            let mut previous: i32 = *differences.first().unwrap();
            let mut first: bool = true;
            differences = differences.iter().map(|x| {
                if !first {
                    let diff: i32 = x - previous;
                    previous = *x;
                    return diff;
                }
                first = false;
                return 0;
            }).collect();
            differences.remove(0);
            end = differences.iter().all(|x| *x == 0);
            all_differences.push(differences.clone());
        }
        self.differences = all_differences;
    }

    fn next_value(&self) -> i32 {
        let mut previous: i32 = *self.differences.last().unwrap().last().unwrap();
        let mut first: bool = true;
        self.differences.iter().rev().for_each(|differences| {
            if !first {
                previous = previous + differences.last().unwrap();
            }
            first = false;
        });
        return previous + self.values.last().unwrap();
    }

}

fn day9_part2(path: &PathBuf) -> i32 {
    let mut sum: i32 = 0;
    // File input.txt must exist in the current path
    if let Ok(lines) = read_lines(path) {
        for line in lines {

        }
    }
    println!("{}", sum);
    return sum;
}

fn day9_part1(path: &PathBuf) -> i32 {
    let mut sum: i32 = 0;
    // File input.txt must exist in the current path
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            let mut history: History = History::new(line.unwrap());
            history.calculate_differences();
            sum = sum + history.next_value();
        }
    }
    println!("{}", sum);
    return sum;
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::day9::{day9_part1, History};

    #[test]
    fn test_calculates_next_history_value_correctly_3() {
        let history_string = String::from("10 13 16 21 30 45");
        let mut history: History = History::new(history_string);
        history.calculate_differences();
        let next_history_value: i32 = history.next_value();
        assert_eq!(next_history_value, 68);
    }

    #[test]
    fn test_calculates_next_history_value_correctly_2() {
        let history_string = String::from("1 3 6 10 15 21");
        let mut history: History = History::new(history_string);
        history.calculate_differences();
        let next_history_value: i32 = history.next_value();
        assert_eq!(next_history_value, 28);
    }

    #[test]
    fn test_calculates_next_history_value_correctly() {
        let history_string = String::from("0 3 6 9 12 15");
        let mut history: History = History::new(history_string);
        history.calculate_differences();
        let next_history_value: i32 = history.next_value();
        assert_eq!(next_history_value, 18);
    }

    #[test]
    fn test_calculates_differences_correctly() {
        let history_string = String::from("0 3 6 9 12 15");
        let mut history: History = History::new(history_string);
        history.calculate_differences();
        let expected_differences: Vec<Vec<i32>> = vec![vec![3,3,3,3,3], vec![0,0,0,0]];
        assert_eq!(history.differences, expected_differences);
    }

    #[test]
    fn test_generates_history_correctly() {
        let history_string = String::from("0 3 6 9 12 15");
        let history: History = History::new(history_string);
        let expected_values: Vec<i32> = vec![0,3,6,9,12,15];
        assert_eq!(history.values, expected_values);
    }

    #[test]
    fn test_part1() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/day9/test/input.txt");
       assert_eq!(day9_part1(&d), 114);
    }

    #[test]
    fn part1() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/day9/input.txt");
        assert_eq!(day9_part1(&d), 1901217887);
    }

    #[test]
    fn test_part2() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/day9/test/input1.txt");
        //assert_eq!(day9_part2(&d), 281);
    }

    #[test]
    fn part2() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/day9/input.txt");
        //assert_eq!(day9_part2(&d), 281);
    }

}
use std::collections::{HashSet};
use std::path::PathBuf;
use futures::AsyncReadExt;
use crate::read_lines;

fn day4_part2(path: &PathBuf) -> i32 {
    let mut sum: i32 = 0;
    let mut instances: Vec<i32> = vec![0; 250];
    // File input.txt must exist in the current path
    if let Ok(lines) = read_lines(path) {
       let mut line_number: i32 = 0;
        for line in lines {
            let matching_numbers: Vec<i32> = match_numbers(&mut sum, line.as_ref().unwrap());
            let count: usize = matching_numbers.iter().count();
            let mut i: i32 = line_number;
            let cards: i32 = instances[i as usize] + 1;
            instances[i as usize] = instances[i as usize] + 1;
            i += 1;
            while i <= line_number + count as i32 {
                instances[i as usize] = instances[i as usize] + cards;
                i += 1;
            }
            line_number += 1;
        }
    }
    sum = instances.iter().sum();
    println!("{}", sum);
    return sum;
}

fn day4_part1(path: &PathBuf) -> i32 {
    let mut sum: i32 = 0;
    // File input.txt must exist in the current path
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            let matching_numbers: Vec<i32> = match_numbers(&mut sum, line.as_ref().unwrap());
            let count: usize = matching_numbers.iter().count();
            if count != 0 {
                sum = sum + 2_i32.pow((count - 1) as u32);
            }
        }
    }
    println!("{}", sum);
    return sum;
}

fn match_numbers(sum: &mut i32, line: &String) -> Vec<i32> {
    let mut sum: i32 = 0;
    let split_1: Vec<&str> = line.split(":").collect();
    let split_2: Vec<&str> = split_1.get(1).unwrap().split("|").collect();
    let winning_numbers: HashSet<i32> = split_2.get(0).unwrap()
        .split(" ")
        .filter(|char| !char.eq(&" ") && !char.eq(&""))
        .map(|char| {
            let number: i32 = char.parse().unwrap();
            return number;
        }).collect();
    let numbers_you_have: HashSet<i32> = split_2.get(1).unwrap()
        .split(" ")
        .filter(|char| !char.eq(&" ") && !char.eq(&""))
        .map(|char| {
            let number: i32 = char.parse().unwrap();
            return number;
        }).collect();

    return winning_numbers.intersection(&numbers_you_have)
        .map(|i| *i).collect();
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::day4::{day4_part1, day4_part2};

    #[test]
    fn test_part1() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/day4/test/input.txt");
       assert_eq!(day4_part1(&d), 13);
    }

    #[test]
    fn part1() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/day4/input.txt");
        assert_eq!(day4_part1(&d), 20107);
    }

    #[test]
    fn test_part2() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/day4/test/input.txt");
        assert_eq!(day4_part2(&d), 30);
    }

    #[test]
    fn part2() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/day4/input.txt");
        assert_eq!(day4_part2(&d), 8172507);
    }

}
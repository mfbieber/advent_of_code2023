use std::collections::HashSet;
use std::path::PathBuf;
use futures::AsyncReadExt;
use crate::read_lines;

fn day4_part2(path: &PathBuf) -> i32 {
    let mut sum: i32 = 0;
    // File input.txt must exist in the current path
    if let Ok(lines) = read_lines(path) {
        for line in lines {

        }
    }
    println!("{}", sum);
    return sum;
}

fn day4_part1(path: &PathBuf) -> i32 {
    let mut sum: i32 = 0;
    // File input.txt must exist in the current path
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            let split_1: Vec<&str> = line.as_ref().unwrap().split(":").collect();
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
            let matching_numbers: usize = winning_numbers.intersection(&numbers_you_have)
                .map(|i| *i).count();
            if matching_numbers != 0 {
                sum = sum + 2_i32.pow((matching_numbers - 1) as u32);
            }
        }
    }
    println!("{}", sum);
    return sum;
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::day4::day4_part1;

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
        d.push("resources/day4/test/input1.txt");
        //assert_eq!(day4_part2(&d), 281);
    }

    #[test]
    fn part2() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/day4/input.txt");
        //assert_eq!(day4_part2(&d), 281);
    }

}
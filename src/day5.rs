use std::path::PathBuf;
use crate::read_lines;

fn day5_part2(path: &PathBuf) -> u32 {
    let mut sum: u32 = 0;
    // File input.txt must exist in the current path
    if let Ok(lines) = read_lines(path) {
        for line in lines {

        }
    }
    println!("{}", sum);
    return sum;
}

fn day5_part1(path: &PathBuf) -> u32 {
    let mut sum: u32 = 0;
    // File input.txt must exist in the current path
    if let Ok(lines) = read_lines(path) {
        for line in lines {

        }
    }
    println!("{}", sum);
    return sum;
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    #[test]
    fn test_part1() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/day5/test/input.txt");
       //assert_eq!(day5_part1(&d), 142);
    }

    #[test]
    fn part1() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/day5/input.txt");
        //assert_eq!(day5_part1(&d), 142);
    }

    #[test]
    fn test_part2() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/day5/test/input1.txt");
        //assert_eq!(day5_part2(&d), 281);
    }

    #[test]
    fn part2() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/day5/input.txt");
        //assert_eq!(day5_part2(&d), 281);
    }

}
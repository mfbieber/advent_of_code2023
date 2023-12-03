use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};

fn main() {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("resources/day1/test/input.txt");
    day1(false, d);
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("resources/day1/test/input2.txt");
    day1(true, d);
}

fn day1(part2: bool, path: PathBuf) -> u32 {
    let mut sum: u32 = 0;
    // File input.txt must exist in the current path
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(mut input) = line {
                if part2 {
                    convert_words_to_numbers(&mut input);
                }
                println!("{}", &mut input);
                let numbers: Vec<char> = input.chars()
                    .filter(|char| char.is_numeric())
                    .collect();
                let line_numbers: String = format!("{}{}", numbers.first().unwrap(), numbers.last().unwrap());
                println!("{}", line_numbers);
                sum = sum + line_numbers.parse::<u32>().unwrap();
            }
        }
    }
    println!("{}", sum);
    return sum;
}

fn convert_words_to_numbers(mut input: &mut String) {
    let mut i = 0;
    while i < input.len() {
        println!("{}", &mut input);
        println!("{}", i);
        if !input[i..i + 1].chars().last().expect("").is_numeric() {
            replace_words(&mut input, i);
        }
        i += 1;
    }
}

fn replace_words(string: &mut String, i: usize) -> bool{
    if string[i..].starts_with("one") {
        string.replace_range(i..i+3, "1");
        return true;
    } else if string[i..].starts_with("two") {
        string.replace_range(i..i+3, "2");
        return true;
    } else if string[i..].starts_with("three") {
        string.replace_range(i..i+5, "3");
        return true;
    } else if string[i..].starts_with("four") {
        string.replace_range(i..i+4, "4");
        return true;
    } else if string[i..].starts_with("five") {
        string.replace_range(i..i+4, "5");
        return true;
    } else if string[i..].starts_with("six") {
        string.replace_range(i..i+3, "6");
        return true;
    } else if string[i..].starts_with("seven") {
        string.replace_range(i..i+5, "7");
        return true;
    } else if string[i..].starts_with("eight") {
        string.replace_range(i..i+5, "8");
        return true;
    } else if string[i..].starts_with("nine") {
        string.replace_range(i..i+4, "9");
        return true;
    } else {
        return false;
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::day1;

    #[test]
    fn test_part1() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/day1/test/input.txt");
        assert_eq!(day1(false, d), 142);
    }

    #[test]
    fn test_part2() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/day1/test/input2.txt");
        assert_eq!(day1(true, d), 281);
    }

}
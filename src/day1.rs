use std::path::PathBuf;
use crate::read_lines;

fn day1(part2: bool, path: &PathBuf) -> u32 {
    let mut sum: u32 = 0;
    // File input.txt must exist in the current path
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            let input = line.unwrap();
            println!("{}", input);
            let line_numbers: String;
            if part2 {
                line_numbers = convert_words_to_numbers(input.clone());
            } else {
                let numbers: Vec<char> = input.chars()
                    .filter(|char| char.is_numeric())
                    .collect();
                line_numbers = format!("{}{}", numbers.first().unwrap(), numbers.last().unwrap());
            }
            println!("{}", line_numbers);
            sum = sum + line_numbers.parse::<u32>().unwrap();
        }
    }
    println!("{}", sum);
    return sum;
}

fn convert_words_to_numbers(input: String) -> String {
    let replace_first = find_first_number(input.clone());
    println!("{}", replace_first);
    let replace_last = find_last_number(input.clone());
    println!("{}", replace_last);
    let numbers_first: Vec<char> = replace_first.chars()
        .filter(|char| char.is_numeric())
        .collect();
    let numbers_last: Vec<char> = replace_last.chars()
        .filter(|char| char.is_numeric())
        .collect();
    let line_numbers: String = format!("{}{}", numbers_first.first().unwrap(), numbers_last.last().unwrap());
    println!("{}", line_numbers);
    return line_numbers;
}

fn find_last_number(input: String) -> String {
    let mut i = input.len() - 1;
    while i > 0 {
        //println!("{}", &mut input);
        //println!("{}", i);
        let lower_bound = i - 1;
        let upper_excluded_bound = i + 1;
        if !input[lower_bound..upper_excluded_bound].chars().last().expect("").is_numeric() {
            let (string, replaced) = replace_words(&input, i);
            if replaced {
                return string;
            }
        }
        i -= 1;
    }
    return input;
}

fn find_first_number(input: String) -> String {
    let mut i = 0;
    while i < input.len() {
        if !input[i..i + 1].chars().last().expect("").is_numeric() {
            let (string, replaced) = replace_words(&input, i);
            if replaced {
                return string;
            }
        }
        i += 1;
    }
    return input;
}

fn replace_word(string: &String, i: usize, word: String, number: i32) -> (String, bool) {
    let len = word.len();
    let mut copy = string.clone();
     if copy[i..].starts_with(&word) {
         copy.replace_range(i..i + len, &*number.to_string());
         return (copy.clone(), true);
    } else if i >= len && string[i - len + 1 ..i + 1].starts_with(&word) {
         copy.replace_range(i - len + 1 ..i + 1, &*number.to_string());
         return (copy.clone(), true);
    }
    return (copy.clone(), false);
}

fn replace_words(string: &String, i: usize) -> (String, bool) {
    let (string_copy, replaced) = replace_word(&string, i, "one".parse().unwrap(), 1);
    if replaced {
        return (string_copy.clone(), true);
    }
    let (string_copy, replaced) = replace_word(&string, i, "two".parse().unwrap(), 2);
    if replaced {
        return (string_copy.clone(), true);
    }
    let (string_copy, replaced) = replace_word(&string, i, "three".parse().unwrap(), 3);
    if replaced {
        return (string_copy.clone(), true);
    }
    let (string_copy, replaced) = replace_word(&string, i, "four".parse().unwrap(), 4);
    if replaced {
        return (string_copy.clone(), true);
    }
    let (string_copy, replaced) = replace_word(&string, i, "five".parse().unwrap(), 5);
    if replaced {
        return (string_copy.clone(), true);
    }
    let (string_copy, replaced) = replace_word(&string, i, "six".parse().unwrap(), 6);
    if replaced {
        return (string_copy.clone(), true);
    }
    let (string_copy, replaced) = replace_word(&string, i, "seven".parse().unwrap(), 7);
    if replaced {
        return (string_copy.clone(), true);
    }
    let (string_copy, replaced) = replace_word(&string, i, "eight".parse().unwrap(), 8);
    if replaced {
        return (string_copy.clone(), true);
    }
    let (string_copy, replaced) = replace_word(&string, i, "nine".parse().unwrap(), 9);
    if replaced {
        return (string_copy.clone(), true);
    }
    return (string_copy.clone(), false);
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::day1::day1;

    #[test]
    fn test_part1() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/day1/test/input.txt");
        assert_eq!(day1(false, &d), 142);
    }

    #[test]
    fn test_part2() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/day1/test/input2.txt");
        assert_eq!(day1(true, &d), 281);
    }

    #[test]
    fn part1() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/day1/input.txt");
        assert_eq!(day1(false, &d), 54081);
    }

    #[test]
    fn part2() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/day1/input.txt");
        assert_eq!(day1(true, &d), 54649);
    }

}
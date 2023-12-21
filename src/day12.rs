use std::path::PathBuf;
use crate::read_lines;

fn day12_part2(path: &PathBuf) -> i32 {
    let mut sum: i32 = 0;
    // File input.txt must exist in the current path
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            let (springs, damaged): (Vec<char>, Vec<i32>) = parse_line(line.unwrap().clone());
            let mut unfolded_springs: Vec<char> = vec![];
            unfolded_springs.append(&mut springs.clone());
            unfolded_springs.append(&mut springs.clone());
            unfolded_springs.append(&mut springs.clone());
            unfolded_springs.append(&mut springs.clone());
            unfolded_springs.append(&mut springs.clone());
            let mut unfolded_damaged: Vec<i32> = vec![];
            unfolded_damaged.append(&mut damaged.clone());
            unfolded_damaged.append(&mut damaged.clone());
            unfolded_damaged.append(&mut damaged.clone());
            unfolded_damaged.append(&mut damaged.clone());
            unfolded_damaged.append(&mut damaged.clone());
            let arrangements: i32 = find_different_arrangements((unfolded_springs.clone(), unfolded_damaged.clone()));
            sum = sum + arrangements;
        }
    }
    println!("{}", sum);
    return sum;
}

fn day12_part1(path: &PathBuf) -> i32 {
    let mut sum: i32 = 0;
    // File input.txt must exist in the current path
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            let (springs, damaged): (Vec<char>, Vec<i32>) = parse_line(line.unwrap().clone());
            let arrangements: i32 = find_different_arrangements((springs.clone(), damaged.clone()));
            sum = sum + arrangements;
        }
    }
    println!("{}", sum);
    return sum;
}

fn parse_line(line: String) -> (Vec<char>, Vec<i32>) {
    let split: Vec<&str> = line.split(" ").collect::<Vec<&str>>();
    let springs: Vec<char> = split[0].chars().collect::<Vec<char>>();
    let damaged: Vec<i32> = split[1].split(",").map(|a| {
        let x: i32 = a.parse().unwrap();
        return x;
    }).collect();
    return (springs, damaged);
}

fn find_different_arrangements((springs, damaged): (Vec<char>, Vec<i32>)) -> i32 {
    let mut i: usize = 0;
    let mut arrangements: Vec<Vec<char>> = vec![vec!['.']];
    while i < springs.len() {
        if springs[i] == '?' {
            let mut j: usize = 0;
            let k: usize = arrangements.len().clone();
            while j < k {
                let mut copy: Vec<char> = arrangements[j].clone();
                if i != 0 {
                    arrangements[j].push('.');
                    copy.push('#');
                } else {
                    arrangements[j][0] = '.';
                    copy[0] = '#';
                }
                let mut modified_list: bool = false;
                if validate_partially(&copy, &damaged) {
                    arrangements.push(copy);
                }
                if modified_list {
                    j = 0;
                    break;
                }
                j += 1;
            }
            j = 0;
            while j < arrangements.len() {
                if !validate_partially(&arrangements[j], &damaged) {
                    arrangements.remove(j);
                }
                j += 1;
            }
        } else {
            let mut j: usize = 0;
            while j < arrangements.len() {
                if i != 0 {
                    arrangements[j].push(springs[i]);
                }
                if !validate_partially(&arrangements[j], &damaged) {
                    arrangements.remove(j);
                    continue;
                }
                j += 1;
            }
        }
        i += 1;
    }
    i = 0;
    let k: usize = arrangements.len().clone();
    let expected_damaged: usize = damaged.iter().map(|d| *d as usize).sum();
    let mut valid_arrangements: i32 = 0;
    while i < k {
        arrangements[i].resize(springs.len(), 'x');
        if  validate_all(&arrangements[i], expected_damaged, &damaged) {
            valid_arrangements += 1;
        }
        i += 1;
    }
    return valid_arrangements;
}

fn validate_all(arrangement: &Vec<char>, expected_damaged: usize, damaged: &Vec<i32>) -> bool {
    let damaged_count: usize = arrangement.iter().filter(|char| **char == '#').count();
    if damaged_count == expected_damaged {
       return validate_partially(arrangement, damaged);
    }
    return false;
}

fn validate_partially(arrangement: &Vec<char>, damaged: &Vec<i32>) -> bool {
    let mut spring_idx: usize = 0;
    let mut damaged_idx: usize = 0;
    let mut valid: bool = true;
    while spring_idx < arrangement.len() {
        if arrangement[spring_idx] == '.' {
            spring_idx += 1;
        } else {
            let mut d: usize = 0;
            while damaged_idx < damaged.len() && d < damaged[damaged_idx] as usize && spring_idx < arrangement.len() {
                if arrangement[spring_idx] != '#' {
                    valid = false;
                    spring_idx += 1;
                    break;
                }
                d += 1;
                spring_idx += 1;
            }
            if damaged_idx < damaged.len() && d > 0
                && spring_idx < arrangement.len()
                && arrangement[spring_idx] != '.' {
                valid = false;
                spring_idx += 1;
                break;
            }
            if damaged_idx > damaged.len() {
                valid = false;
                break;
            }
            damaged_idx += 1;
        }
        if !valid {
            break;
        }
    }
    return valid;
}



#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::day12::{day12_part1, day12_part2, find_different_arrangements, parse_line};

    #[test]
    fn finds_arrangement_correctly6() {
        let (springs, damaged): (Vec<char>, Vec<i32>) = parse_line(String::from("?###???????? 3,2,1"));
        let arrangements: i32 = find_different_arrangements((springs.clone(), damaged.clone()));
        assert_eq!(arrangements, 10);
    }

    #[test]
    fn finds_arrangement_correctly5() {
        let (springs, damaged): (Vec<char>, Vec<i32>) = parse_line(String::from("????.######..#####. 1,6,5"));
        let arrangements: i32 = find_different_arrangements((springs.clone(), damaged.clone()));
        assert_eq!(arrangements, 4);
    }

    #[test]
    fn finds_arrangement_correctly4() {
        let (springs, damaged): (Vec<char>, Vec<i32>) = parse_line(String::from("????.#...#... 4,1,1"));
        let arrangements: i32 = find_different_arrangements((springs.clone(), damaged.clone()));
        assert_eq!(arrangements, 1);
    }

    #[test]
    fn finds_arrangement_correctly3() {
        let (springs, damaged): (Vec<char>, Vec<i32>) = parse_line(String::from("?#?#?#?#?#?#?#? 1,3,1,6"));
        let arrangements: i32 = find_different_arrangements((springs.clone(), damaged.clone()));
        assert_eq!(arrangements, 1);
    }

    #[test]
    fn finds_arrangement_correctly2() {
        let (springs, damaged): (Vec<char>, Vec<i32>) = parse_line(String::from(".??..??...?##. 1,1,3"));
        let arrangements: i32 = find_different_arrangements((springs.clone(), damaged.clone()));
        let expected_arrangement: Vec<char> = String::from("..#...#....###.").chars().collect();
       // assert_eq!(arrangements[0], expected_arrangement);
        assert_eq!(arrangements, 4);
    }

    #[test]
    fn finds_arrangement_correctly() {
        let (springs, damaged): (Vec<char>, Vec<i32>) = parse_line(String::from("???.### 1,1,3"));
        let arrangements: i32 = find_different_arrangements((springs.clone(), damaged.clone()));
        let expected_arrangement: Vec<char> = String::from(".#.#.###").chars().collect();
        assert_eq!(arrangements, 1);
    }

    #[test]
    fn parses_line_correctly() {
        let (springs, damaged): (Vec<char>, Vec<i32>) = parse_line(String::from("???.### 1,1,3"));
        let expected_springs: Vec<char> = vec!['?','?','?','.','#','#','#'];
        let expected_damaged: Vec<i32> = vec![1,1,3];
        assert_eq!(springs, expected_springs);
        assert_eq!(damaged, expected_damaged);
    }

    #[test]
    fn test_part1() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/day12/test/input.txt");
       assert_eq!(day12_part1(&d), 21);
    }

    #[test]
    fn part1() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/day12/input.txt");
        assert_eq!(day12_part1(&d), 7195);
    }

    #[test]
    fn test_part2() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/day12/test/input.txt");
        assert_eq!(day12_part2(&d), 525152);
    }

    #[test]
    fn part2() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/day12/input.txt");
        //assert_eq!(day12_part2(&d), 281);
    }

}
use std::ops::Deref;
use std::path::PathBuf;
use crate::read_lines;
use rayon::prelude::*;

fn unfold(springs: &Vec<char>, damaged: &Vec<i32>) -> (Vec<char>, Vec<i32>) {
    let mut unfolded_springs: Vec<char> = vec![];
    unfolded_springs.append(&mut springs.clone());
    unfolded_springs.push('?');
    unfolded_springs.append(&mut springs.clone());
    unfolded_springs.push('?');
    unfolded_springs.append(&mut springs.clone());
    unfolded_springs.push('?');
    unfolded_springs.append(&mut springs.clone());
    unfolded_springs.push('?');
    unfolded_springs.append(&mut springs.clone());
    let mut unfolded_damaged: Vec<i32> = vec![];
    unfolded_damaged.append(&mut damaged.clone());
    unfolded_damaged.append(&mut damaged.clone());
    unfolded_damaged.append(&mut damaged.clone());
    unfolded_damaged.append(&mut damaged.clone());
    unfolded_damaged.append(&mut damaged.clone());
    (unfolded_springs, unfolded_damaged)
}

fn day12_part1(path: &PathBuf) -> i64 {
    // File input.txt must exist in the current path
    let mut count: i64 = 0;
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            let (springs, damaged): (Vec<char>, Vec<i32>) = parse_line(line.unwrap());
            let arrangement: Vec<char> = vec![];
            let expected_damaged: usize = damaged.iter().map(|d| *d as usize).sum();
            count += count_valid_arrangements(arrangement, vec![0,0],(&springs, &damaged), expected_damaged);
        }
    }
    println!("{}", count);
    return count;
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

fn day12_part2(path: &PathBuf) -> i64 {
    let mut line_inputs: Vec<(Vec<char>, Vec<i32>)> = vec![];
    // File input.txt must exist in the current path
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            let (springs, damaged): (Vec<char>, Vec<i32>) = parse_line(line.unwrap());
            line_inputs.push((springs, damaged));
        }
    }
    let mut count: i64 = line_inputs.par_iter_mut().map(|line_input| {
        let (springs, damaged): (Vec<char>, Vec<i32>) = line_input.to_owned();
        let arrangement: Vec<char> = vec![];
        let (unfolded_springs, unfolded_damaged) = unfold(&springs, &damaged);
        let expected_damaged: usize = unfolded_damaged.iter().map(|d| *d as usize).sum();
        return count_valid_arrangements(arrangement, vec![0,0],(&unfolded_springs, &unfolded_damaged), expected_damaged);
    }).sum();
    println!("{}", count);
    return count;
}

fn count_valid_arrangements(mut arrangement: Vec<char>, mut valid_damaged_group_end_idx: Vec<usize>,
                            (springs, damaged): (&Vec<char>, &Vec<i32>), expected_damaged: usize) -> i64 {
    if springs.len() == arrangement.len() {
        if validate_all((&arrangement, &mut valid_damaged_group_end_idx),
                        expected_damaged, damaged, springs.len()) {
            return 1;
        } else {
            return 0;
        }
    }
    if arrangement.len()> 0
        && !validate_partially((&arrangement, &mut valid_damaged_group_end_idx),
                                                   damaged, expected_damaged, springs.len()) {
       return 0;
    }
    let next_spring: char = springs[arrangement.len()];
    if next_spring == '?' {
        let mut arrangement_copy_a: Vec<char> = arrangement.clone();
        arrangement_copy_a.push('.');
        let mut arrangement_copy_b: Vec<char> = arrangement.clone();
        arrangement_copy_b.push('#');
        return count_valid_arrangements(arrangement_copy_a, valid_damaged_group_end_idx.clone(),
                                        (springs, damaged), expected_damaged)
            + count_valid_arrangements(arrangement_copy_b, valid_damaged_group_end_idx.clone(),
                                       (springs, damaged), expected_damaged);
    } else {
        let mut index_to_check: usize = arrangement.len();
        while index_to_check < springs.len() && springs[index_to_check] != '?' {
            arrangement.push(springs[index_to_check]);
            index_to_check += 1;
        }
        return count_valid_arrangements(arrangement, valid_damaged_group_end_idx.clone(),
                                        (springs, damaged), expected_damaged);
    }
}

fn validate_all((arrangement, valid_damaged_group_end_idx): (&Vec<char>, &mut Vec<usize>),
                expected_damaged: usize, damaged: &Vec<i32>, springs_len: usize) -> bool {
    let damaged_count: usize = arrangement.iter().filter(|char| **char == '#').count();
    if damaged_count == expected_damaged {
       return validate_partially((arrangement, valid_damaged_group_end_idx),
                                 damaged, expected_damaged, springs_len);
    }
    return false;
}

fn validate_partially((arrangement, valid_damaged_group_end_idx): (&Vec<char>, &mut Vec<usize>),
                      damaged_groups: &Vec<i32>, expected_damaged: usize, springs_len: usize) -> bool {
    let damaged_count: usize = arrangement.iter().filter(|char| **char == '#').count();
    if expected_damaged < damaged_count {
        return false;
    }
    let missing_damaged: usize = expected_damaged - damaged_count;
    if missing_damaged > springs_len - arrangement.len() {
        return false;
    }
    let mut spring_idx: usize = valid_damaged_group_end_idx[0];
    let mut damaged_group_idx: usize = valid_damaged_group_end_idx[1];
    while spring_idx < arrangement.len() {
        if spring_idx != 0
            && spring_idx == valid_damaged_group_end_idx[0]
            && arrangement[spring_idx] != '.' {
            return false;
        } else if arrangement[spring_idx] == '.' {
            spring_idx += 1;
        } else {
            let mut damaged_spring_idx: usize = 0;
            while damaged_group_idx < damaged_groups.len()
                && spring_idx < arrangement.len()
                && damaged_spring_idx < damaged_groups[damaged_group_idx] as usize {
                if arrangement[spring_idx] != '#' {
                    return false;
                }
                damaged_spring_idx += 1;
                spring_idx += 1;
            }
            if damaged_spring_idx == damaged_groups[damaged_group_idx] as usize {
                if damaged_group_idx < damaged_groups.len()
                    && damaged_spring_idx > 0
                    && spring_idx < arrangement.len()
                    && arrangement[spring_idx] != '.' {
                    return false;
                }
                valid_damaged_group_end_idx[0] = spring_idx;
                damaged_group_idx += 1;
                valid_damaged_group_end_idx[1] = damaged_group_idx;
            }
        }
    }
    return true;
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::day12::{count_valid_arrangements, day12_part1, day12_part2, parse_line, unfold};

    #[test]
    fn finds_arrangement_correctly8() {
        let (springs, damaged): (Vec<char>, Vec<i32>) = parse_line(String::from("..???#??????#?????? 3,6"));
        let arrangement: Vec<char> = vec![];
        let expected_damaged: usize = damaged.iter().map(|d| *d as usize).sum();
        let valid_damaged_group_end_idx: usize = 0;
        let count: i64 = count_valid_arrangements(arrangement, vec![0,0], (&springs, &damaged), expected_damaged);
        assert_eq!(count, 15);
    }

    #[test]
    fn finds_arrangement_correctly7() {
        let (springs, damaged): (Vec<char>, Vec<i32>) = parse_line(String::from("#??.???### 1,1,5"));
        let arrangement: Vec<char> = vec![];
        let expected_damaged: usize = damaged.iter().map(|d| *d as usize).sum();
        let valid_damaged_group_end_idx: usize = 0;
        let count: i64 = count_valid_arrangements(arrangement, vec![0,0], (&springs, &damaged), expected_damaged);
        assert_eq!(count, 1);
    }

    #[test]
    fn finds_arrangement_correctly6() {
        let (springs, damaged): (Vec<char>, Vec<i32>) = parse_line(String::from("?###???????? 3,2,1"));
        let arrangement: Vec<char> = vec![];
        let expected_damaged: usize = damaged.iter().map(|d| *d as usize).sum();
        let valid_damaged_group_end_idx: usize = 0;
        let count: i64 = count_valid_arrangements(arrangement, vec![0,0], (&springs, &damaged), expected_damaged);
        assert_eq!(count, 10);
    }

    #[test]
    fn finds_arrangement_correctly5() {
        let (springs, damaged): (Vec<char>, Vec<i32>) = parse_line(String::from("????.######..#####. 1,6,5"));
        let arrangement: Vec<char> = vec![];
        let expected_damaged: usize = damaged.iter().map(|d| *d as usize).sum();
        let valid_damaged_group_end_idx: usize = 0;
        let count: i64 = count_valid_arrangements(arrangement, vec![0,0], (&springs, &damaged), expected_damaged);
        assert_eq!(count, 4);
    }

    #[test]
    fn finds_arrangement_correctly4() {
        let (springs, damaged): (Vec<char>, Vec<i32>) = parse_line(String::from("????.#...#... 4,1,1"));
        let arrangement: Vec<char> = vec![];
        let expected_damaged: usize = damaged.iter().map(|d| *d as usize).sum();
        let valid_damaged_group_end_idx: usize = 0;
        let count: i64 = count_valid_arrangements(arrangement, vec![0,0], (&springs, &damaged), expected_damaged);
        assert_eq!(count, 1);
    }

    #[test]
    fn finds_arrangement_correctly3() {
        let (springs, damaged): (Vec<char>, Vec<i32>) = parse_line(String::from("?#?#?#?#?#?#?#? 1,3,1,6"));
        let arrangement: Vec<char> = vec![];
        let expected_damaged: usize = damaged.iter().map(|d| *d as usize).sum();
        let valid_damaged_group_end_idx: usize = 0;
        let count: i64 = count_valid_arrangements(arrangement, vec![0,0], (&springs, &damaged), expected_damaged);
        assert_eq!(count, 1);
    }

    #[test]
    fn finds_arrangement_correctly2() {
        let (springs, damaged): (Vec<char>, Vec<i32>) = parse_line(String::from(".??..??...?##. 1,1,3"));
        let arrangement: Vec<char> = vec![];
        let expected_damaged: usize = damaged.iter().map(|d| *d as usize).sum();
        let valid_damaged_group_end_idx: usize = 0;
        let count: i64 = count_valid_arrangements(arrangement, vec![0,0], (&springs, &damaged), expected_damaged);
        assert_eq!(count, 4);
    }

    #[test]
    fn finds_arrangement_correctly() {
        let (springs, damaged): (Vec<char>, Vec<i32>) = parse_line(String::from("???.### 1,1,3"));
        let arrangement: Vec<char> = vec![];
        let expected_damaged: usize = damaged.iter().map(|d| *d as usize).sum();
        let valid_damaged_group_end_idx: usize = 0;
        let count: i64 = count_valid_arrangements(arrangement, vec![0,0], (&springs, &damaged), expected_damaged);
        assert_eq!(count, 1);
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
        assert_eq!(day12_part2(&d), 33992866292225);
    }

    #[test]
    fn finds_expanded_arrangement_correctly6_faster() {
        let (springs, damaged): (Vec<char>, Vec<i32>) = parse_line(String::from("?###???????? 3,2,1"));
        let (unfolded_springs, unfolded_damaged) = unfold(&springs, &damaged);
        let expected_damaged: usize = unfolded_damaged.iter().map(|d| *d as usize).sum();
        let arrangement: Vec<char> = vec![];
        let valid_damaged_group_end_idx: usize = 0;
        let arrangements: i64 = count_valid_arrangements(arrangement, vec![0,0],
                                                         (&unfolded_springs, &unfolded_damaged), expected_damaged);
        assert_eq!(arrangements, 506250);
    }

    #[test]
    fn finds_expanded_arrangement_correctly5_faster() {
        let (springs, damaged): (Vec<char>, Vec<i32>) = parse_line(String::from("????.######..#####. 1,6,5"));
        let (unfolded_springs, unfolded_damaged) = unfold(&springs, &damaged);
        let expected_damaged: usize = unfolded_damaged.iter().map(|d| *d as usize).sum();
        let arrangement: Vec<char> = vec![];
        let valid_damaged_group_end_idx: usize = 0;
        let arrangements: i64 = count_valid_arrangements(arrangement, vec![0,0],
                                                         (&unfolded_springs, &unfolded_damaged), expected_damaged);
        assert_eq!(arrangements, 2500);
    }

    #[test]
    fn finds_expanded_arrangement_correctly4_faster() {
        let (springs, damaged): (Vec<char>, Vec<i32>) = parse_line(String::from("????.#...#... 4,1,1"));
        let (unfolded_springs, unfolded_damaged) = unfold(&springs, &damaged);
        let expected_damaged: usize = unfolded_damaged.iter().map(|d| *d as usize).sum();
        let arrangement: Vec<char> = vec![];
        let valid_damaged_group_end_idx: usize = 0;
        let arrangements: i64 = count_valid_arrangements(arrangement, vec![0,0],
                                                         (&unfolded_springs, &unfolded_damaged), expected_damaged);
        assert_eq!(arrangements, 16);
    }

    #[test]
    fn finds_expanded_arrangement_correctly3_faster() {
        let (springs, damaged): (Vec<char>, Vec<i32>) = parse_line(String::from("?#?#?#?#?#?#?#? 1,3,1,6"));
        let (unfolded_springs, unfolded_damaged) = unfold(&springs, &damaged);
        let expected_damaged: usize = unfolded_damaged.iter().map(|d| *d as usize).sum();
        let arrangement: Vec<char> = vec![];
        let valid_damaged_group_end_idx: usize = 0;
        let arrangements: i64 = count_valid_arrangements(arrangement, vec![0,0],
                                                         (&unfolded_springs, &unfolded_damaged), expected_damaged);
        assert_eq!(arrangements, 1);
    }

    #[test]
    fn finds_expanded_arrangement_correctly2_faster() {
        let (springs, damaged): (Vec<char>, Vec<i32>) = parse_line(String::from(".??..??...?##. 1,1,3"));
        let (unfolded_springs, unfolded_damaged) = unfold(&springs, &damaged);
        let expected_damaged: usize = unfolded_damaged.iter().map(|d| *d as usize).sum();
        let arrangement: Vec<char> = vec![];
        let valid_damaged_group_end_idx: usize = 0;
        let arrangements: i64 = count_valid_arrangements(arrangement, vec![0,0],
                                                         (&unfolded_springs, &unfolded_damaged), expected_damaged);        let expected_arrangement: Vec<char> = String::from("..#...#....###.").chars().collect();
        assert_eq!(arrangements, 16384);
    }

    #[test]
    fn finds_expanded_arrangement_correctly_faster() {
        let (springs, damaged): (Vec<char>, Vec<i32>) = parse_line(String::from("???.### 1,1,3"));
        let (unfolded_springs, unfolded_damaged) = unfold(&springs, &damaged);
        let expected_damaged: usize = unfolded_damaged.iter().map(|d| *d as usize).sum();
        let arrangement: Vec<char> = vec![];
        let valid_damaged_group_end_idx: usize = 0;
        let arrangements: i64 = count_valid_arrangements(arrangement, vec![0,0],
                                                         (&unfolded_springs, &unfolded_damaged), expected_damaged);
        let expected_arrangement: Vec<char> = String::from(".#.#.###").chars().collect();
        assert_eq!(arrangements, 1);
    }

}
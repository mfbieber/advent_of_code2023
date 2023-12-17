use std::path::PathBuf;
use crate::read_lines;

fn day11_part2(path: &PathBuf) -> i32 {
    let mut sum: i32 = 0;
    // File input.txt must exist in the current path
    if let Ok(lines) = read_lines(path) {
        for line in lines {

        }
    }
    println!("{}", sum);
    return sum;
}

fn read_input(path: &PathBuf) -> Vec<Vec<char>> {
    let mut space: Vec<Vec<char>> = vec![];
    // File input.txt must exist in the current path
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            let space_line: Vec<char> = line
                .as_ref()
                .unwrap()
                .chars()
                .collect::<Vec<char>>();
            space.push(space_line);
        }
    }
    return space;
}

fn expand_space(space: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut expanded_space_x: Vec<Vec<char>> = vec![];
    let mut y: usize = 0;
    let mut x: usize = 0;
    while y < space.len() {
        let mut no_galaxy_in_line: bool = true;
        while x < space[y].len() {
            if space[y][x] != '.'{
                no_galaxy_in_line = false;
            }
            x += 1;
        }
        expanded_space_x.push(space[y].clone());
        if no_galaxy_in_line {
            expanded_space_x.push(space[y].clone());
        }
        y += 1;
        x = 0;
    }
    y = 0;
    x = 0;
    let mut shifts: usize = 0;
    while x < space[0].len() {
        let mut no_galaxy_in_line: bool = true;
        while y < space.len() {
            if space[y][x] != '.'{
                no_galaxy_in_line = false;
            }
            y += 1;
        }
        if no_galaxy_in_line {
            let mut i: usize = 0;
            while i < expanded_space_x.len() {
                expanded_space_x[i].insert(x+shifts, '.');
                i += 1;
            }
            shifts += 1;
        }
        x += 1;
        y = 0;
    }
    return expanded_space_x;
}

fn find_shortest_paths_to_galaxies_sum(space: &Vec<Vec<char>>) -> i32 {
    let mut galaxies: Vec<(usize, usize)> = vec![];
    let mut y: usize = 0;
    let mut x: usize = 0;
    space.iter().for_each(|line| {
        line.iter().for_each(|coordinate| {
            if *coordinate == '#' {
                galaxies.push((y, x));
            }
            x += 1;
        });
        y += 1;
        x = 0;
    });
    let mut sum: i32 = 0;
    let mut i: usize = 0;
    while i < galaxies.len() {
        let mut j: usize = i+1;
        while j < galaxies.len() {
            let path: i32 = (galaxies[i].0 as i32 - galaxies[j].0 as i32).abs()
                + (galaxies[i].1 as i32 - galaxies[j].1 as i32).abs();
            sum = sum + path;
            j += 1;
        }
        i += 1;
    }
    return sum;
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::day11::{expand_space, find_shortest_paths_to_galaxies_sum, read_input};

    #[test]
    fn test_finds_sum_shortest_paths() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/day11/test/input.txt");
        let space: Vec<Vec<char>> = read_input(&d);
        let expanded_space: Vec<Vec<char>> = expand_space(&space);
        let sum_paths: i32 = find_shortest_paths_to_galaxies_sum(&expanded_space);
        assert_eq!(sum_paths, 374);
    }

    #[test]
    fn test_expands_space_correctly() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/day11/test/input.txt");
        let space: Vec<Vec<char>> = read_input(&d);
        let expanded_space: Vec<Vec<char>> = expand_space(&space);
        let expected_space: Vec<Vec<char>> = vec![
            vec!['.','.','.','.','#','.','.','.','.','.','.','.','.'],
            vec!['.','.','.','.','.','.','.','.','.','#','.','.','.'],
            vec!['#','.','.','.','.','.','.','.','.','.','.','.','.'],
            vec!['.','.','.','.','.','.','.','.','.','.','.','.','.'],
            vec!['.','.','.','.','.','.','.','.','.','.','.','.','.'],
            vec!['.','.','.','.','.','.','.','.','#','.','.','.','.'],
            vec!['.','#','.','.','.','.','.','.','.','.','.','.','.'],
            vec!['.','.','.','.','.','.','.','.','.','.','.','.','#'],
            vec!['.','.','.','.','.','.','.','.','.','.','.','.','.'],
            vec!['.','.','.','.','.','.','.','.','.','.','.','.','.'],
            vec!['.','.','.','.','.','.','.','.','.','#','.','.','.'],
            vec!['#','.','.','.','.','#','.','.','.','.','.','.','.'],
        ];
        assert_eq!(expanded_space, expected_space);
    }

    #[test]
    fn test_reads_input_correctly() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/day11/test/input.txt");
        let space: Vec<Vec<char>> = read_input(&d);
        let expected_space: Vec<Vec<char>> = vec![
            vec!['.','.','.','#','.','.','.','.','.','.'],
            vec!['.','.','.','.','.','.','.','#','.','.'],
            vec!['#','.','.','.','.','.','.','.','.','.'],
            vec!['.','.','.','.','.','.','.','.','.','.'],
            vec!['.','.','.','.','.','.','#','.','.','.'],
            vec!['.','#','.','.','.','.','.','.','.','.'],
            vec!['.','.','.','.','.','.','.','.','.','#'],
            vec!['.','.','.','.','.','.','.','.','.','.'],
            vec!['.','.','.','.','.','.','.','#','.','.'],
            vec!['#','.','.','.','#','.','.','.','.','.'],
        ];
        assert_eq!(space, expected_space);
    }

    #[test]
    fn part1() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/day11/input.txt");
        let space: Vec<Vec<char>> = read_input(&d);
        let expanded_space: Vec<Vec<char>> = expand_space(&space);
        let sum_paths: i32 = find_shortest_paths_to_galaxies_sum(&expanded_space);
        assert_eq!(sum_paths, 10033566);
    }

    #[test]
    fn test_part2() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/day11/test/input.txt");
        //assert_eq!(day11_part2(&d), 281);
    }

    #[test]
    fn part2() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/day11/input.txt");
        //assert_eq!(day11_part2(&d), 281);
    }

}
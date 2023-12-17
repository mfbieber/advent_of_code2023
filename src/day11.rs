use std::path::PathBuf;
use crate::read_lines;

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

fn expand_space(space: &Vec<Vec<char>>, expansion_factor: i32) -> (Vec<usize>,Vec<usize>) {
    let mut expanded_y_indices: Vec<usize> = vec![];
    let mut expanded_x_indices: Vec<usize> = vec![];
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
        if no_galaxy_in_line {
            expanded_y_indices.push(y);
        }
        y += 1;
        x = 0;
    }
    y = 0;
    x = 0;
    while x < space[0].len() {
        let mut no_galaxy_in_line: bool = true;
        while y < space.len() {
            if space[y][x] != '.'{
                no_galaxy_in_line = false;
            }
            y += 1;
        }
        if no_galaxy_in_line {
            expanded_x_indices.push(x);
        }
        x += 1;
        y = 0;
    }
    return (expanded_y_indices, expanded_x_indices);
}



fn find_shortest_paths_to_galaxies_sum(space: &Vec<Vec<char>>, expansion_factor: i32,
                                       (expanded_y_indices, expanded_x_indices): (Vec<usize>,Vec<usize>))
    -> i64 {
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
    let mut sum: i64 = 0;
    let mut i: usize = 0;
    while i < galaxies.len() {
        let mut j: usize = i+1;
        while j < galaxies.len() {
            let y_start: i32 = galaxies[i].0 as i32;
            let y_end: i32 = galaxies[j].0 as i32;
            let x_start: i32 = galaxies[i].1 as i32;
            let x_end: i32 = galaxies[j].1 as i32;
            let expanded_y_count: usize = expanded_y_indices.iter().filter(|index| {
                (**index >= (y_start as usize) && **index <= (y_end as usize))
                || (**index <= (y_start as usize) && **index >= (y_end as usize))
            }).count();
            let expanded_x_count: usize = expanded_x_indices.iter().filter(|index| {
                (**index >= (x_start as usize) && **index <= (x_end as usize))
                || (**index <= (x_start as usize) && **index >= (x_end as usize))
            }).count();
            let path: i64 = (galaxies[i].0 as i64 - galaxies[j].0 as i64).abs() + expanded_y_count as i64*expansion_factor as i64
                + (galaxies[i].1 as i64 - galaxies[j].1 as i64).abs() + expanded_x_count as i64*expansion_factor as i64;
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
        let (y_idx, x_idx): (Vec<usize>, Vec<usize>) = expand_space(&space, 1);
        let sum_paths: i64 = find_shortest_paths_to_galaxies_sum(&space, 1,(y_idx, x_idx)).try_into().unwrap();
        assert_eq!(sum_paths, 374);
    }

    #[test]
    fn test_expands_space_correctly() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/day11/test/input.txt");
        let space: Vec<Vec<char>> = read_input(&d);
        let (y_idx, x_idx): (Vec<usize>, Vec<usize>) = expand_space(&space, 1);
        let expected_y_idx: Vec<usize> = vec![3,7];
        let expected_x_idx: Vec<usize> = vec![2,5,8];
        assert_eq!(y_idx, expected_y_idx);
        assert_eq!(x_idx, expected_x_idx);
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
        let (y_idx, x_idx): (Vec<usize>, Vec<usize>) = expand_space(&space, 1);
        let sum_paths: i64 = find_shortest_paths_to_galaxies_sum(&space, 1, (y_idx, x_idx));
        assert_eq!(sum_paths, 10033566);
    }

    #[test]
    fn test_part2_10() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/day11/test/input.txt");
        let space: Vec<Vec<char>> = read_input(&d);
        let (y_idx, x_idx): (Vec<usize>, Vec<usize>) = expand_space(&space, 10 - 1);
        let sum_paths: i64 = find_shortest_paths_to_galaxies_sum(&space, 9, (y_idx, x_idx));
        assert_eq!(sum_paths, 1030);
    }

    #[test]
    fn test_part2_100() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/day11/test/input.txt");
        let space: Vec<Vec<char>> = read_input(&d);
        let (y_idx, x_idx): (Vec<usize>, Vec<usize>) = expand_space(&space, 100 - 1);
        let sum_paths: i64 = find_shortest_paths_to_galaxies_sum(&space, 100 - 1, (y_idx, x_idx));
        assert_eq!(sum_paths, 8410);
    }

    #[test]
    fn part2() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/day11/input.txt");
        let space: Vec<Vec<char>> = read_input(&d);
        let (y_idx, x_idx): (Vec<usize>, Vec<usize>) = expand_space(&space, 1000000 - 1);
        let sum_paths: i64 = find_shortest_paths_to_galaxies_sum(&space, 1000000 - 1, (y_idx, x_idx));
        assert_eq!(sum_paths, 560822911938);
    }

}
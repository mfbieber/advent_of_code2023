use std::ops::{Deref, DerefMut};
use std::path::PathBuf;
use crate::day3::Number;
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

const SIZE: usize = 11322857500;

#[derive(Clone)]
struct MappingRange {
    source: i64,
    dest: i64,
    range: i64
}

impl MappingRange {

    fn get_dest(&self, value: i64) -> i64 {
        let diff: i64 = self.source - self.dest;
        return value - diff;
    }

    fn is_in_dest_range(&self, value: i64) -> bool {
        return (self.dest + self.range) >= value && (self.dest <= value)
    }

    fn is_in_source_range(&self, value: i64) -> bool {
        return (self.source + self.range) >= value && (self.source <= value)
    }
}

#[derive(Clone)]
struct Mapping {
    name: String,
    mapping_ranges: Vec<MappingRange>
}

impl Mapping {
    fn push_mapping_range(&mut self, range: MappingRange) {
        let owned_range = range;
        self.mapping_ranges.push(owned_range);
    }
}

fn day5_part1(path: &PathBuf, seeds: Vec<i64>) -> i64 {

    let mut mappings: Vec<Mapping> = vec![];
    let mut mapping_name = String::from("");
    let mut mr0: Vec<MappingRange> = vec![];
    let mut mr1: Vec<MappingRange> = vec![];
    let mut mr2: Vec<MappingRange> = vec![];
    let mut mr3: Vec<MappingRange> = vec![];
    let mut mr4: Vec<MappingRange> = vec![];
    let mut mr5: Vec<MappingRange> = vec![];
    let mut mr6: Vec<MappingRange> = vec![];

    // File input.txt must exist in the current path
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            let line_clone = line.unwrap().clone();
            if line_clone.len() > 0 {
                if line_clone.ends_with("map:") {
                    mapping_name = line_clone;
                    continue;
                }
                let mapping_range: MappingRange = get_mapping_range(&line_clone, &mapping_name);
                if mapping_name.starts_with("seed-to-soil") {
                    mr0.push(mapping_range);
                } else if mapping_name.starts_with("soil-to-fertilizer") {
                    mr1.push(mapping_range);
                } else if mapping_name.starts_with("fertilizer-to-water") {
                    mr2.push(mapping_range);
                } else if mapping_name.starts_with("water-to-light") {
                    mr3.push(mapping_range);
                } else if mapping_name.starts_with("light-to-temperature") {
                    mr4.push(mapping_range);
                } else if mapping_name.starts_with("temperature-to-humidity") {
                    mr5.push(mapping_range);
                } else if mapping_name.starts_with("humidity-to-location") {
                    mr6.push(mapping_range);
                }
            }
        }
    }
    mappings.push(Mapping {
        name: String::from("seed-to-soil"),
        mapping_ranges: mr0,
    });
    mappings.push(Mapping {
        name: String::from("soil-to-fertilizer"),
        mapping_ranges: mr1,
    });
    mappings.push(Mapping {
        name: String::from("fertilizer-to-water"),
        mapping_ranges: mr2,
    });
    mappings.push(Mapping {
        name: String::from("water-to-light"),
        mapping_ranges: mr3,
    });
    mappings.push(Mapping {
        name: String::from("light-to-temperature"),
        mapping_ranges: mr4,
    });
    mappings.push(Mapping {
        name: String::from("temperature-to-humidity"),
        mapping_ranges: mr5,
    });
    mappings.push(Mapping {
        name: String::from("humidity-to-location"),
        mapping_ranges: mr6,
    });
    println!("mappings");

    let mut smallest_location: i64 = SIZE as i64;
    for seed in seeds {
        let mut dest: i64 = seed;
        for mapping in &mappings {
            for range in &mapping.mapping_ranges {
                if range.is_in_source_range(dest) {
                    dest = range.get_dest(dest);
                    break;
                }
            }
        }
        if dest < smallest_location {
            smallest_location = dest;
        }
    }
    println!("{}", smallest_location);
    return smallest_location;
}

fn get_mapping_range(line_ref: &String, mut mapping_name: &String) -> MappingRange {
    let split_line: Vec<i64> = line_ref.split(" ")
        .map(|item| item.parse().unwrap())
        .collect();
    return MappingRange {
        source: split_line[1],
        dest: split_line[0],
        range: split_line[2],
    };
}

//destination range start, the source range start, and the range length

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::day5::{day5_part1, MappingRange};

    #[test]
    fn test_is_in_source_range() {
        let mapping_range: MappingRange = MappingRange {
            source: 0,
            dest: 6,
            range: 5,
        };
        assert_eq!(mapping_range.is_in_source_range(2), true);
    }

    #[test]
    fn test_is_in_dest_range() {
        let mapping_range: MappingRange = MappingRange {
            source: 0,
            dest: 6,
            range: 5,
        };
        assert_eq!(mapping_range.is_in_dest_range(2), false);
    }

    #[test]
    fn test_get_dest() {
        let mapping_range: MappingRange = MappingRange {
            source: 0,
            dest: 6,
            range: 5,
        };
        assert_eq!(mapping_range.get_dest(2), 8);
    }

    #[test]
    fn test_get_dest2() {
        let mapping_range: MappingRange = MappingRange {
            source: 6,
            dest: 0,
            range: 5,
        };
        assert_eq!(mapping_range.get_dest(8), 2);
    }

    #[test]
    fn test_part1() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/day5/test/input.txt");
        let seeds: Vec<i64> = vec![79, 14, 55, 13];
        assert_eq!(day5_part1(&d, seeds), 35);
    }

    #[test]
    fn part1() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/day5/input.txt");
        let seeds: Vec<i64> = vec![5844012, 110899473, 1132285750, 58870036, 986162929, 109080640,
            3089574276, 100113624, 2693179996, 275745330, 2090752257, 201704169, 502075018,
            396653347, 1540050181, 277513792, 1921754120, 26668991, 3836386950, 66795009];
        assert_eq!(day5_part1(&d, seeds), 825516882);
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
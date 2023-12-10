use std::collections::HashSet;
use std::io::Write;
use std::ops::{Deref, DerefMut};
use std::path::PathBuf;
use crate::read_lines;

#[derive(Debug)]
struct SeedRange {
    source: i64,
    range: i64
}

fn day5_part2(path: &PathBuf, seed_ranges: Vec<SeedRange>) -> i64 {
    println!("start");
    let mappings = read_mappings(path);
    let mut smallest_location: i64 = SIZE as i64;
    let mut location: i64 = 0;
    for seed_range in seed_ranges {
        println!("{:?}", seed_range);
        let source: i64 = seed_range.source;
        let range: i64 = seed_range.range;
        let mut i: i64 = 0;
        while i < range {
            location = find_smallest_location_in_mappings_for_seed(source+i, &mappings);
            i += 1;
            if location < smallest_location {
                smallest_location = location;
                println!("{}", smallest_location);
            }
        }
    }
    return smallest_location;
}

fn find_smallest_location_in_mappings_for_seed(seed: i64, mappings: &Vec<Mapping>) -> i64 {
    let mut smallest_location: i64 = SIZE as i64;
    let mut dest: i64 = seed;
    for mapping in mappings {
        for range in &mapping.mapping_ranges {
            if range.is_in_source_range(dest) {
                dest = range.get_dest(dest);
                break;
            }
        }
    }
    if dest < smallest_location {
        smallest_location = dest;
        println!("{}", smallest_location)
    }
    println!("{}", smallest_location);
    return smallest_location;
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

fn get_smallest_location_for_seeds(path: &PathBuf, seeds: HashSet<i64>) -> i64 {
    let mappings = read_mappings(path);
    return find_smallest_location_in_mappings(seeds, &mappings);
}

fn find_smallest_location_in_mappings(seeds: HashSet<i64>, mappings: &Vec<Mapping>) -> i64 {
    let mut smallest_location: i64 = SIZE as i64;
    for seed in seeds {
        let mut dest: i64 = seed;
        for mapping in mappings {
            for range in &mapping.mapping_ranges {
                if range.is_in_source_range(dest) {
                    dest = range.get_dest(dest);
                    break;
                }
            }
        }
        if dest < smallest_location {
            smallest_location = dest;
            println!("{}", smallest_location)
        }
    }
    println!("{}", smallest_location);
    return smallest_location;
}

fn read_mappings(path: &PathBuf) -> Vec<Mapping> {
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

    push_mapping(&mut mappings, mr0, String::from("seed-to-soil"));
    push_mapping(&mut mappings, mr1, String::from("soil-to-fertilizer"));
    push_mapping(&mut mappings, mr2, String::from("fertilizer-to-water"));
    push_mapping(&mut mappings, mr3, String::from("water-to-light"));
    push_mapping(&mut mappings, mr4, String::from("light-to-temperature"));
    push_mapping(&mut mappings, mr5, String::from("temperature-to-humidity"));
    push_mapping(&mut mappings, mr6, String::from("humidity-to-location"));
    println!("mappings");
    mappings
}

fn push_mapping(mappings: &mut Vec<Mapping>, mut mapping_ranges: Vec<MappingRange>, mut name: String) {
    mappings.push(Mapping {
        name,
        mapping_ranges,
    });
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
    use std::collections::HashSet;
    use std::path::PathBuf;
    use crate::day5::{day5_part2, get_smallest_location_for_seeds, MappingRange, SeedRange};

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
        let mut seeds: HashSet<i64> = HashSet::new();
        seeds.insert(79);
        seeds.insert(14);
        seeds.insert(55);
        seeds.insert(13);
        assert_eq!(get_smallest_location_for_seeds(&d, seeds), 35);
    }

    #[test]
    fn part1() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/day5/input.txt");
        let mut seeds: HashSet<i64> = HashSet::new();
        seeds.insert(5844012);
        seeds.insert(110899473);
        seeds.insert(1132285750);
        seeds.insert(58870036);
        seeds.insert(986162929);
        seeds.insert(109080640);
        seeds.insert(3089574276);
        seeds.insert(100113624);
        seeds.insert(2693179996);
        seeds.insert(275745330);
        seeds.insert(2090752257);
        seeds.insert(201704169);
        seeds.insert(502075018);
        seeds.insert(396653347);
        seeds.insert(1540050181);
        seeds.insert(277513792);
        seeds.insert(1921754120);
        seeds.insert(26668991);
        seeds.insert(3836386950);
        seeds.insert(66795009);
        assert_eq!(get_smallest_location_for_seeds(&d, seeds), 825516882);
    }

    #[test]
    fn test_part2() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/day5/test/input.txt");
        let mut seed_ranges: Vec<SeedRange> = vec![];
        seed_ranges.push(SeedRange {
            source: 79,
            range: 14,
        });
        seed_ranges.push(SeedRange {
            source: 55,
            range: 13,
        });
        assert_eq!(day5_part2(&d, seed_ranges), 46);
    }

    #[test]
    fn part2() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/day5/input.txt");
        let mut seed_ranges: Vec<SeedRange> = vec![];
        seed_ranges.push(SeedRange {
            source: 5844012,
            range: 110899473,
        });
        seed_ranges.push(SeedRange {
            source: 1132285750,
            range: 58870036,
        });
        seed_ranges.push(SeedRange {
            source: 986162929,
            range: 109080640,
        });
        seed_ranges.push(SeedRange {
            source: 3089574276,
            range: 100113624,
        });
        seed_ranges.push(SeedRange {
            source: 2693179996,
            range: 275745330,
        });
        seed_ranges.push(SeedRange {
            source: 2090752257,
            range: 201704169,
        });
        seed_ranges.push(SeedRange {
            source: 502075018,
            range: 396653347,
        });
        seed_ranges.push(SeedRange {
            source: 1540050181,
            range: 277513792,
        });
        seed_ranges.push(SeedRange {
            source: 1921754120,
            range: 26668991,
        });
        seed_ranges.push(SeedRange {
            source: 3836386950,
            range: 66795009,
        });
        assert_eq!(day5_part2(&d, seed_ranges), 136096660);
    }

}
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

const SIZE: usize = 11322857500;

fn day5_part1(path: &PathBuf, seeds: Vec<i64>) -> i64 {
    let mut sum: i32 = 0;

    let mut seed_to_soil: Vec<i64> = vec![0; SIZE];
    let mut soil_to_fertilizer: Vec<i64> = vec![0; SIZE];
    let mut fertilizer_to_water: Vec<i64> = vec![0; SIZE];
    let mut water_to_light: Vec<i64> = vec![0; SIZE];
    let mut light_to_temperature: Vec<i64> = vec![0; SIZE];
    let mut temperature_to_humidity: Vec<i64> = vec![0; SIZE];
    let mut humidity_to_location: Vec<i64> = vec![0; SIZE];

    // File input.txt must exist in the current path
    if let Ok(lines) = read_lines(path) {
        let mut mapping = String::from("");
        for line in lines {
            let line_clone = line.unwrap().clone();
            if line_clone.ends_with("map:") {
                mapping = line_clone;
                continue;
            }
            if line_clone.len() > 0 {
                do_mapping(&mut seed_to_soil, &mut soil_to_fertilizer, &mut fertilizer_to_water,
                           &mut water_to_light, &mut light_to_temperature, &mut temperature_to_humidity,
                           &mut humidity_to_location, &line_clone, &mapping);
            }
            println!("{}", mapping);
        }
    }

    let mut smallest_location: i64 = SIZE as i64;
    for seed in seeds {
        let mut soil: i64 = seed_to_soil[seed as usize];
        if soil == 0 {
            soil = seed;
        }
        let mut fertilizer: i64 = soil_to_fertilizer[soil as usize];
        if fertilizer == 0 {
            fertilizer = soil;
        }
        let mut water: i64 = fertilizer_to_water[fertilizer as usize];
        if water == 0 {
            water = fertilizer;
        }
        let mut light: i64 = water_to_light[water as usize];
        if light == 0 {
            light = water;
        }
        let mut temperature: i64 = light_to_temperature[light as usize];
        if temperature == 0 {
            temperature = light;
        }
        let mut humidity: i64 = temperature_to_humidity[temperature as usize];
        if humidity == 0 {
            humidity = temperature;
        }
        let mut location: i64 = humidity_to_location[humidity as usize];
        if location == 0 {
            location = humidity;
        }
        if location < smallest_location {
            smallest_location = location;
        }
    }

    println!("{}", smallest_location);
    return smallest_location;
}

fn do_mapping(seed_to_soil: &mut Vec<i64>, soil_to_fertilizer: &mut Vec<i64>,
              fertilizer_to_water: &mut Vec<i64>, water_to_light: &mut Vec<i64>,
              light_to_temperature: &mut Vec<i64>, temperature_to_humidity: &mut Vec<i64>,
              humidity_to_location: &mut Vec<i64>, line_ref: &String, mut mapping: &String) {
    let split_line: Vec<i64> = line_ref.split(" ")
        .map(|item| item.parse().unwrap())
        .collect();
    let mut source_range_start: i64 = split_line[1];
    let mut dest_range_start: i64 = split_line[0];
    let range_length: i64 = split_line[2];
    if mapping.starts_with("seed-to-soil") {
        let mut i : i64 = 0;
        while i < range_length {
            seed_to_soil[source_range_start as usize] = dest_range_start;
            dest_range_start += 1;
            source_range_start += 1;
            i += 1;
        }
    } else if mapping.starts_with("soil-to-fertilizer") {
        let mut i : i64 = 0;
        while i < range_length {
            soil_to_fertilizer[source_range_start as usize] = dest_range_start;
            dest_range_start += 1;
            source_range_start += 1;
            i += 1;
        }
    } else if mapping.starts_with("fertilizer-to-water") {
        let mut i : i64 = 0;
        while i < range_length {
            fertilizer_to_water[source_range_start as usize] = dest_range_start;
            dest_range_start += 1;
            source_range_start += 1;
            i += 1;
        }
    } else if mapping.starts_with("water-to-light") {
        let mut i : i64 = 0;
        while i < range_length {
            water_to_light[source_range_start as usize] = dest_range_start;
            dest_range_start += 1;
            source_range_start += 1;
            i += 1;
        }
    } else if mapping.starts_with("light-to-temperature") {
        let mut i : i64 = 0;
        while i < range_length {
            light_to_temperature[source_range_start as usize] = dest_range_start;
            dest_range_start += 1;
            source_range_start += 1;
            i += 1;
        }
    } else if mapping.starts_with("temperature-to-humidity") {
        let mut i : i64 = 0;
        while i < range_length {
            temperature_to_humidity[source_range_start as usize] = dest_range_start;
            dest_range_start += 1;
            source_range_start += 1;
            i += 1;
        }
    } else if mapping.starts_with("humidity-to-location") {
        let mut i : i64 = 0;
        while i < range_length {
            humidity_to_location[source_range_start as usize] = dest_range_start;
            dest_range_start += 1;
            source_range_start += 1;
            i += 1;
        }
    }
}

//destination range start, the source range start, and the range length

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::day5::day5_part1;

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
        assert_eq!(day5_part1(&d, seeds), 35);
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
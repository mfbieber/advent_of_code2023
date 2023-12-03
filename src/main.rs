extern crate core;

use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

mod day1;
mod day2;
mod day3;

fn main() {}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
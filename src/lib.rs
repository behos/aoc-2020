use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_lines(path: &str) -> impl Iterator<Item = String> {
    let file = File::open(path).expect("Could not open file.");
    let reader = BufReader::new(file);
    reader.lines().map(|s| s.expect("Could not read line."))
}

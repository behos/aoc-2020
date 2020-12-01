use aoc_2020::read_lines;
use std::collections::HashSet;

fn main() {
    let mut entry_set = HashSet::<u32>::new();
    read_lines("./data/day-01.txt")
        .map(|s| s.parse::<u32>().expect("Failed to parse line."))
        .for_each(|entry| {
            let complement = 2020 - entry;
            if entry_set.contains(&complement) {
                println!(
                    "Found {} and {}. Result is {}",
                    entry,
                    complement,
                    entry * complement
                )
            }
            entry_set.insert(entry);
        });
}

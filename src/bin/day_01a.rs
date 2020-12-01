use aoc_2020::read_entries;
use std::collections::HashSet;

fn main() {
    let mut entry_set = HashSet::<u32>::new();
    read_entries::<u32>("./data/day-01.txt").for_each(|entry| {
        let complement = 2020 - entry;
        if entry_set.contains(&complement) {
            println!(
                "Found {} and {}. Result is {}",
                entry,
                complement,
                entry * complement
            );
        }
        entry_set.insert(entry);
    });
}

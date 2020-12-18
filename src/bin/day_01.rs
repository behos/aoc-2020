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

    let entries = entry_set.iter().collect::<Vec<_>>();
    // Let's do a naive solution.
    for i in 0..entries.len() {
        let iv = entries[i];
        for j in i..entries.len() {
            let jv = entries[j];
            for k in j..entries.len() {
                let kv = entries[k];
                if (iv + jv + kv) == 2020 {
                    println!(
                        "Found result {} {} {} which has a product of {}",
                        iv,
                        jv,
                        kv,
                        iv * jv * kv
                    )
                }
            }
        }
    }
}

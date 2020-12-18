use aoc_2020::read_entries;

fn main() {
    let entries = read_entries::<u32>("./data/day-01.txt").collect::<Vec<_>>();
    // Let's do a naive solution.
    for i in 0..entries.len() {
        let iv = entries[i];
        for j in i..entries.len() {
            let jv = entries[j];
            if (iv + jv) == 2020 {
                println!("Part 1: {} * {} = {}", iv, jv, iv * jv)
            }
            for k in j..entries.len() {
                let kv = entries[k];
                if (iv + jv + kv) == 2020 {
                    println!(
                        "Part 2: {} * {} * {} = {}",
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

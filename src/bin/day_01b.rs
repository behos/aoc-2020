use aoc_2020::read_lines;

fn main() {
    let entries: Vec<_> = read_lines("./data/day-01.txt")
        .map(|s| s.parse::<usize>().expect("Failed to parse line."))
        .collect();

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

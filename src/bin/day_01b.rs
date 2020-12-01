use aoc_2020::read_entries;

fn main() {
    let entries: Vec<_> = read_entries::<usize>("./data/day-01.txt").collect();

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

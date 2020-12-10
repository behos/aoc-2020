use aoc_2020::read_entries;

fn main() {
    let mut numbers: Vec<_> =
        read_entries::<usize>("./data/day-10.txt").collect();
    numbers.sort();
    let mut count_1 = if numbers[0] == 1 { 1 } else { 0 };
    let mut count_3 = if numbers[0] == 3 { 2 } else { 1 };
    for i in 1..numbers.len() {
        match numbers[i] - numbers[i - 1] {
            3 => {
                count_3 += 1;
            }
            1 => {
                count_1 += 1;
            }
            _ => {}
        }
    }
    println!("Result: {} * {} = {}", count_1, count_3, count_1 * count_3);

    // let perms = find_permutations(&numbers);
    // println!("Perms {}", perms);
}

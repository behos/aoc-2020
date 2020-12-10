use aoc_2020::read_entries;
use std::usize;

fn main() {
    let mut numbers: Vec<_> =
        read_entries::<usize>("./data/day-10.txt").collect();
    numbers.sort();

    println!("Jumps {}", get_jumps(&numbers));
    println!("Permutations {}", get_permutations(&numbers));
}

fn get_jumps(numbers: &[usize]) -> usize {
    let (j1, j3) = (0..numbers.len()).fold((0, 1), |(j1, j3), index| {
        let prev = if index == 0 { 0 } else { numbers[index - 1] };
        let cur = numbers[index];
        match cur - prev {
            1 => (j1 + 1, j3),
            3 => (j1, j3 + 1),
            _ => (j1, j3),
        }
    });
    j1 * j3
}

fn get_permutations(numbers: &[usize]) -> usize {
    let mut paths = vec![0 as usize; numbers.len()];
    for i in 0..numbers.len() {
        let val = numbers[i];
        if val <= 3 {
            paths[i] += 1;
        }
        let mut j = i;
        while j > 0 && val - numbers[j - 1] <= 3 {
            paths[i] += paths[j - 1];
            j -= 1;
        }
    }
    paths[numbers.len() - 1]
}

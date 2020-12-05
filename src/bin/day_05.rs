use aoc_2020::read_entries;
use std::{cmp, usize::MAX};

fn main() {
    let (min, max, sum) = read_entries::<String>("./data/day-05.txt")
        .map(to_seat_id)
        .fold((MAX, 0, 0), |(min, max, sum), current| {
            (
                cmp::min(min, current),
                cmp::max(max, current),
                sum + current,
            )
        });
    println!("Found max seat id {}", max);
    println!("Found my seat {}", find_missing(min, max, sum));
}

fn to_seat_id(id: String) -> usize {
    usize::from_str_radix(
        &id.chars()
            .map(|c| match c {
                'B' | 'R' => '1',
                'F' | 'L' => '0',
                _ => panic!("Unexpected char"),
            })
            .collect::<String>(),
        2,
    )
    .expect("This is for sure an int")
}

fn find_missing(min: usize, max: usize, sum: usize) -> usize {
    (max - (min - 1)) * (max + min) / 2 - sum
}

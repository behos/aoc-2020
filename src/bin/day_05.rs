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
    let (row_range, col_range) = id.chars().enumerate().fold(
        ((0, 127), (0, 7)),
        |(row, col), (index, c)| {
            if index < 7 {
                (split(row, c == 'B'), col)
            } else {
                (row, split(col, c == 'R'))
            }
        },
    );
    row_range.0 * 8 + col_range.0
}

fn split((min, max): (usize, usize), high: bool) -> (usize, usize) {
    let mid = (min + max) / 2;
    if high {
        (mid + 1, max)
    } else {
        (min, mid)
    }
}

fn find_missing(min: usize, max: usize, sum: usize) -> usize {
    (max - (min - 1)) * (max + min) / 2 - sum
}

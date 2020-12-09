use aoc_2020::read_entries;
use std::{cmp, usize};

fn main() {
    let numbers: Vec<_> = read_entries::<usize>("./data/day-09.txt").collect();
    let inv = first_invalid(&numbers).expect("Didn't find invalid number.");
    println!("{} is invalid.", inv);
    let enc_weakness =
        find_encryption_weakness(inv, &numbers).expect("Didn't find weakness.");
    println!("Encryption weakness {}", enc_weakness);
}

fn first_invalid(numbers: &[usize]) -> Option<usize> {
    for i in 25..numbers.len() {
        let number = numbers[i];
        if !is_valid(number, &numbers[i - 25..i]) {
            return Some(number);
        }
    }
    None
}

fn is_valid(num: usize, preamble: &[usize]) -> bool {
    preamble.iter().enumerate().any(|(index, value)| {
        num > *value
            && preamble[index + 1..preamble.len()].contains(&(num - value))
    })
}

fn find_encryption_weakness(num: usize, numbers: &[usize]) -> Option<usize> {
    let mut min_index = 0;
    let mut max_index = 0;
    let mut running_sum = numbers[0];

    for i in 1..numbers.len() {
        let value = numbers[i];

        running_sum += value;
        max_index += 1;

        while running_sum > num && min_index < max_index {
            running_sum -= numbers[min_index];
            min_index += 1;
        }

        if running_sum == num {
            return Some(min_plus_max(&numbers[min_index..=max_index]));
        }
    }
    None
}

fn min_plus_max(numbers: &[usize]) -> usize {
    let mut min_val = usize::MAX;
    let mut max_val = usize::MIN;
    for j in 0..numbers.len() {
        min_val = cmp::min(numbers[j], min_val);
        max_val = cmp::max(numbers[j], max_val);
    }
    return min_val + max_val;
}

use aoc_2020::read_entries;

fn main() {
    let mut ids: Vec<_> = read_entries::<String>("./data/day-05.txt")
        .map(to_seat_id)
        .collect();
    ids.sort_unstable();
    println!("Found max seat id {}", ids[ids.len() - 1]);
    println!("Found my seat {}", find_missing(&ids));
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

fn find_missing(ids: &[usize]) -> usize {
    let mut range = (0, ids.len());
    let offset = ids[0];
    while range.0 < range.1 {
        let (min, max) = range;
        let mid = (min + max) / 2;
        range = split(range, ids[mid] == mid + offset);
    }
    range.0 + offset
}

use anyhow::{bail, Error, Result};
use aoc_2020::read_entries;
use std::str::FromStr;

#[derive(PartialEq)]
enum Space {
    Occupied,
    Empty,
    Floor,
}

impl Space {
    fn from_char(input: char) -> Result<Self> {
        Ok(match input {
            'L' => Space::Empty,
            '#' => Space::Occupied,
            '.' => Space::Floor,
            hm => bail!("What's a {}", hm),
        })
    }
}

#[derive(PartialEq)]
struct Seating {
    spaces: Vec<Vec<Space>>,
}

impl Seating {
    fn width(&self) -> usize {
        self.spaces[0].len()
    }

    fn height(&self) -> usize {
        self.spaces.len()
    }

    fn next(&self) -> Self {
        Seating {
            spaces: self
                .spaces
                .iter()
                .enumerate()
                .map(|(i, row)| {
                    row.iter()
                        .enumerate()
                        .map(|(j, seat)| match seat {
                            Space::Floor => Space::Floor,
                            Space::Occupied => {
                                if self.occupied_neighbors(i, j) >= 4 {
                                    Space::Empty
                                } else {
                                    Space::Occupied
                                }
                            }
                            Space::Empty => {
                                if self.occupied_neighbors(i, j) == 0 {
                                    Space::Occupied
                                } else {
                                    Space::Empty
                                }
                            }
                        })
                        .collect()
                })
                .collect(),
        }
    }

    fn occupied_neighbors(&self, x: usize, y: usize) -> usize {
        let positions = (x as isize - 1..=x as isize + 1)
            .map(|i| (y as isize - 1..=y as isize + 1).map(move |j| (i, j)))
            .flatten();

        positions.fold(0, |acc, (i, j)| {
            if i < 0
                || j < 0
                || i as usize >= self.height()
                || j as usize >= self.width()
                || (i as usize, j as usize) == (x, y)
                || self.spaces[i as usize][j as usize] != Space::Occupied
            {
                acc
            } else {
                acc + 1
            }
        })
    }

    fn count_occupied(&self) -> usize {
        self.spaces
            .iter()
            .map(|row| row.iter().filter(|s| s == &&Space::Occupied).count())
            .sum()
    }
}

struct Entry(Vec<Space>);

impl FromStr for Entry {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self> {
        let vec = input
            .chars()
            .map(Space::from_char)
            .collect::<Result<Vec<Space>>>()?;
        Ok(Self(vec))
    }
}

fn main() {
    let spaces: Vec<Vec<_>> = read_entries::<Entry>("./data/day-11.txt")
        .map(|Entry(spaces)| spaces)
        .collect();
    let mut current_seating = Seating { spaces };
    loop {
        println!(
            "There are {} occupied seats.",
            current_seating.count_occupied()
        );
        let next = current_seating.next();
        if next == current_seating {
            break;
        }
        current_seating = next;
    }
}

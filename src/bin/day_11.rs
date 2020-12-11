use anyhow::{bail, Error, Result};
use aoc_2020::read_entries;
use std::{cmp, str::FromStr};

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[derive(PartialEq, Clone, Copy)]
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

#[derive(PartialEq, Clone)]
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

    fn next(&self, limit: usize, max_distance: Option<usize>) -> Self {
        let mut spaces = vec![vec![Space::Floor; self.width()]; self.height()];
        for i in 0..self.height() {
            for j in 0..self.width() {
                spaces[i][j] = self.next_state(i, j, limit, max_distance);
            }
        }
        Seating { spaces }
    }

    fn next_state(
        &self,
        i: usize,
        j: usize,
        limit: usize,
        max_distance: Option<usize>,
    ) -> Space {
        let occupied_neighbors = self.occupied_neighbors(i, j, max_distance);
        match (self.spaces[i][j], occupied_neighbors) {
            (Space::Occupied, o) if o >= limit => Space::Empty,
            (Space::Empty, 0) => Space::Occupied,
            (space, _) => space,
        }
    }

    fn count_occupied(&self) -> usize {
        self.spaces
            .iter()
            .map(|row| row.iter().filter(|s| s == &&Space::Occupied).count())
            .sum()
    }

    fn occupied_neighbors(
        &self,
        x: usize,
        y: usize,
        max_depth: Option<usize>,
    ) -> usize {
        DIRECTIONS
            .iter()
            .map(|direction| {
                self.first_in_direction(direction, x, y, max_depth)
            })
            .filter(|s| s == &Space::Occupied)
            .count()
    }

    fn first_in_direction(
        &self,
        (dx, dy): &(isize, isize),
        x: usize,
        y: usize,
        max_distance: Option<usize>,
    ) -> Space {
        let (mut pos_x, mut pos_y) = (x as isize, y as isize);
        loop {
            pos_x = pos_x + dx;
            pos_y = pos_y + dy;
            if self.in_bounds(pos_x, pos_y, x, y, max_distance) {
                let space = self.spaces[pos_x as usize][pos_y as usize];
                if space != Space::Floor {
                    return space;
                }
            } else {
                return Space::Floor;
            }
        }
    }

    fn in_bounds(
        &self,
        p_x: isize,
        p_y: isize,
        x: usize,
        y: usize,
        max_distance: Option<usize>,
    ) -> bool {
        let (min_x, max_x, min_y, max_y) = match max_distance {
            Some(i) => (
                cmp::max(0, x as isize - i as isize),
                cmp::min(self.height() - 1, x + i) as isize,
                cmp::max(0, y as isize - i as isize),
                cmp::min(self.width() - 1, y + i) as isize,
            ),
            None => {
                (0, self.height() as isize - 1, 0, self.width() as isize - 1)
            }
        };
        p_x >= min_x && p_y >= min_y && p_x <= max_x && p_y <= max_y
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
    let initial_seating = Seating { spaces };
    let mut current_seating = initial_seating.clone();
    loop {
        let next = current_seating.next(4, Some(1));
        if next == current_seating {
            break;
        }
        current_seating = next;
    }
    println!(
        "There are {} occupied seats.",
        current_seating.count_occupied()
    );

    let mut current_seating = initial_seating.clone();
    loop {
        let next = current_seating.next(5, None);
        if next == current_seating {
            break;
        }
        current_seating = next;
    }
    println!(
        "There are {} occupied seats.",
        current_seating.count_occupied()
    );
}

use anyhow::{bail, Error, Result};
use aoc_2020::read_entries;
use std::str::FromStr;

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

    fn next(&self, limit: usize, visible: &[Space]) -> Self {
        let mut spaces = vec![vec![Space::Floor; self.width()]; self.height()];
        for i in 0..self.height() {
            for j in 0..self.width() {
                spaces[i][j] = self.next_state(i, j, limit, visible);
            }
        }
        Seating { spaces }
    }

    fn next_state(
        &self,
        i: usize,
        j: usize,
        tolerance: usize,
        visible: &[Space],
    ) -> Space {
        let occupied_neighbors = self.occupied_neighbors(i, j, visible);
        match (self.spaces[i][j], occupied_neighbors) {
            (Space::Occupied, o) if o >= tolerance => Space::Empty,
            (Space::Empty, 0) => Space::Occupied,
            (space, _) => space,
        }
    }

    fn occupied_neighbors(
        &self,
        x: usize,
        y: usize,
        visible: &[Space],
    ) -> usize {
        DIRECTIONS
            .iter()
            .map(|direction| self.first_in_direction(direction, x, y, visible))
            .filter(|s| s == &Space::Occupied)
            .count()
    }

    fn first_in_direction(
        &self,
        (dx, dy): &(isize, isize),
        x: usize,
        y: usize,
        visible: &[Space],
    ) -> Space {
        let (mut pos_x, mut pos_y) = (x as isize, y as isize);
        loop {
            pos_x = pos_x + dx;
            pos_y = pos_y + dy;
            if self.in_bounds(pos_x, pos_y) {
                let space = self.spaces[pos_x as usize][pos_y as usize];
                if visible.contains(&space) {
                    return space;
                }
            } else {
                return Space::Floor;
            }
        }
    }

    fn in_bounds(&self, p_x: isize, p_y: isize) -> bool {
        p_x >= 0
            && p_y >= 0
            && p_x <= (self.height() - 1) as isize
            && p_y <= (self.width() - 1) as isize
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
    let initial_seating = Seating { spaces };

    run_simulation(
        &initial_seating,
        4,
        &[Space::Occupied, Space::Floor, Space::Empty],
    );
    run_simulation(&initial_seating, 5, &[Space::Occupied, Space::Empty]);
}

fn run_simulation(seating: &Seating, tolerance: usize, visible: &[Space]) {
    let mut current_seating = seating.clone();
    loop {
        let next = current_seating.next(tolerance, visible);
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

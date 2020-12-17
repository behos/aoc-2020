use aoc_2020::read_entries;
use std::{collections::HashSet, hash::Hash};

trait HasNeighbors: Sized + Clone + Eq + Hash {
    fn neighbors(&self) -> HashSet<Self>;
}

type Cube = (isize, isize, isize);
type Cubes = HashSet<Cube>;

impl HasNeighbors for Cube {
    fn neighbors(&self) -> Cubes {
        let (x, y, z) = self;
        let mut neighbors = Cubes::new();
        for xn in x - 1..=x + 1 {
            for yn in y - 1..=y + 1 {
                for zn in z - 1..=z + 1 {
                    if (xn, yn, zn) != (*x, *y, *z) {
                        neighbors.insert((xn, yn, zn));
                    }
                }
            }
        }
        neighbors
    }
}

type HyperCube = (isize, isize, isize, isize);
type HyperCubes = HashSet<HyperCube>;

impl HasNeighbors for HyperCube {
    fn neighbors(&self) -> HyperCubes {
        let (x, y, z, q) = self;
        let mut neighbors = HyperCubes::new();
        for xn in x - 1..=x + 1 {
            for yn in y - 1..=y + 1 {
                for zn in z - 1..=z + 1 {
                    for qn in q - 1..=q + 1 {
                        if (xn, yn, zn, qn) != (*x, *y, *z, *q) {
                            neighbors.insert((xn, yn, zn, qn));
                        }
                    }
                }
            }
        }
        neighbors
    }
}

fn main() {
    let active_cubes: Cubes = read_entries::<String>("./data/day-17.txt")
        .enumerate()
        .map(|(x, row)| {
            row.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(|(y, _)| (x as isize, y as isize, 1))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect();

    let active_hypercubes: HyperCubes = active_cubes
        .iter()
        .cloned()
        .map(|(x, y, z)| (x, y, z, 1))
        .collect();

    println!("Active cubes: {:?}", run_cycles(active_cubes, 6));
    println!("Active cubes: {:?}", run_cycles(active_hypercubes, 6));
}

fn run_cycles(
    mut active_cubes: HashSet<impl HasNeighbors>,
    cycles: usize,
) -> usize {
    for _ in 0..cycles {
        active_cubes = cycle(active_cubes);
    }
    active_cubes.len()
}

fn cycle<T: HasNeighbors>(active_cubes: HashSet<T>) -> HashSet<T> {
    let remain_active = active_cubes
        .iter()
        .cloned()
        .filter(|c| {
            let active_count = active_neighbors(c, &active_cubes);
            active_count == 2 || active_count == 3
        })
        .collect::<HashSet<_>>();

    let mut activated = active_cubes
        .iter()
        .map(HasNeighbors::neighbors)
        .flatten()
        .filter(|c| {
            !active_cubes.contains(&c)
                && active_neighbors(c, &active_cubes) == 3
        })
        .collect::<HashSet<_>>();

    activated.extend(remain_active);
    activated
}

fn active_neighbors<T: HasNeighbors>(
    cube: &T,
    active_cubes: &HashSet<T>,
) -> usize {
    cube.neighbors().intersection(active_cubes).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let active_cubes =
            [(0, 1, 1), (1, 2, 1), (2, 0, 1), (2, 1, 1), (2, 2, 1)]
                .iter()
                .cloned()
                .collect();
        assert_eq!(112, run_cycles(active_cubes, 6));
    }
}

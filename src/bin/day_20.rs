use anyhow::{Context, Error, Result};
use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
    str::FromStr,
};

#[derive(Clone, Copy)]
enum Position {
    Top,
    Left,
    Right,
    Bottom,
}

struct Tile {
    id: usize,
    grid: Matrix<char>,
}

impl FromStr for Tile {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self> {
        let mut parts = input.split('\n');
        let id = parts
            .next()
            .context("Expected tile metadata.")?
            .strip_prefix("Tile ")
            .context("String did not start with 'Tile'.")?
            .strip_suffix(":")
            .context("String did noe end with ':'")?
            .parse()
            .context("Unparseable id.")?;

        let inner = parts.map(|row| row.chars().collect()).collect();

        Ok(Self {
            id,
            grid: Matrix { inner },
        })
    }
}

impl Tile {
    fn row(&self, position: Position, transform: Transform) -> Vec<char> {
        let max = self.grid.inner.len() - 1;
        let range = 0..=max;
        match position {
            Position::Top => {
                range.map(|i| self.grid.get(0, i, transform)).collect()
            }
            Position::Bottom => range
                .map(|i| self.grid.get(max, i, transform))
                .rev()
                .collect(),
            Position::Left => range
                .map(|i| self.grid.get(i, 0, transform))
                .rev()
                .collect(),
            Position::Right => {
                range.map(|i| self.grid.get(i, max, transform)).collect()
            }
        }
    }

    fn inner(&self, row: usize, col: usize, transform: Transform) -> char {
        self.grid.get(row + 1, col + 1, transform)
    }
}

struct Matrix<T: Copy> {
    inner: Vec<Vec<T>>,
}

impl<T: Copy> Matrix<T> {
    fn get(&self, mut row: usize, mut col: usize, transform: Transform) -> T {
        let max = self.inner.len() - 1;
        for _ in 0..transform.rotation {
            let t = col;
            col = max - row;
            row = t;
        }
        if transform.flipped {
            col = max - col;
        }
        self.inner[row][col]
    }
}

#[derive(Clone, Copy, Debug)]
struct Transform {
    rotation: usize,
    flipped: bool,
}

#[derive(Clone, Copy, Debug)]
struct Placement {
    id: usize,
    transform: Transform,
}

fn main() -> Result<()> {
    let input = read_to_string("./data/day-20.txt")?;
    let grids = input
        .split("\n\n")
        .filter(|s| s != &"")
        .map(|input| {
            let tile = Tile::from_str(input)?;
            Ok((tile.id, tile))
        })
        .collect::<Result<HashMap<usize, Tile>>>()?;

    let mut placements = vec![];
    let mut remaining_ids = grids.keys().copied().collect::<HashSet<_>>();
    if solve(&grids, &mut remaining_ids, &mut placements) {
        println!("Solved!");
    } else {
        println!("Not solved!");
    }

    let corner_product = &[
        placements[0],
        placements[11],
        placements[132],
        placements[143],
    ]
    .iter()
    .map(|p| p.id)
    .product::<usize>();

    println!("Corner products: {}", corner_product);

    let _compiled_image = compile_image(&grids, &placements);

    Ok(())
}

fn solve(
    grids: &HashMap<usize, Tile>,
    mut remaining_ids: &mut HashSet<usize>,
    mut placements: &mut Vec<Placement>,
) -> bool {
    if remaining_ids.len() == 0 {
        return true;
    }
    let candidates = remaining_ids.iter().cloned().collect::<Vec<_>>();
    for id in candidates {
        remaining_ids.remove(&id);
        for rotation in 0..=3 {
            for flipped in &[true, false] {
                let placement = Placement {
                    id,
                    transform: Transform {
                        rotation,
                        flipped: *flipped,
                    },
                };
                if fits(grids, placements, placement) {
                    placements.push(placement);
                    if solve(grids, &mut remaining_ids, &mut placements) {
                        return true;
                    }
                    placements.pop();
                }
            }
        }
        remaining_ids.insert(id);
    }
    return false;
}

fn fits(
    grids: &HashMap<usize, Tile>,
    placements: &[Placement],
    placement: Placement,
) -> bool {
    let tile = &grids[&placement.id];

    if placements.len() % 12 != 0 {
        let left = placements[placements.len() - 1];
        let left_tile = &grids[&left.id];
        let left_row = left_tile.row(Position::Right, left.transform);
        let tile_row = tile
            .row(Position::Left, placement.transform)
            .iter()
            .copied()
            .rev()
            .collect::<Vec<_>>();
        if left_row != tile_row {
            return false;
        }
    }

    if placements.len() / 12 != 0 {
        let top_index =
            ((placements.len() / 12 - 1) * 12) + placements.len() % 12;
        let top = placements[top_index];
        let top_tile = &grids[&top.id];
        let top_row = top_tile.row(Position::Bottom, top.transform);
        let tile_row = tile
            .row(Position::Top, placement.transform)
            .iter()
            .copied()
            .rev()
            .collect::<Vec<_>>();
        if top_row != tile_row {
            return false;
        }
    }
    return true;
}

fn compile_image(
    grids: &HashMap<usize, Tile>,
    placements: &[Placement],
) -> [[char; 96]; 96] {
    let mut compiled_image = [['.'; 96]; 96];
    for x in 0..96 {
        for y in 0..96 {
            let placement = placements[(y / 8 * 12) + x / 8];
            let tile = &grids[&placement.id];
            let rel_x = x % 8;
            let rel_y = y % 8;
            compiled_image[x][y] =
                tile.inner(rel_x, rel_y, placement.transform);
        }
    }
    compiled_image
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flips_and_rotations() {
        let example_tile = r#"Tile 1787:
.#..#....#
#..#.#...#
#........#
.......#..
...#.....#
#.....#..#
#..#....##
#.....#.##
##.......#
....###..#"#
            .parse::<Tile>()
            .unwrap();
        assert_eq!(
            vec!['.', '#', '.', '.', '#', '.', '.', '.', '.', '#'],
            example_tile.row(
                Position::Top,
                Transform {
                    flipped: false,
                    rotation: 0
                }
            )
        );
        assert_eq!(
            vec!['#', '#', '#', '.', '#', '#', '#', '#', '#', '#'],
            example_tile.row(
                Position::Top,
                Transform {
                    flipped: false,
                    rotation: 1
                }
            )
        );
        assert_eq!(
            vec!['.', '#', '.', '.', '#', '.', '.', '.', '.', '#'],
            example_tile.row(
                Position::Bottom,
                Transform {
                    flipped: false,
                    rotation: 2
                }
            )
        );
        assert_eq!(
            vec!['.', '#', '.', '.', '#', '.', '.', '.', '.', '#'],
            example_tile.row(
                Position::Left,
                Transform {
                    flipped: false,
                    rotation: 1
                }
            )
        );

        assert_eq!(
            vec!['.', '.', '.', '.', '#', '#', '#', '.', '.', '#'],
            example_tile.row(
                Position::Bottom,
                Transform {
                    flipped: true,
                    rotation: 0
                }
            )
        );

        assert_eq!(
            vec!['#', '.', '.', '.', '.', '#', '.', '.', '#', '.'],
            example_tile.row(
                Position::Left,
                Transform {
                    flipped: true,
                    rotation: 1
                }
            )
        );
    }

    #[test]
    fn inner_points() {
        let example_tile = r#"Tile 1787:
.#..#....#
#abcdefgh#
#ijklmnop#
.......#..
...#.....#
#.....#..#
#..#....##
#.....#.##
##.......#
....###..#"#
            .parse::<Tile>()
            .unwrap();
        assert_eq!(
            example_tile.inner(
                0,
                0,
                Transform {
                    flipped: false,
                    rotation: 0
                }
            ),
            'a'
        );
        assert_eq!(
            example_tile.inner(
                0,
                2,
                Transform {
                    flipped: false,
                    rotation: 0
                }
            ),
            'c'
        );
        assert_eq!(
            example_tile.inner(
                0,
                6,
                Transform {
                    flipped: false,
                    rotation: 3
                }
            ),
            'i'
        );
        assert_eq!(
            example_tile.inner(
                0,
                6,
                Transform {
                    flipped: true,
                    rotation: 3
                }
            ),
            'p'
        );
    }
}

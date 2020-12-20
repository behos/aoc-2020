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

impl Position {
    fn rotate(&self) -> Self {
        match self {
            Position::Top => Position::Left,
            Position::Left => Position::Bottom,
            Position::Bottom => Position::Right,
            Position::Right => Position::Top,
        }
    }

    fn flip(&self) -> Self {
        match self {
            Position::Right => Position::Left,
            Position::Left => Position::Right,
            pos => *pos,
        }
    }
}

struct Tile {
    id: usize,
    grid: [[char; 10]; 10],
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

        let mut grid = [['.'; 10]; 10];

        parts.enumerate().for_each(|(x, row)| {
            row.chars().enumerate().for_each(|(y, c)| grid[x][y] = c)
        });

        Ok(Self { id, grid })
    }
}

impl Tile {
    fn row(&self, mut position: Position, placement: Placement) -> Vec<char> {
        for _ in 0..placement.rotation {
            position = position.rotate();
        }
        if placement.flipped {
            position = position.flip();
        }

        let mut row = match position {
            Position::Top => self.grid[0].to_vec(),
            Position::Bottom => self.grid[9].iter().copied().rev().collect(),
            Position::Left => {
                self.grid.iter().map(|r| r[0]).rev().collect::<Vec<_>>()
            }
            Position::Right => {
                self.grid.iter().map(|r| r[9]).collect::<Vec<_>>()
            }
        };

        if placement.flipped {
            row = row.iter().copied().rev().collect()
        }
        row
    }
}

#[derive(Clone, Copy, Debug)]
struct Placement {
    id: usize,
    rotation: usize,
    flipped: bool,
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
                    rotation,
                    flipped: *flipped,
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
        let left_row = left_tile.row(Position::Right, left);
        let tile_row = tile
            .row(Position::Left, placement)
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
        let top_row = top_tile.row(Position::Bottom, top);
        let tile_row = tile
            .row(Position::Top, placement)
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
                Placement {
                    flipped: false,
                    id: 1787,
                    rotation: 0
                }
            )
        );
        assert_eq!(
            vec!['.', '#', '.', '.', '#', '.', '.', '.', '.', '#'],
            example_tile.row(
                Position::Right,
                Placement {
                    flipped: false,
                    id: 1787,
                    rotation: 1
                }
            )
        );
        assert_eq!(
            vec!['.', '#', '.', '.', '#', '.', '.', '.', '.', '#'],
            example_tile.row(
                Position::Bottom,
                Placement {
                    flipped: false,
                    id: 1787,
                    rotation: 2
                }
            )
        );
        assert_eq!(
            vec!['#', '.', '.', '#', '#', '#', '.', '.', '.', '.'],
            example_tile.row(
                Position::Left,
                Placement {
                    flipped: false,
                    id: 1787,
                    rotation: 1
                }
            )
        );

        assert_eq!(
            vec!['.', '.', '.', '.', '#', '#', '#', '.', '.', '#'],
            example_tile.row(
                Position::Bottom,
                Placement {
                    flipped: true,
                    id: 1787,
                    rotation: 0
                }
            )
        );

        assert_eq!(
            vec!['.', '.', '.', '.', '#', '#', '#', '.', '.', '#'],
            example_tile.row(
                Position::Left,
                Placement {
                    flipped: true,
                    id: 1787,
                    rotation: 1
                }
            )
        );
    }
}

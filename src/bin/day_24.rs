use anyhow::{bail, Result};
use aoc_2020::read_entries;
use std::collections::HashSet;

enum Direction {
    East,
    NorthEast,
    NorthWest,
    West,
    SouthEast,
    SouthWest,
}

impl Direction {
    fn parse(input: String) -> Result<Vec<Direction>> {
        let mut directions = vec![];
        let mut carry = None;
        for c in input.chars() {
            if c == 'n' || c == 's' {
                carry = Some(c)
            } else {
                directions.push(match (carry, c) {
                    (None, 'e') => Direction::East,
                    (None, 'w') => Direction::West,
                    (Some('n'), 'e') => Direction::NorthEast,
                    (Some('n'), 'w') => Direction::NorthWest,
                    (Some('s'), 'e') => Direction::SouthEast,
                    (Some('s'), 'w') => Direction::SouthWest,
                    _ => bail!("Got me some weird results here"),
                });
                carry = None;
            }
        }
        Ok(directions)
    }
}

type Point = (isize, isize, isize);

fn main() -> Result<()> {
    let directions_set = read_entries::<String>("./data/day-24.txt")
        .map(Direction::parse)
        .collect::<Result<Vec<_>>>()?;

    let mut black_tiles = HashSet::new();
    for directions in directions_set {
        let point = follow_directions(&directions);
        if black_tiles.contains(&point) {
            black_tiles.remove(&point);
        } else {
            black_tiles.insert(point);
        }
    }

    println!("There are {} black tiles", black_tiles.len());

    for _ in 0..100 {
        black_tiles = advance(black_tiles)
    }
    println!(
        "After 100 days, there are {} black tiles",
        black_tiles.len()
    );
    Ok(())
}

fn follow_directions(directions: &[Direction]) -> Point {
    directions.iter().fold((0, 0, 0), point_move)
}

fn point_move((x, y, z): Point, direction: &Direction) -> Point {
    // https://www.redblobgames.com/grids/hexagons/#coordinates-cube
    // Using the cube coordinate system from here.
    match direction {
        Direction::East => (x + 1, y - 1, z),
        Direction::West => (x - 1, y + 1, z),
        Direction::NorthWest => (x, y + 1, z - 1),
        Direction::NorthEast => (x + 1, y, z - 1),
        Direction::SouthWest => (x - 1, y, z + 1),
        Direction::SouthEast => (x, y - 1, z + 1),
    }
}

fn neighbors(point: &Point) -> HashSet<Point> {
    let mut set = HashSet::new();
    set.insert(point_move(*point, &Direction::East));
    set.insert(point_move(*point, &Direction::West));
    set.insert(point_move(*point, &Direction::NorthWest));
    set.insert(point_move(*point, &Direction::NorthEast));
    set.insert(point_move(*point, &Direction::SouthWest));
    set.insert(point_move(*point, &Direction::SouthEast));
    set
}

fn advance(black_tiles: HashSet<Point>) -> HashSet<Point> {
    let all_neighbors = black_tiles
        .iter()
        .map(neighbors)
        .flatten()
        .collect::<HashSet<Point>>();

    let white_tile_neighbors = all_neighbors
        .difference(&black_tiles)
        .collect::<HashSet<&Point>>();

    let mut new_black_tiles = HashSet::new();

    for tile in &black_tiles {
        let black_tile_neighbors =
            neighbors(&tile).intersection(&black_tiles).count();
        if black_tile_neighbors == 1 || black_tile_neighbors == 2 {
            new_black_tiles.insert(*tile);
        }
    }

    for tile in white_tile_neighbors {
        let black_tile_neighbors =
            neighbors(&tile).intersection(&black_tiles).count();
        if black_tile_neighbors == 2 {
            new_black_tiles.insert(*tile);
        }
    }
    new_black_tiles
}

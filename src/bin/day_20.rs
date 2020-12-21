use anyhow::{Context, Error, Result};
use std::{
    collections::{HashMap, HashSet},
    fmt::{self, Display},
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
            Position::Bottom => {
                range.map(|i| self.grid.get(max, i, transform)).collect()
            }
            Position::Left => {
                range.map(|i| self.grid.get(i, 0, transform)).collect()
            }
            Position::Right => {
                range.map(|i| self.grid.get(i, max, transform)).collect()
            }
        }
    }

    fn inner(&self, row: usize, col: usize, transform: Transform) -> char {
        self.grid.get(row + 1, col + 1, transform)
    }
}

#[derive(Debug)]
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

impl Display for Matrix<char> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.inner
                .iter()
                .map(|row| row.iter().collect::<String>())
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[derive(Clone, Copy, Debug)]
struct Transform {
    rotation: usize,
    flipped: bool,
}

const ALL_TRANSFORMATIONS: [Transform; 8] = [
    Transform {
        rotation: 0,
        flipped: false,
    },
    Transform {
        rotation: 1,
        flipped: false,
    },
    Transform {
        rotation: 2,
        flipped: false,
    },
    Transform {
        rotation: 3,
        flipped: false,
    },
    Transform {
        rotation: 0,
        flipped: true,
    },
    Transform {
        rotation: 1,
        flipped: true,
    },
    Transform {
        rotation: 2,
        flipped: true,
    },
    Transform {
        rotation: 3,
        flipped: true,
    },
];

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
    solve(&grids, &mut remaining_ids, &mut placements);

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

    let image = compile_image(&grids, &placements);
    let monsters = find_monsters(&image);
    let total_roughness = image
        .inner
        .iter()
        .map(|row| row.iter().filter(|c| c == &&'#').count())
        .sum::<usize>();
    println!("Found {} monsters.", monsters);
    println!("Water roughness total {}.", total_roughness);
    println!("Water roughness {}.", total_roughness - monsters * 15);
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
    let mut candidates = remaining_ids.iter().cloned().collect::<Vec<_>>();
    candidates.sort();
    for id in candidates {
        remaining_ids.remove(&id);
        for &transform in &ALL_TRANSFORMATIONS {
            let placement = Placement { id, transform };
            if fits(grids, placements, placement) {
                placements.push(placement);
                if solve(grids, &mut remaining_ids, &mut placements) {
                    return true;
                }
                placements.pop();
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
        let tile_row = tile.row(Position::Left, placement.transform);
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
        let tile_row = tile.row(Position::Top, placement.transform);
        if top_row != tile_row {
            return false;
        }
    }
    return true;
}

fn compile_image(
    grids: &HashMap<usize, Tile>,
    placements: &[Placement],
) -> Matrix<char> {
    let inner = (0..96)
        .map(|row| {
            (0..96)
                .map(|col| {
                    let index = row / 8 * 12 + col / 8;
                    let placement = placements[index];
                    let tile = &grids[&placement.id];
                    let rel_row = row % 8;
                    let rel_col = col % 8;
                    tile.inner(rel_row, rel_col, placement.transform)
                })
                .collect()
        })
        .collect();
    Matrix { inner }
}

fn find_monsters(image: &Matrix<char>) -> usize {
    ALL_TRANSFORMATIONS
        .iter()
        .map(|&transform| find_monsters_with_transform(image, transform))
        .max()
        .expect("There's at least one number.")
}

fn find_monsters_with_transform(
    image: &Matrix<char>,
    transform: Transform,
) -> usize {
    //                   #
    // #    ##    ##    ###
    //  #  #  #  #  #  #
    let monster_shape = [
        (0, 18),
        (1, 0),
        (1, 5),
        (1, 6),
        (1, 11),
        (1, 12),
        (1, 17),
        (1, 18),
        (1, 19),
        (2, 1),
        (2, 4),
        (2, 7),
        (2, 10),
        (2, 13),
        (2, 16),
    ];
    let len = image.inner.len();
    (0..len)
        .map(|row| {
            (0..len)
                .filter(|col| {
                    monster_shape.iter().all(|(offset_row, offset_col)| {
                        let check_row = row + offset_row;
                        let check_col = col + offset_col;
                        check_row < len
                            && check_col < len
                            && image.get(check_row, check_col, transform) == '#'
                    })
                })
                .count()
        })
        .sum()
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
            vec!['#', '.', '.', '.', '.', '#', '.', '.', '#', '.'],
            example_tile.row(
                Position::Bottom,
                Transform {
                    flipped: false,
                    rotation: 2
                }
            )
        );
        assert_eq!(
            vec!['#', '.', '.', '.', '.', '#', '.', '.', '#', '.'],
            example_tile.row(
                Position::Left,
                Transform {
                    flipped: false,
                    rotation: 1
                }
            )
        );

        assert_eq!(
            vec!['#', '.', '.', '#', '#', '#', '.', '.', '.', '.'],
            example_tile.row(
                Position::Bottom,
                Transform {
                    flipped: true,
                    rotation: 0
                }
            )
        );

        assert_eq!(
            vec!['.', '#', '.', '.', '#', '.', '.', '.', '.', '#'],
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
                1,
                2,
                Transform {
                    flipped: false,
                    rotation: 0
                }
            ),
            'k'
        );
        assert_eq!(
            example_tile.inner(
                6,
                7,
                Transform {
                    flipped: false,
                    rotation: 2
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

    #[test]
    fn transformations() {
        let matrix = Matrix {
            inner: vec![
                vec!['a', 'b', 'c'],
                vec!['d', 'e', 'f'],
                vec!['g', 'h', 'i'],
            ],
        };

        let t = Transform {
            flipped: false,
            rotation: 1,
        };

        assert_eq!(
            (0..3)
                .map(|r| (0..3)
                    .map(|c| matrix.get(r, c, t))
                    .collect::<Vec<_>>())
                .collect::<Vec<_>>(),
            vec![
                vec!['c', 'f', 'i'],
                vec!['b', 'e', 'h'],
                vec!['a', 'd', 'g']
            ]
        )
    }

    #[test]
    fn monster_detection() {
        let image = r#"
.#.#..#.##...#.##..#####
###....#.#....#..#......
##.##.###.#.#..######...
###.#####...#.#####.#..#
##.#....#.##.####...#.##
...########.#....#####.#
....#..#...##..#.#.###..
.####...#..#.....#......
#..#.##..#..###.#.##....
#.####..#.####.#.#.###..
###.#.#...#.######.#..##
#.####....##..########.#
##..##.#...#...#.#.#.#..
...#..#..#.#.##..###.###
.#.#....#.##.#...###.##.
###.#...#..#.##.######..
.#.#.###.##.##.#..#.##..
.####.###.#...###.#..#.#
..#.#..#..#.#.#.####.###
#..####...#.#.#.###.###.
#####..#####...###....##
#.##..#..#...#..####...#
.#.###..##..##..####.##.
...###...##...#...#..###
"#
        .trim()
        .split("\n")
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
        let matrix = Matrix { inner: image };
        assert_eq!(2, find_monsters(&matrix));
        assert_eq!(
            273,
            matrix
                .inner
                .iter()
                .map(|row| row.iter().filter(|c| c == &&'#').count())
                .sum::<usize>()
                - 30
        );
    }
}

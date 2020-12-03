use anyhow::{bail, Error, Result};
use aoc_2020::read_entries;
use std::str::FromStr;

/// One would argue defining this is unnecessary, but what is rust without
/// cool types?
#[derive(Debug, PartialEq)]
enum Cell {
    Tree,
    Free,
}

impl Cell {
    fn from_char(input: char) -> Result<Self> {
        match input {
            '.' => Ok(Cell::Free),
            '#' => Ok(Cell::Tree),
            e => bail!("Unknown entry {}", e),
        }
    }
}

struct Forest {
    inner: Vec<Vec<Cell>>,
}

impl Forest {
    fn get(&self, x: usize, y: usize) -> &Cell {
        let row = &self.inner[y];
        &row[x % row.len()]
    }

    fn height(&self) -> usize {
        self.inner.len()
    }
}

struct Entry(Vec<Cell>);

impl FromStr for Entry {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self> {
        Ok(Entry(
            input
                .chars()
                .map(Cell::from_char)
                .collect::<Result<Vec<Cell>>>()?,
        ))
    }
}

fn count_trees(x: usize, y: usize, forest: &Forest) -> usize {
    (0..forest.height())
        .filter(|row| {
            row % y == 0 && forest.get(x * row / y, *row) == &Cell::Tree
        })
        .count()
}

fn main() {
    let map = Forest {
        inner: read_entries::<Entry>("./data/day-03.txt")
            .map(|Entry(e)| e)
            .collect(),
    };

    let trees = count_trees(3, 1, &map);
    println!("Bumped into {} trees in my testing.", trees);

    let product: usize = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|(x, y)| count_trees(*x, *y, &map))
        .product();

    println!("Bumped into a lot of trees with final product {}", product);
}

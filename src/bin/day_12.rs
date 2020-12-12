use anyhow::{bail, Context, Error, Result};
use aoc_2020::read_entries;
use std::str::FromStr;

#[derive(Clone, Copy)]
enum Relative {
    Left,
    Right,
}

impl Relative {
    fn relative_offset(&self, (x, y): (isize, isize)) -> (isize, isize) {
        match self {
            Relative::Left => (-y, x),
            Relative::Right => (y, -x),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Compass {
    North,
    South,
    East,
    West,
}

impl Compass {
    fn relative_to(&self, relative: &Relative) -> Compass {
        match (relative, self) {
            (Relative::Left, Compass::North) => Compass::West,
            (Relative::Left, Compass::West) => Compass::South,
            (Relative::Left, Compass::South) => Compass::East,
            (Relative::Left, Compass::East) => Compass::North,
            (Relative::Right, Compass::North) => Compass::East,
            (Relative::Right, Compass::East) => Compass::South,
            (Relative::Right, Compass::South) => Compass::West,
            (Relative::Right, Compass::West) => Compass::North,
        }
    }

    fn offset(&self) -> (isize, isize) {
        match self {
            Compass::North => (0, 1),
            Compass::South => (0, -1),
            Compass::East => (1, 0),
            Compass::West => (-1, 0),
        }
    }
}

enum Direction {
    Absolute(Compass),
    Relative(Relative),
    Forward,
}

struct Instruction {
    direction: Direction,
    amount: usize,
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self> {
        let direction =
            match input.chars().next().context("Must start with a char")? {
                'N' => Direction::Absolute(Compass::North),
                'S' => Direction::Absolute(Compass::South),
                'E' => Direction::Absolute(Compass::East),
                'W' => Direction::Absolute(Compass::West),
                'L' => Direction::Relative(Relative::Left),
                'R' => Direction::Relative(Relative::Right),
                'F' => Direction::Forward,
                hm => bail!("What is {}?", hm),
            };
        let amount = input[1..]
            .parse::<usize>()
            .context("Failed to parse amount")?;
        Ok(Self { direction, amount })
    }
}

#[derive(Debug)]
struct Ship {
    position: (isize, isize),
    facing_direction: Compass,
}

impl Ship {
    fn new() -> Self {
        Self {
            position: (0, 0),
            facing_direction: Compass::East,
        }
    }

    fn execute(&mut self, instruction: &Instruction) {
        let amount = instruction.amount;
        match &instruction.direction {
            Direction::Absolute(compass) => self.move_ship(*compass, amount),
            Direction::Forward => self.move_ship(self.facing_direction, amount),
            Direction::Relative(direction) => self.turn(direction, amount),
        }
    }

    fn move_ship(&mut self, compass: Compass, amount: usize) {
        self.position = move_point(self.position, compass.offset(), amount);
    }

    fn turn(&mut self, direction: &Relative, mut amount: usize) {
        while amount > 0 {
            self.facing_direction =
                self.facing_direction.relative_to(&direction);
            amount -= 90;
        }
    }
}

struct ShipWithWaypoint {
    waypoint: (isize, isize),
    position: (isize, isize),
}

impl ShipWithWaypoint {
    fn new() -> Self {
        Self {
            position: (0, 0),
            waypoint: (10, 1),
        }
    }

    fn execute(&mut self, instruction: &Instruction) {
        let amount = instruction.amount;
        match &instruction.direction {
            Direction::Absolute(compass) => {
                self.move_waypoint(*compass, amount)
            }
            Direction::Forward => self.move_ship(amount),
            Direction::Relative(direction) => {
                self.rotate_waypoint(*direction, amount)
            }
        }
    }

    fn move_waypoint(&mut self, compass: Compass, amount: usize) {
        self.waypoint = move_point(self.waypoint, compass.offset(), amount);
    }

    fn rotate_waypoint(&mut self, direction: Relative, mut amount: usize) {
        while amount > 0 {
            self.waypoint = direction.relative_offset(self.waypoint);
            amount -= 90;
        }
    }

    fn move_ship(&mut self, times: usize) {
        self.position = move_point(self.position, self.waypoint, times)
    }
}

fn move_point(
    (x, y): (isize, isize),
    (dx, dy): (isize, isize),
    multiplier: usize,
) -> (isize, isize) {
    (
        x + (dx * multiplier as isize),
        y + (dy * multiplier as isize),
    )
}

fn manhattan_distance((x, y): (isize, isize)) -> isize {
    x.abs() + y.abs()
}

fn main() {
    let instructions: Vec<_> =
        read_entries::<Instruction>("./data/day-12.txt").collect();

    let mut ship = Ship::new();
    let mut ship_waypoint = ShipWithWaypoint::new();
    for instruction in &instructions {
        ship.execute(instruction);
        ship_waypoint.execute(instruction);
    }
    println!("Distance {:?}", manhattan_distance(ship.position));
    println!("Distance {:?}", manhattan_distance(ship_waypoint.position));
}

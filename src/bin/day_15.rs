use std::collections::HashMap;

struct State {
    latest: usize,
    prev: usize,
}

impl State {
    fn new(turn: usize) -> Self {
        Self {
            latest: turn,
            prev: turn,
        }
    }

    fn updated(&self, turn: usize) -> Self {
        Self {
            latest: turn,
            prev: self.latest,
        }
    }
}

struct Game {
    numbers: HashMap<usize, State>,
    turn: usize,
    last: usize,
}

impl Game {
    fn new() -> Self {
        Self {
            numbers: HashMap::new(),
            turn: 0,
            last: 0,
        }
    }

    fn feed(&mut self, num: usize) -> usize {
        self.turn += 1;
        self.last = num;
        let entry = self.numbers.entry(num).or_insert(State::new(self.turn));
        *entry = entry.updated(self.turn);
        entry.latest - entry.prev
    }
}

fn run_game(starting_nums: &[usize], ending_turn: usize) -> usize {
    let mut game = Game::new();
    let mut next = 0;
    for num in starting_nums {
        next = game.feed(*num);
    }
    while game.turn < ending_turn {
        next = game.feed(next);
    }
    game.last
}

fn main() {
    let starting_nums = [7, 14, 0, 17, 11, 1, 2];
    println!("Number is {}", run_game(&starting_nums, 2020));
    println!("Number is {}", run_game(&starting_nums, 30_000_000));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(1, run_game(&[1, 3, 2], 2020));
        assert_eq!(10, run_game(&[2, 1, 3], 2020));
    }
}

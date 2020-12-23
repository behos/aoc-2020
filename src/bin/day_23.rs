use std::collections::HashMap;

#[derive(Debug)]
struct Cup {
    label: usize,
    next: usize,
}

impl Cup {
    fn new(label: usize, next: usize) -> Self {
        Self { label, next }
    }
}

struct Circle {
    cups: Vec<Cup>,
    current: usize,
    label_index: HashMap<usize, usize>,
}

impl Circle {
    fn new(cups: Vec<Cup>) -> Self {
        let label_index = cups
            .iter()
            .enumerate()
            .map(|(i, cup)| (cup.label, i))
            .collect();
        Self {
            cups,
            current: 0,
            label_index,
        }
    }

    fn rotate(&mut self) {
        self.current = self.next(self.current)
    }

    fn rotate_to_label(&mut self, label: usize) {
        self.current = self.label_index[&label];
    }

    fn next(&self, index: usize) -> usize {
        self.cups[index].next
    }

    fn label(&self, index: usize) -> usize {
        self.cups[index].label
    }

    fn crab_move(&mut self) {
        let removed = self.next(self.current);
        self.cups[self.current].next = self.next(self.next(self.next(removed)));
        let picked_labels = [
            self.label(removed),
            self.label(self.next(removed)),
            self.label(self.next(self.next(removed))),
        ];
        let mut label = self.label(self.current);
        while picked_labels.contains(&label)
            || label == self.label(self.current)
        {
            label -= 1;
            if label == 0 {
                label = self.cups.len()
            }
        }

        let target = self.label_index[&label];
        let i = self.next(self.next(removed));
        self.cups[i].next = self.next(target);
        self.cups[target].next = removed;
        self.rotate();
    }
}

fn main() {
    let cups = vec![
        Cup::new(5, 1),
        Cup::new(8, 2),
        Cup::new(6, 3),
        Cup::new(4, 4),
        Cup::new(3, 5),
        Cup::new(9, 6),
        Cup::new(1, 7),
        Cup::new(7, 8),
        Cup::new(2, 0),
    ];
    let mut circle = Circle::new(cups);

    for _ in 0..100 {
        circle.crab_move()
    }
    circle.rotate_to_label(1);
    circle.rotate();
    let mut result = vec![];
    while circle.label(circle.current) != 1 {
        result.push(format!("{}", circle.label(circle.current)));
        circle.rotate()
    }
    println!("Result after end {}", result.drain(..).collect::<String>());

    let mut more_cups = vec![
        Cup::new(5, 1),
        Cup::new(8, 2),
        Cup::new(6, 3),
        Cup::new(4, 4),
        Cup::new(3, 5),
        Cup::new(9, 6),
        Cup::new(1, 7),
        Cup::new(7, 8),
        Cup::new(2, 9),
    ];
    more_cups.extend(
        (10..=1_000_000)
            .map(|label| Cup::new(label, label))
            .collect::<Vec<_>>(),
    );
    more_cups[999_999].next = 0;

    let mut circle = Circle::new(more_cups);
    for _ in 0..10_000_000 {
        circle.crab_move()
    }
    circle.rotate_to_label(1);
    circle.rotate();
    let first = circle.label(circle.current);
    circle.rotate();
    let second = circle.label(circle.current);
    println!(
        "Stars should be under {} * {} = {}",
        first,
        second,
        first * second
    );
}

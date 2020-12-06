use aoc_2020::read_entries;
use std::collections::HashSet;

fn main() {
    let mut groups = vec![vec![]];
    for entry in read_entries::<String>("./data/day-06.txt") {
        if entry == "" {
            groups.push(vec![]);
        } else {
            groups
                .last_mut()
                .expect("There's always a last.")
                .push(entry.chars().collect::<HashSet<_>>());
        }
    }

    let (unions, intersections): (Vec<usize>, Vec<usize>) =
        groups.iter().map(count_union_and_intersect).unzip();
    println!("Sum of unions {}", unions.iter().sum::<usize>());
    println!("Sum of intersects {}", intersections.iter().sum::<usize>());
}

fn count_union_and_intersect(group: &Vec<HashSet<char>>) -> (usize, usize) {
    let (group_union, group_intersection) = group.iter().skip(1).fold(
        (group[0].clone(), group[0].clone()),
        |(union, intersection), set| {
            (
                union.union(&set).cloned().collect(),
                intersection.intersection(&set).cloned().collect(),
            )
        },
    );
    (group_union.len(), group_intersection.len())
}

use anyhow::{Context, Result};
use std::{
    collections::{hash_map::DefaultHasher, HashSet, VecDeque},
    fs::read_to_string,
    hash::{Hash, Hasher},
};

fn main() -> Result<()> {
    let input = read_to_string("./data/day-22.txt")?;
    let decks = input
        .trim()
        .split("\n\n")
        .map(|deck| {
            deck.trim()
                .split("\n")
                .skip(1)
                .map(|s| {
                    s.parse::<usize>().context("Failed to parse input {}.")
                })
                .collect::<Result<VecDeque<_>>>()
        })
        .collect::<Result<Vec<_>>>()?;
    let score = lets_play(decks[0].clone(), decks[1].clone());
    println!("Winning score is {}", score);
    let (_, score) = lets_play_recursive(decks[0].clone(), decks[1].clone());
    println!("Winning score in recursive is {}", score);
    Ok(())
}

fn lets_play(mut p1: VecDeque<usize>, mut p2: VecDeque<usize>) -> usize {
    while p1.len() > 0 && p2.len() > 0 {
        let (c1, c2) = (p1.pop_front().unwrap(), p2.pop_front().unwrap());
        if c1 > c2 {
            p1.push_back(c1);
            p1.push_back(c2);
        } else {
            p2.push_back(c2);
            p2.push_back(c1);
        }
    }
    let p1_score = get_score(&p1);
    let p2_score = get_score(&p2);
    return usize::max(p1_score, p2_score);
}

fn get_score(deck: &VecDeque<usize>) -> usize {
    deck.iter()
        .rev()
        .enumerate()
        .map(|(i, val)| (i + 1) * val)
        .sum()
}

#[derive(PartialEq)]
enum Player {
    P1,
    P2,
}

fn lets_play_recursive(
    mut p1: VecDeque<usize>,
    mut p2: VecDeque<usize>,
) -> (Player, usize) {
    let mut hashes = HashSet::new();

    while p1.len() > 0 && p2.len() > 0 {
        let hash = get_hash(&p1, &p2);

        if hashes.contains(&hash) {
            return (Player::P1, get_score(&p1));
        }

        hashes.insert(hash);

        let (c1, c2) = (p1.pop_front().unwrap(), p2.pop_front().unwrap());

        let winner = if p1.len() >= c1 && p2.len() >= c2 {
            let mut sub_p1 = p1.clone();
            sub_p1.resize(c1, 0);
            let mut sub_p2 = p2.clone();
            sub_p2.resize(c2, 0);
            lets_play_recursive(sub_p1, sub_p2).0
        } else if c1 > c2 {
            Player::P1
        } else {
            Player::P2
        };

        if winner == Player::P1 {
            p1.push_back(c1);
            p1.push_back(c2);
        } else {
            p2.push_back(c2);
            p2.push_back(c1);
        }
    }
    return if p1.len() > 0 {
        (Player::P1, get_score(&p1))
    } else {
        (Player::P2, get_score(&p2))
    };
}

fn get_hash(deck_a: &VecDeque<usize>, deck_b: &VecDeque<usize>) -> u64 {
    let mut hasher = DefaultHasher::new();
    deck_a.hash(&mut hasher);
    deck_b.hash(&mut hasher);
    hasher.finish()
}

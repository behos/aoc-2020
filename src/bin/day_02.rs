use anyhow::{Context, Error, Result};
use aoc_2020::read_entries;
use std::str::FromStr;

struct Policy {
    low: usize,
    high: usize,
    character: char,
}

impl FromStr for Policy {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self> {
        let mut split = input.split(" ");
        let range = split.next().context("Missing range.")?;
        let mut range_split = range.split("-");
        let low = range_split
            .next()
            .context("Missing low.")?
            .parse::<usize>()
            .context("Unparseable low")?;
        let high = range_split
            .next()
            .context("Missing high.")?
            .parse::<usize>()
            .context("Unparseable high")?;
        let character = split
            .next()
            .context("Missing char.")?
            .parse::<char>()
            .context("Unparseable char")?;
        Ok(Self {
            low,
            high,
            character,
        })
    }
}

struct Entry {
    policy: Policy,
    password: String,
}

impl FromStr for Entry {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self> {
        let mut split = input.split(": ");
        let policy =
            split.next().context("Missing policy.")?.parse::<Policy>()?;
        let password = split.next().context("Missing password.")?.to_string();
        Ok(Self { policy, password })
    }
}

impl Entry {
    fn is_compliant(&self) -> bool {
        let matches = self.password.matches(self.policy.character).count();
        self.policy.low <= matches && matches <= self.policy.high
    }

    fn is_compliant_new(&self) -> bool {
        let chars: Vec<_> = self.password.chars().collect();
        (chars[self.policy.low - 1] == self.policy.character)
            ^ (chars[self.policy.high - 1] == self.policy.character)
    }
}

fn main() {
    let (compliant, compliant_new) = read_entries::<Entry>("./data/day-02.txt")
        .fold((0, 0), |(c1, c2), e| {
            (
                c1 + if e.is_compliant() { 1 } else { 0 },
                c2 + if e.is_compliant_new() { 1 } else { 0 },
            )
        });
    println!("Found {} compliant.", compliant);
    println!("Found {} compliant new.", compliant_new);
}

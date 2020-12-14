use anyhow::{bail, Context, Error, Result};
use aoc_2020::read_entries;
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::HashMap,
    str::{self, FromStr},
    u64,
};

lazy_static! {
    static ref MEM_REGEX: Regex =
        Regex::new("^mem\\[(?P<address>\\d+)\\] = (?P<value>\\d+)$").unwrap();
}

struct Program {
    mask: Vec<(usize, u8)>,
    memory: HashMap<usize, u64>,
}

impl Program {
    fn new() -> Self {
        Self {
            mask: vec![],
            memory: HashMap::new(),
        }
    }

    fn execute(&mut self, command: Command) -> Result<()> {
        match command {
            Command::Mask(mask) => Ok(self.update_mask(&mask)),
            Command::Mem(address, value) => self.set_mem(address, value),
        }
    }

    fn update_mask(&mut self, mask_str: &str) {
        self.mask = mask_str
            .as_bytes()
            .iter()
            .cloned()
            .enumerate()
            .filter(|(_, c)| c != &b'X')
            .map(|(i, c)| (i + 64 - 36, c)) // Offset for 64 bits
            .collect();
    }

    fn set_mem(&mut self, address: usize, val: u64) -> Result<()> {
        let mut bin = format!("{:064b}", val).into_bytes();
        for (offset, val) in &self.mask {
            bin[*offset] = *val;
        }
        let entry = self.memory.entry(address).or_insert(val);
        *entry = u64::from_str_radix(
            str::from_utf8(&bin).context("Failed to parse what I created.")?,
            2,
        )
        .context("Unparseable binary.")?;
        Ok(())
    }

    fn mem_sum(&self) -> u64 {
        self.memory.values().sum()
    }
}

enum Command {
    Mask(String),
    Mem(usize, u64),
}

impl FromStr for Command {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self> {
        let mut parts = input.split(" = ");
        Ok(match parts.next() {
            Some("mask") => Command::Mask(
                parts.next().context("Missing mask string.")?.to_string(),
            ),
            Some(memory) if memory.starts_with("mem") => {
                match MEM_REGEX.captures(input) {
                    Some(captures) => Command::Mem(
                        captures
                            .name("address")
                            .context("Missing address.")?
                            .as_str()
                            .parse()
                            .context("Unparseable address")?,
                        captures
                            .name("value")
                            .context("Missing value.")?
                            .as_str()
                            .parse()
                            .context("Unparseable value.")?,
                    ),
                    None => bail!("Unparseable mem command."),
                }
            }
            _ => bail!("Missing command part."),
        })
    }
}

fn main() -> Result<()> {
    let mut program = Program::new();
    for command in read_entries::<Command>("./data/day-14.txt") {
        program.execute(command)?;
    }
    println!("Mem sum = {}", program.mem_sum());
    Ok(())
}

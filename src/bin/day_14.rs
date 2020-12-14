use anyhow::{bail, Context, Error, Result};
use aoc_2020::read_entries;
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::HashMap,
    str::{self, FromStr},
    usize,
};

lazy_static! {
    static ref MEM_REGEX: Regex =
        Regex::new("^mem\\[(?P<address>\\d+)\\] = (?P<value>\\d+)$").unwrap();
}

struct ProgramV1 {
    mask: Vec<(usize, u8)>,
    memory: HashMap<usize, usize>,
}

impl ProgramV1 {
    fn new() -> Self {
        ProgramV1 {
            mask: vec![],
            memory: HashMap::new(),
        }
    }

    fn execute(&mut self, command: &Command) -> Result<()> {
        match command {
            Command::Mask(mask) => Ok(self.update_mask(&mask)),
            Command::Mem(address, value) => self.set_mem(*address, *value),
        }
    }

    fn update_mask(&mut self, mask_str: &str) {
        self.mask = mask_str
            .as_bytes()
            .iter()
            .cloned()
            .enumerate()
            .filter(|(_, c)| c != &b'X')
            .collect();
    }

    fn set_mem(&mut self, address: usize, val: usize) -> Result<()> {
        let mask_val = self.apply_mask(val)?;
        self.memory.insert(address, mask_val);
        Ok(())
    }

    fn apply_mask(&self, val: usize) -> Result<usize> {
        let mut bin = format!("{:036b}", val).into_bytes();
        for (offset, val) in &self.mask {
            bin[*offset] = *val;
        }
        usize::from_str_radix(
            str::from_utf8(&bin).context("Failed to parse what I created.")?,
            2,
        )
        .context("Unparseable binary.")
    }

    fn mem_sum(&self) -> usize {
        self.memory.values().sum()
    }
}

struct ProgramV2 {
    mask: Vec<u8>,
    memory: HashMap<usize, usize>,
}

impl ProgramV2 {
    fn new() -> Self {
        ProgramV2 {
            mask: vec![],
            memory: HashMap::new(),
        }
    }

    fn execute(&mut self, command: &Command) -> Result<()> {
        match command {
            Command::Mask(mask) => self.mask = mask.clone().into_bytes(),
            Command::Mem(address, value) => {
                self.set_mem_range(*address, *value)?
            }
        }
        Ok(())
    }

    fn set_mem_range(&mut self, address: usize, val: usize) -> Result<()> {
        for a in self.addresses(address)? {
            self.memory.insert(a, val);
        }
        Ok(())
    }

    fn addresses(&self, address: usize) -> Result<Vec<usize>> {
        let mut address_bin = format!("{:036b}", address).into_bytes();
        for (i, byte) in self.mask.iter().enumerate() {
            match byte {
                b'1' => address_bin[i] = b'1',
                b'0' => {}
                b'X' => address_bin[i] = b'0',
                hm => bail!("What is {}", hm),
            }
        }

        let floating_indices: Vec<usize> = self
            .mask
            .iter()
            .enumerate()
            .filter(|(_, &c)| c == b'X')
            .map(|(i, _)| i)
            .collect();

        let mut addresses = vec![];

        for v in 0..2_usize.pow(floating_indices.len() as u32) {
            let version_bin = format!("{:064b}", v).into_bytes();
            for (index, value) in
                floating_indices.iter().zip(version_bin.iter().rev())
            {
                address_bin[*index] = *value;
                addresses.push(
                    usize::from_str_radix(
                        str::from_utf8(&address_bin)
                            .context("Failed to parse what I created.")?,
                        2,
                    )
                    .context("Unparseable binary.")?,
                )
            }
        }
        Ok(addresses)
    }

    fn mem_sum(&self) -> usize {
        self.memory.values().sum()
    }
}

enum Command {
    Mask(String),
    Mem(usize, usize),
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
    let commands: Vec<Command> =
        read_entries::<Command>("./data/day-14.txt").collect();
    let mut program = ProgramV1::new();
    for command in &commands {
        program.execute(command)?;
    }
    println!("Mem sum = {}", program.mem_sum());
    let mut program = ProgramV2::new();
    for command in &commands {
        program.execute(command)?;
    }
    println!("Mem sum = {}", program.mem_sum());
    Ok(())
}

use anyhow::{bail, Context, Error, Result};
use aoc_2020::read_entries;
use std::{collections::HashSet, str::FromStr};

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self> {
        let mut parts = input.split(" ");
        let name = parts.next().context("Missing instruction.")?;
        let value = parts
            .next()
            .context("Missing value")?
            .parse::<i32>()
            .context("Unparseable value")?;
        Ok(match name {
            "nop" => Self::Nop(value),
            "acc" => Self::Acc(value),
            "jmp" => Self::Jmp(value),
            _ => bail!("Unexpected instruction"),
        })
    }
}

#[derive(Debug)]
struct Program {
    acc: i32,
    instructions: Vec<Instruction>,
    cursor: usize,
}

impl Program {
    fn step(&mut self) -> bool {
        let instruction = &self.instructions[self.cursor];
        match instruction {
            Instruction::Nop(_) => self.cursor += 1,
            Instruction::Acc(value) => {
                self.acc += value;
                self.cursor += 1
            }
            Instruction::Jmp(value) => {
                self.cursor = (self.cursor as i32 + value) as usize
            }
        }
        self.cursor == self.instructions.len()
    }

    fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            acc: 0,
            cursor: 0,
            instructions,
        }
    }

    fn reset(&mut self) {
        self.acc = 0;
        self.cursor = 0;
    }
}

fn main() {
    let instructions: Vec<_> =
        read_entries::<Instruction>("./data/day-08.txt").collect();
    let mut program = Program::new(instructions);
    run_until_end(&mut program);
    println!("At the start of the second loop acc={}", program.acc);
    for i in 0..program.instructions.len() {
        program.reset();
        if patch_and_run(&mut program, i) {
            println!("At the program termination the acc={}", program.acc)
        }
    }
}

fn run_until_end(program: &mut Program) -> bool {
    // We'll return false if a loop is detected.
    let mut cursor_positions = HashSet::<usize>::new();
    while !cursor_positions.contains(&program.cursor) {
        cursor_positions.insert(program.cursor);
        if program.step() {
            return true;
        }
    }
    return false;
}

fn patch_and_run(program: &mut Program, index: usize) -> bool {
    let instruction = program.instructions[index];
    let patch = match instruction {
        Instruction::Nop(val) => Instruction::Jmp(val),
        Instruction::Jmp(val) => Instruction::Nop(val),
        Instruction::Acc(val) => Instruction::Acc(val),
    };

    program.instructions[index] = patch;
    let res = run_until_end(program);
    program.instructions[index] = instruction;
    res
}

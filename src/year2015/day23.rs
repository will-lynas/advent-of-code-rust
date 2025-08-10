use std::str::FromStr;

use gxhash::{
    HashMap,
    HashMapExt,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Register {
    A,
    B,
}

impl FromStr for Register {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "a" => Ok(Register::A),
            "b" => Ok(Register::B),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub enum Instruction {
    Hlf(Register),
    Tpl(Register),
    Inc(Register),
    Jmp(i32),
    Jie(Register, i32),
    Jio(Register, i32),
}

pub fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let (instruction, remainder) = line.split_once(' ').unwrap();
            match instruction {
                "hlf" => Instruction::Hlf(remainder.parse().unwrap()),
                "tpl" => Instruction::Tpl(remainder.parse().unwrap()),
                "inc" => Instruction::Inc(remainder.parse().unwrap()),
                "jmp" => Instruction::Jmp(remainder.parse().unwrap()),
                "jie" => {
                    let (register, offset) = remainder.split_once(", ").unwrap();
                    Instruction::Jie(register.parse().unwrap(), offset.parse().unwrap())
                }
                "jio" => {
                    let (register, offset) = remainder.split_once(", ").unwrap();
                    Instruction::Jio(register.parse().unwrap(), offset.parse().unwrap())
                }
                _ => unreachable!(),
            }
        })
        .collect()
}

pub fn part1(instructions: &[Instruction]) -> i32 {
    let mut registers: HashMap<Register, i32> = HashMap::new();
    let n = instructions.len();
    let mut pc = 0i32;
    while 0 <= pc && pc < n as i32 {
        match instructions[pc as usize] {
            Instruction::Hlf(register) => {
                *registers.entry(register).or_insert(0) /= 2;
                pc += 1;
            }
            Instruction::Tpl(register) => {
                *registers.entry(register).or_insert(0) *= 3;
                pc += 1;
            }
            Instruction::Inc(register) => {
                *registers.entry(register).or_insert(0) += 1;
                pc += 1;
            }
            Instruction::Jmp(offset) => pc += offset,
            Instruction::Jie(register, offset) => {
                if registers.get(&register).unwrap_or(&0) % 2 == 0 {
                    pc += offset;
                } else {
                    pc += 1;
                }
            }
            Instruction::Jio(register, offset) => {
                if *registers.get(&register).unwrap_or(&0) == 1 {
                    pc += offset;
                } else {
                    pc += 1;
                }
            }
        }
    }
    registers[&Register::B]
}

pub fn part2(input: &[Instruction]) -> usize {
    input.len()
}

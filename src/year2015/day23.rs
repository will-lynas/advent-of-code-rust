use std::{
    ops::{
        Index,
        IndexMut,
    },
    str::FromStr,
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

struct RegisterManager {
    data: [i32; 2],
}

impl RegisterManager {
    fn new() -> Self {
        Self { data: [0, 0] }
    }
}

impl Index<Register> for RegisterManager {
    type Output = i32;
    fn index(&self, index: Register) -> &Self::Output {
        match index {
            Register::A => &self.data[0],
            Register::B => &self.data[1],
        }
    }
}

impl IndexMut<Register> for RegisterManager {
    fn index_mut(&mut self, index: Register) -> &mut Self::Output {
        match index {
            Register::A => &mut self.data[0],
            Register::B => &mut self.data[1],
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

fn run(instructions: &[Instruction], registers: &mut RegisterManager) {
    let n = instructions.len();
    let mut pc = 0i32;
    while 0 <= pc && pc < n as i32 {
        match instructions[pc as usize] {
            Instruction::Hlf(register) => {
                registers[register] /= 2;
                pc += 1;
            }
            Instruction::Tpl(register) => {
                registers[register] *= 3;
                pc += 1;
            }
            Instruction::Inc(register) => {
                registers[register] += 1;
                pc += 1;
            }
            Instruction::Jmp(offset) => pc += offset,
            Instruction::Jie(register, offset) => {
                if registers[register] % 2 == 0 {
                    pc += offset;
                } else {
                    pc += 1;
                }
            }
            Instruction::Jio(register, offset) => {
                if registers[register] == 1 {
                    pc += offset;
                } else {
                    pc += 1;
                }
            }
        }
    }
}

pub fn part1(instructions: &[Instruction]) -> i32 {
    let mut registers = RegisterManager::new();
    run(instructions, &mut registers);
    registers[Register::B]
}

pub fn part2(instructions: &[Instruction]) -> i32 {
    let mut registers = RegisterManager::new();
    registers[Register::A] = 1;
    run(instructions, &mut registers);
    registers[Register::B]
}

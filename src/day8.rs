use crate::{Day, NoSolutionFoundError};

use lazy_static::lazy_static;
use rayon::prelude::*;
use regex::Regex;
use std::{
    collections::HashSet,
    convert::TryInto,
    error::Error,
    fmt::{Display, Formatter},
};

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    #[test]
    fn day8_1_sample() {
        let lines: Vec<&str> = crate::split_input(SAMPLE);
        let day8 = Day8::new(lines);
        assert_eq!(5, day8.solve1().unwrap());
    }

    #[test]
    fn day8_2_sample() {
        let lines: Vec<&str> = crate::split_input(SAMPLE);
        let day8 = Day8::new(lines);
        assert_eq!(8, day8.solve2().unwrap());
    }
}

pub struct Day8 {
    lines: Vec<String>,
}

impl Day8 {
    pub fn new(lines: Vec<&str>) -> Box<dyn Day> {
        Box::new(Day8 {
            lines: lines.into_iter().map(str::to_string).collect(),
        })
    }
}

impl Day for Day8 {
    fn solve1(&self) -> Result<i64, Box<dyn Error>> {
        let mut program = Program::new(&self.lines);
        match program.run() {
            Ok(a) | Err(ProgramError::InfiniteLoop(a)) => Ok(a),
            _ => Err(NoSolutionFoundError::new(8, 1)),
        }
    }

    fn solve2(&self) -> Result<i64, Box<dyn Error>> {
        let solver = Solver::new(&self.lines);
        Ok(solver.solve())
    }
}

lazy_static! {
    pub static ref PARSER: Regex = Regex::new(r"(\w{3}) ([+-]\d+)").unwrap();
}

#[derive(Clone, Copy)]
enum Instruction {
    NOP(i64),
    ACC(i64),
    JMP(i64),
}

impl Instruction {
    fn parse(line: &str) -> Instruction {
        let cap = PARSER.captures(line).unwrap();
        match cap.get(1).unwrap().as_str() {
            "nop" => Instruction::NOP(cap.get(2).unwrap().as_str().parse().unwrap()),
            "acc" => Instruction::ACC(cap.get(2).unwrap().as_str().parse().unwrap()),
            "jmp" => Instruction::JMP(cap.get(2).unwrap().as_str().parse().unwrap()),
            _ => panic!("Invalid instruction: {}", line),
        }
    }

    fn twiddle(self) -> Instruction {
        match self {
            Instruction::NOP(i) => Instruction::JMP(i),
            Instruction::JMP(i) => Instruction::NOP(i),
            i => i,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ProgramError {
    InfiniteLoop(i64),
    InvalidIp(i64),
}

impl std::error::Error for ProgramError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl Display for ProgramError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ProgramError::InvalidIp(ip) => {
                write!(f, "Instruction pointer reached invalid position: {}", ip)
            }
            ProgramError::InfiniteLoop(acc) => {
                write!(f, "Infinite loop found. Accumulator at loop: {}", acc)
            }
        }
    }
}

pub struct Program {
    instructions: Vec<Instruction>,
    acc: i64,
    ip: usize,
    visited: HashSet<usize>,
}

impl Program {
    pub fn new(lines: &Vec<String>) -> Program {
        Program::new_internal(lines.iter().map(|l| Instruction::parse(l)).collect())
    }

    fn new_internal(instructions: Vec<Instruction>) -> Program {
        Program {
            instructions,
            acc: 0,
            ip: 0,
            visited: HashSet::new(),
        }
    }

    pub fn run(&mut self) -> Result<i64, ProgramError> {
        while !self.visited.contains(&self.ip) {
            if self.ip == self.instructions.len() {
                return Ok(self.acc);
            }
            self.visited.insert(self.ip);
            let instruction = self.instructions.get(self.ip).unwrap();
            match instruction {
                Instruction::ACC(i) => self.acc += i,
                Instruction::JMP(i) => {
                    let new_ip = self.ip as i64 + i - 1;
                    self.ip = new_ip
                        .try_into()
                        .map_err(|_| ProgramError::InvalidIp(new_ip))?;
                }
                Instruction::NOP(_) => (),
            };
            self.ip += 1;
        }
        Err(ProgramError::InfiniteLoop(self.acc))
    }
}

pub struct Solver {
    instructions: Vec<Instruction>,
    possible_changes: HashSet<usize>,
}

impl Solver {
    pub fn new(lines: &Vec<String>) -> Solver {
        let instructions: Vec<Instruction> = lines.iter().map(|l| Instruction::parse(l)).collect();
        let possible_changes: HashSet<usize> = instructions
            .iter()
            .enumerate()
            .filter(|(_, i)| match i {
                Instruction::ACC(_) => false,
                _ => true,
            })
            .map(|(p, _)| p)
            .collect();

        Solver {
            instructions,
            possible_changes,
        }
    }

    pub fn solve(&self) -> i64 {
        let solutions: Vec<i64> = self
            .possible_changes
            .par_iter()
            .map(|m| self.twiddle_instruction(*m))
            .map(Program::new_internal)
            .map(|mut p| p.run())
            .flatten()
            .collect();

        *solutions.get(0).unwrap()
    }

    fn twiddle_instruction(&self, change: usize) -> Vec<Instruction> {
        let mut instructions = self.instructions.clone();
        if let Some(instruction) = instructions.get_mut(change) {
            *instruction = instruction.twiddle();
        }
        instructions
    }
}

use lazy_static::lazy_static;
use rayon::prelude::*;
use regex::Regex;
use std::collections::HashSet;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

lazy_static! {
    pub static ref PARSER: Regex = Regex::new(r"(\w{3}) ([+-]\d+)").unwrap();
}

#[derive(Clone, Copy)]
enum Instruction {
    NOP(i32),
    ACC(i32),
    JMP(i32),
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

pub struct Program {
    instructions: Vec<Instruction>,
    acc: i32,
    ip: usize,
    visited: HashSet<usize>,
}

impl Program {
    pub fn new(lines: Vec<&str>) -> Program {
        Program::new_internal(
            lines
                .into_iter()
                .map(Instruction::parse)
                .collect::<Vec<_>>(),
        )
    }

    fn new_internal(instructions: Vec<Instruction>) -> Program {
        Program {
            instructions,
            acc: 0,
            ip: 0,
            visited: HashSet::new(),
        }
    }

    pub fn run(&mut self) -> Option<i32> {
        while !self.visited.contains(&self.ip) {
            if self.ip == self.instructions.len() {
                return Some(self.acc);
            }
            self.visited.insert(self.ip);
            let instruction = self.instructions.get(self.ip).unwrap();
            match instruction {
                Instruction::ACC(i) => self.acc += i,
                Instruction::JMP(i) => self.ip = (self.ip as i32 + i - 1) as usize,
                Instruction::NOP(_) => (),
            };
            self.ip += 1;
        }
        None
    }

    pub fn reset(&mut self) {
        self.acc = 0;
        self.ip = 0;
        self.visited = HashSet::new();
    }
}

pub struct Solver {
    instructions: Vec<Instruction>,
    possible_changes: HashSet<usize>,
}

impl Solver {
    pub fn new(lines: Vec<&str>) -> Solver {
        let instructions: Vec<Instruction> = lines.into_iter().map(Instruction::parse).collect();
        let possible_changes: HashSet<usize> = instructions
            .iter()
            .enumerate()
            .filter(|(_, i)| match i {
                Instruction::NOP(_) | Instruction::JMP(_) => true,
                _ => false,
            })
            .map(|(p, _)| p)
            .collect();

        Solver {
            instructions,
            possible_changes,
        }
    }

    pub fn solve(&self) -> i32 {
        let solutions: Vec<i32> = self
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

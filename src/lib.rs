use lazy_static::lazy_static;
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

enum Instruction {
    NOP,
    ACC(i32),
    JMP(i32),
}

impl Instruction {
    fn parse(line: &String) -> Instruction {
        let cap = PARSER.captures(line).unwrap();
        match cap.get(1).unwrap().as_str() {
            "nop" => Instruction::NOP,
            "acc" => Instruction::ACC(cap.get(2).unwrap().as_str().parse().unwrap()),
            "jmp" => Instruction::JMP(cap.get(2).unwrap().as_str().parse().unwrap()),
            _ => panic!("Invalid instruction: {}", line),
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
    pub fn new(lines: Vec<String>) -> Program {
        Program {
            instructions: lines.iter().map(Instruction::parse).collect(),
            acc: 0,
            ip: 0,
            visited: HashSet::new(),
        }
    }
    pub fn run(mut self) -> i32 {
        while !self.visited.contains(&self.ip) {
            self.visited.insert(self.ip);
            let instruction = self.instructions.get(self.ip).unwrap();
            match instruction {
                Instruction::ACC(i) => self.acc += i,
                Instruction::JMP(i) => self.ip = (self.ip as i32 + i - 1) as usize,
                Instruction::NOP => (),
            };
            self.ip += 1;
        }
        self.acc
    }
}

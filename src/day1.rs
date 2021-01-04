use std::error::Error;

use crate::Day;
use crate::NoSolutionFoundError;

pub struct Day1 {
    expenses: Vec<u32>,
}

impl Day1 {
    pub fn new(lines: Vec<&str>) -> Box<dyn Day> {
        Box::new(Day1 {
            expenses: lines.iter().flat_map(|l| l.parse()).collect(),
        })
    }
}

impl Day for Day1 {
    fn solve1(&self) -> Result<u64, Box<dyn Error>> {
        for first in self.expenses.iter() {
            for second in self.expenses.iter().skip(1) {
                if first + second == 2020 {
                    return Ok((first * second) as u64);
                }
            }
        }
        Err(NoSolutionFoundError::new(1, 1))
    }

    fn solve2(&self) -> Result<u64, Box<dyn Error>> {
        for i in 0..self.expenses.len() - 2 {
            for j in i + 1..self.expenses.len() - 1 {
                for k in j + 1..self.expenses.len() {
                    let first = self.expenses.get(i).unwrap();
                    let second = self.expenses.get(j).unwrap();
                    let third = self.expenses.get(k).unwrap();
                    if first + second + third == 2020 {
                        return Ok((first * second * third) as u64);
                    }
                }
            }
        }
        Err(NoSolutionFoundError::new(1, 2))
    }
}

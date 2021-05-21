use std::error::Error;

use crate::Day;
use crate::NoSolutionFoundError;

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "1721
979
366
299
675
1456";

    #[test]
    fn day1_1_sample() {
        let lines: Vec<&str> = crate::split_input(SAMPLE);
        let day1 = Day1::new(lines);
        assert_eq!(514579, day1.solve1().unwrap());
    }

    #[test]
    fn day1_2_sample() {
        let lines: Vec<&str> = crate::split_input(SAMPLE);
        let day1 = Day1::new(lines);
        assert_eq!(241861950, day1.solve2().unwrap());
    }

    #[test]
    fn day1_1() {
        let file = std::fs::read_to_string("data/day1.txt").unwrap();
        let lines: Vec<&str> = crate::split_input(&file);
        let day1 = Day1::new(lines);
        assert_eq!(964875, day1.solve1().unwrap());
    }

    #[test]
    fn day1_2() {
        let file = std::fs::read_to_string("data/day1.txt").unwrap();
        let lines: Vec<&str> = crate::split_input(&file);
        let day1 = Day1::new(lines);
        assert_eq!(158661360, day1.solve2().unwrap());
    }
}

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

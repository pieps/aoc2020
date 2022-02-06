mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

use std::fmt::{Display, Formatter};
use std::ops::Range;
use std::{error::Error, fs};

pub trait Day {
    fn solve1(&self) -> Result<i64, Box<dyn Error>>;
    fn solve2(&self) -> Result<i64, Box<dyn Error>>;
}

#[derive(Debug, Clone)]
pub struct NoSolutionFoundError {
    day: u32,
    part: u32,
}

impl NoSolutionFoundError {
    pub fn new(day: u32, part: u32) -> Box<NoSolutionFoundError> {
        Box::new(NoSolutionFoundError { day, part })
    }
}

impl Display for NoSolutionFoundError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "No solution found for day {}, part {}",
            self.day, self.part
        )
    }
}

impl Error for NoSolutionFoundError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

#[derive(Debug, Clone)]
pub struct OutOfBoundsError {
    value: u32,
    bounds: Range<u32>,
}

impl OutOfBoundsError {
    pub fn new(value: u32, bounds: Range<u32>) -> Box<OutOfBoundsError> {
        Box::new(OutOfBoundsError { value, bounds })
    }
}

impl Display for OutOfBoundsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Value {} not in the range [{}..{}].",
            self.value, self.bounds.start, self.bounds.end
        )
    }
}

impl Error for OutOfBoundsError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

pub fn split_input(text: &str) -> Vec<&str> {
    text.trim_end().split('\n').collect()
}

pub fn get_day(day: u32) -> Result<Box<dyn Day>, Box<dyn Error>> {
    if day >= 25 || day == 0 {
        Err(OutOfBoundsError::new(day, 1..25))
    } else {
        let l = fs::read_to_string(format!("data/day{}.txt", day))?;
        let lines: Vec<&str> = split_input(&l);
        match day {
            1 => Ok(day1::Day1::new(lines)),
            2 => Ok(day2::Day2::new(lines)),
            3 => Ok(day3::Day3::new(lines)),
            4 => Ok(day4::Day4::new(lines)),
            5 => Ok(day5::Day5::new(lines)),
            6 => Ok(day6::Day6::new(lines)),
            7 => Ok(day7::Day7::new(lines)),
            8 => Ok(day8::Day8::new(lines)),
            9 => Ok(day9::Day9::<25>::new(lines)),
            //10 => Ok(day10::Day10::new(lines)),
            //11 => Ok(day11::Day11::new(lines)),
            //12 => Ok(day12::Day12::new(lines)),
            //13 => Ok(day13::Day13::new(lines)),
            //14 => Ok(day14::Day14::new(lines)),
            //15 => Ok(day15::Day15::new(lines)),
            //16 => Ok(day16::Day16::new(lines)),
            //17 => Ok(day17::Day17::new(lines)),
            //18 => Ok(day18::Day18::new(lines)),
            //19 => Ok(day19::Day19::new(lines)),
            //20 => Ok(day20::Day20::new(lines)),
            //21 => Ok(day21::Day21::new(lines)),
            //22 => Ok(day22::Day22::new(lines)),
            //23 => Ok(day23::Day23::new(lines)),
            //24 => Ok(day24::Day24::new(lines)),
            //25 => Ok(day25::Day25::new(lines)),
            _ => Err(OutOfBoundsError::new(day, 1..25)),
        }
    }
}

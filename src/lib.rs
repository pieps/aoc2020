//use day1::Day1;
use std::fs;

pub trait Day {
    fn solve1(&self) -> u64;
    fn solve2(&self) -> u64;
}

pub fn get_day(day: u32) -> Result<Box<dyn Day>, String> {
    if day >= 25 || day == 0 {
        Err(format!("Day must be in the range [1..25]. Found {}.", day))
    } else {
        let l = fs::read_to_string(format!("data/day{}.txt", day));
        match day {
            //1 => Ok(Day1::new(l)),
            //2 => Ok(Day2::new(l)),
            //3 => Ok(Day3::new(l)),
            //4 => Ok(Day4::new(l)),
            //5 => Ok(Day5::new(l)),
            //6 => Ok(Day6::new(l)),
            //7 => Ok(Day7::new(l)),
            //8 => Ok(Day8::new(l)),
            //9 => Ok(Day9::new(l)),
            //10 => Ok(Day10::new(l)),
            //11 => Ok(Day11::new(l)),
            //12 => Ok(Day12::new(l)),
            //13 => Ok(Day13::new(l)),
            //14 => Ok(Day14::new(l)),
            //15 => Ok(Day15::new(l)),
            //16 => Ok(Day16::new(l)),
            //17 => Ok(Day17::new(l)),
            //18 => Ok(Day18::new(l)),
            //19 => Ok(Day19::new(l)),
            //20 => Ok(Day20::new(l)),
            //21 => Ok(Day21::new(l)),
            //22 => Ok(Day22::new(l)),
            //23 => Ok(Day23::new(l)),
            //24 => Ok(Day24::new(l)),
            //25 => Ok(Day25::new(l)),
            _ => Err(format!("Day must be in the range [1..25]. Found {}.", day)),
        }
    }
}

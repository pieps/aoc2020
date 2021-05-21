use lazy_static::lazy_static;
use regex::{Captures, Regex};
use std::error::Error;

use crate::Day;

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";

    #[test]
    fn day2_1_sample() {
        let lines: Vec<&str> = crate::split_input(SAMPLE);
        let day2 = Day2::new(lines);
        assert_eq!(2, day2.solve1().unwrap());
    }

    #[test]
    fn day2_2_sample() {
        let lines: Vec<&str> = crate::split_input(SAMPLE);
        let day2 = Day2::new(lines);
        assert_eq!(1, day2.solve2().unwrap());
    }

    #[test]
    fn day2_1() {
        let file = std::fs::read_to_string("data/day2.txt").unwrap();
        let lines: Vec<&str> = crate::split_input(&file);
        let day2 = Day2::new(lines);
        assert_eq!(477, day2.solve1().unwrap());
    }

    #[test]
    fn day2_2() {
        let file = std::fs::read_to_string("data/day2.txt").unwrap();
        let lines: Vec<&str> = crate::split_input(&file);
        let day2 = Day2::new(lines);
        assert_eq!(686, day2.solve2().unwrap());
    }
}

struct Entry(Rule, Password);
struct Rule {
    start: usize,
    end: usize,
    letter: char,
}
type Password = String;

pub struct Day2 {
    entries: Vec<Entry>,
}

impl Day2 {
    pub fn new(lines: Vec<&str>) -> Box<Day2> {
        Box::new(Day2 {
            entries: lines.iter().map(|l| process_line(l)).collect(),
        })
    }
}

impl Day for Day2 {
    fn solve1(&self) -> Result<u64, Box<dyn Error>> {
        Ok(self.entries.iter().map(Entry::check1).sum())
    }

    fn solve2(&self) -> Result<u64, Box<dyn Error>> {
        Ok(self.entries.iter().map(Entry::check2).sum())
    }
}

fn process_line(line: &str) -> Entry {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();
    }
    let captures: Captures = RE.captures(&line).unwrap();

    let rule = Rule {
        start: captures.get(1).unwrap().as_str().parse().unwrap(),
        end: captures.get(2).unwrap().as_str().parse().unwrap(),
        letter: captures.get(3).unwrap().as_str().parse().unwrap(),
    };

    let password = captures.get(4).unwrap().as_str();

    Entry(rule, String::from(password))
}

impl Entry {
    fn check1(&self) -> u64 {
        let count = self.1.chars().filter(|c| c == &self.0.letter).count();
        match count {
            x if x >= self.0.start && x <= self.0.end => 1,
            _ => 0,
        }
    }

    fn check2(&self) -> u64 {
        let first = self.char_at_ord(self.0.start) == self.0.letter;
        let second = self.char_at_ord(self.0.end) == self.0.letter;
        if first ^ second {
            1
        } else {
            0
        }
    }

    fn char_at_ord(&self, pos: usize) -> char {
        self.1.chars().skip(pos - 1).next().unwrap()
    }
}

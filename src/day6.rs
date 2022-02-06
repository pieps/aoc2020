use crate::Day;

use std::collections::HashSet;

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "abc

a
b
c

ab
ac

a
a
a
a

b";

    #[test]
    fn day6_1_sample() {
        let lines: Vec<&str> = crate::split_input(SAMPLE);
        let day6 = Day6::new(lines);
        assert_eq!(11, day6.solve1().unwrap());
    }

    #[test]
    fn day6_2_sample() {
        let lines: Vec<&str> = crate::split_input(SAMPLE);
        let day6 = Day6::new(lines);
        assert_eq!(6, day6.solve2().unwrap());
    }
}

pub struct Day6 {
    lines: Vec<Vec<String>>,
}

impl Day for Day6 {
    fn solve1(&self) -> Result<u64, Box<dyn std::error::Error>> {
        let mut total = 0;
        for line in &self.lines {
            let set: HashSet<char> = line.iter().flat_map(|l| l.chars()).collect();
            total += set.len() as u64;
        }
        Ok(total)
    }
    fn solve2(&self) -> Result<u64, Box<dyn std::error::Error>> {
        let mut total = 0;
        for line in &self.lines {
            let mut iter = line.iter().map(|l| l.chars().collect::<HashSet<char>>());
            let set: HashSet<char> = iter.next().map(|s| iter.fold(s, |a, b| &a & &b)).unwrap();
            total += set.len() as u64;
        }
        Ok(total)
    }
}

impl Day6 {
    pub fn new(lines: Vec<&str>) -> Box<dyn Day> {
        Box::new(Day6 {
            lines: partition(lines),
        })
    }
}

fn partition(lines: Vec<&str>) -> Vec<Vec<String>> {
    let mut v: Vec<Vec<String>> = Vec::new();
    let mut group: Vec<String> = Vec::new();
    for line in lines {
        if line == "" {
            v.push(group);
            group = Vec::new();
        } else {
            group.push(line.to_owned());
        }
    }

    v.push(group);
    v
}

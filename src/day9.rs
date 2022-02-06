use crate::{Day, NoSolutionFoundError};

use std::collections::{HashSet, VecDeque};

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE: &str = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

    #[test]
    fn day9_1_sample() {
        let lines: Vec<&str> = crate::split_input(SAMPLE);
        let d = Day9::<5>::new(lines);
        assert_eq!(127, d.solve1().unwrap());
    }

    #[test]
    fn day9_2_sample() {
        let lines: Vec<&str> = crate::split_input(SAMPLE);
        let d = Day9::<5>::new(lines);
        assert_eq!(62, d.solve2().unwrap());
    }
}

pub struct Day9<const WINDOW_SIZE: usize> {
    numbers: Vec<u32>,
}

impl<const WINDOW_SIZE: usize> Day for Day9<WINDOW_SIZE> {
    fn solve1(&self) -> Result<i64, Box<dyn std::error::Error>> {
        self.find_needle()
            .ok_or(Box::new(NoSolutionFoundError::new(9, 1)))
    }

    fn solve2(&self) -> Result<i64, Box<dyn std::error::Error>> {
        self.find_weakness(127)
            .ok_or(Box::new(NoSolutionFoundError::new(9, 2)))
    }
}

impl<const WINDOW_SIZE: usize> Day9<WINDOW_SIZE> {
    pub fn new(lines: Vec<&str>) -> Box<dyn Day> {
        let numbers = lines.iter().map(|l| str::parse(l).unwrap()).collect();
        Box::new(Day9::<WINDOW_SIZE> { numbers })
    }

    pub fn find_weakness(&self, needle: u32) -> Option<i64> {
        self.sum_finder(needle)
            .map(|set| (set.iter().max().unwrap() + set.iter().min().unwrap()) as i64)
    }

    fn sum_finder(&self, needle: u32) -> Option<VecDeque<u32>> {
        let mut set: VecDeque<u32> = VecDeque::new();
        let mut sum = 0;
        for &num in self.numbers.iter() {
            set.push_back(num);
            sum += num;
            while sum > needle {
                sum -= set.pop_front().unwrap();
            }
            if sum == needle {
                return Some(set);
            }
        }
        None
    }

    pub fn find_needle(&self) -> Option<i64> {
        let mut window: VecDeque<u32> = self.numbers.iter().take(WINDOW_SIZE).cloned().collect();
        for &i in self.numbers.iter().skip(WINDOW_SIZE) {
            if let None = Day9::<WINDOW_SIZE>::find_pair(i, &window) {
                return Some(i as i64);
            }
            window.pop_front();
            window.push_back(i);
        }
        None
    }

    fn find_pair(num: u32, window: &VecDeque<u32>) -> Option<(u32, u32)> {
        let window_set: HashSet<u32> = window.iter().cloned().collect();
        for &i in window_set.iter() {
            if i >= num {
                continue;
            }
            let remainder = num - i;
            if remainder != i && window_set.contains(&remainder) {
                return Some((i, remainder));
            }
        }
        None
    }
}

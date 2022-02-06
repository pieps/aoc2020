use std::collections::{HashMap, VecDeque};

use crate::Day;

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_1: &str = "16
10
15
5
1
11
7
19
6
12
4";

    const SAMPLE_2: &str = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

    #[test]
    fn day10_1_sample_1() {
        let lines: Vec<&str> = crate::split_input(SAMPLE_1);
        let d = Day10::new(lines);
        assert_eq!(d.solve1().unwrap(), 7 * 5);
    }

    #[test]
    fn day10_1_sample_2() {
        let lines: Vec<&str> = crate::split_input(SAMPLE_2);
        let d = Day10::new(lines);
        assert_eq!(d.solve1().unwrap(), 22 * 10);
    }

    #[test]
    fn day10_2_sample_1() {
        let lines: Vec<&str> = crate::split_input(SAMPLE_1);
        let d = Day10::new(lines);
        assert_eq!(d.solve2().unwrap(), 8);
    }

    #[test]
    fn day10_2_sample_2() {
        let lines: Vec<&str> = crate::split_input(SAMPLE_2);
        let d = Day10::new(lines);
        assert_eq!(d.solve2().unwrap(), 19208);
    }
}

pub struct Day10 {
    jolts: VecDeque<u32>,
}

impl Day for Day10 {
    fn solve1(&self) -> Result<i64, Box<dyn std::error::Error>> {
        Ok(self.solve() as i64)
    }

    fn solve2(&self) -> Result<i64, Box<dyn std::error::Error>> {
        Ok(self.reachable_from(0) as i64)
    }
}

impl Day10 {
    pub fn new(lines: Vec<&str>) -> Box<dyn Day> {
        let mut jolts: Vec<u32> = lines.into_iter().map(|l| l.parse().unwrap()).collect();
        jolts.sort();
        let mut jolts: VecDeque<u32> = jolts.into();
        jolts.push_front(0);
        Box::new(Day10 { jolts })
    }

    pub fn solve(&self) -> u32 {
        let mut bags: HashMap<u32, u32> = HashMap::new();
        self.jolts
            .iter()
            .skip(1)
            .fold(self.jolts.front().unwrap(), |a, b| {
                *(bags.entry(b - a).or_insert(0)) += 1;
                b
            });
        bags[&1] * (bags[&3] + 1)
    }

    pub fn reachable_from(&self, idx: usize) -> u64 {
        let mut reachable: Vec<Option<u64>> = Vec::new();
        reachable.resize(self.jolts.len(), None);
        *reachable.last_mut().unwrap() = Some(1);
        self.dp(idx, &mut reachable)
    }

    fn dp(&self, idx: usize, reachable: &mut Vec<Option<u64>>) -> u64 {
        if let Some(memo) = reachable[idx] {
            return memo;
        }
        let last = self.jolts.len() - 1;
        if idx == last {
            return 1;
        }
        let mut sum = 0;
        let output = self.jolts[idx];
        for i in (idx + 1)..=(idx + 3) {
            if i > last || self.jolts[i] > output + 3 {
                break;
            }
            sum += self.dp(i, reachable);
        }
        reachable[idx] = Some(sum);
        sum
    }
}

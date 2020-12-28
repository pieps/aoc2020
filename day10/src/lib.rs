use std::collections::{HashMap, VecDeque};

#[cfg(test)]
mod tests {
    use super::*;

    const JOLTS: [u32; 31] = [
        28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8,
        17, 7, 9, 4, 2, 34, 10, 3,
    ];

    #[test]
    fn solve_first() {
        let jolts: Vec<u32> = Vec::from(JOLTS);
        let d = Day10::new(jolts);
        assert_eq!(d.solve(), 22 * 10);
    }

    #[test]
    fn solve_reachable() {
        let jolts: Vec<u32> = Vec::from(JOLTS);
        let d = Day10::new(jolts);
        assert_eq!(d.reachable_from(0), 19208);
    }
}

pub struct Day10 {
    jolts: VecDeque<u32>,
}

impl Day10 {
    pub fn new(mut jolts: Vec<u32>) -> Day10 {
        jolts.sort();
        let mut jolts: VecDeque<u32> = jolts.into();
        jolts.push_front(0);
        Day10 { jolts }
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

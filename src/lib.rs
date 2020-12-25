use std::collections::HashMap;

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
}

pub struct Day10 {
    jolts: Vec<u32>,
}

impl Day10 {
    pub fn new(mut jolts: Vec<u32>) -> Day10 {
        jolts.sort();
        Day10 { jolts }
    }

    pub fn solve(&self) -> u32 {
        let mut bags: HashMap<u32, u32> = HashMap::new();
        self.jolts.iter().fold(0, |a, b| {
            *(bags.entry(b - a).or_insert(0)) += 1;
            *b
        });
        bags[&1] * (bags[&3] + 1)
    }
}

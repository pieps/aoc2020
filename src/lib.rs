use std::collections::{HashSet, VecDeque};

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: [u32; 20] = [
        35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576,
    ];

    #[test]
    fn it_works() {
        let v = Vec::from(INPUT);
        let d = Day9::new(&v, 5);
        assert_eq!(d.find_needle(), Some(127));
    }
}

pub struct Day9<'a> {
    numbers: &'a Vec<u32>,
    window_size: usize,
}

impl<'a> Day9<'a> {
    pub fn new(numbers: &'a Vec<u32>, window_size: usize) -> Day9 {
        Day9 {
            numbers,
            window_size,
        }
    }

    pub fn find_needle(&self) -> Option<u32> {
        let mut window: VecDeque<u32> = self
            .numbers
            .iter()
            .take(self.window_size)
            .cloned()
            .collect();
        for &i in self.numbers.iter().skip(self.window_size) {
            if let None = Day9::<'a>::find_pair(i, &window) {
                return Some(i);
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

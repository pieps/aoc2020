use std::collections::HashMap;

use crate::Day;

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "939
7,13,x,x,59,x,31,19";

    #[test]
    fn day13_1_sample() {
        let lines = crate::split_input(SAMPLE);
        let d = Day13::new(lines);
        assert_eq!(295, d.solve1().unwrap());
    }

    #[test]
    fn day13_2_sample() {
        let lines = crate::split_input(SAMPLE);
        let d = Day13::new(lines);
        assert_eq!(1068781, d.solve2().unwrap());
    }
}

pub struct Day13 {
    start: u32,
    buses: HashMap<usize, u32>,
}

impl Day13 {
    pub fn new(lines: Vec<&str>) -> Box<dyn Day> {
        let start = lines.get(0).unwrap().parse().unwrap();
        let buses = lines
            .get(1)
            .unwrap()
            .split(',')
            .enumerate()
            .filter(|(_, b)| *b != "x")
            .map(|(a, b)| (a, b.parse().unwrap()))
            .collect();

        Box::new(Day13 { start, buses })
    }

    fn modified_lcm(&self) -> i64 {
        let orig: Vec<u32> = self.buses.values().copied().collect();
        let mut sequence: Vec<i64> = self
            .buses
            .iter()
            .map(|(k, v)| *v as i64 - *k as i64)
            .collect();
        loop {
            if sequence.iter().unique().count() == 1 {
                return *sequence.first().unwrap();
            }
            let lowest = sequence
                .iter()
                .enumerate()
                .reduce(|(i, x), (j, y)| if x < y { (i, x) } else { (j, y) })
                .unwrap()
                .0;
            let to_increment = sequence.get_mut(lowest).unwrap();
            *to_increment = *to_increment + *orig.get(lowest).unwrap() as i64;
        }
    }
}

/**
* Given T is the start time of the first bus, B1, and T+J is the start time of the Nth bus, BN,
* which has an offset of J from B1.
*
* We first want to find T1, which is the solution for B1 and B2:
*   T1 % B1 == 0. (T1 + 1) % B2 == 0 -> T1 % B2 == B2-1.
*
*/

/// Finds T such that T % B1 == B1 - D1 && T % B2 == B2 - D2.
fn find_start_time(b1: i64, d1: i64, b2: i64, d2: i64) -> i64 {}

impl Day for Day13 {
    fn solve1(&self) -> Result<i64, Box<dyn std::error::Error>> {
        let mut soonest = *self.buses.values().next().unwrap();
        for &bus in self.buses.values() {
            if bus - self.start.rem_euclid(bus) < soonest - self.start.rem_euclid(soonest) {
                soonest = bus;
            }
        }
        Ok((soonest * (soonest - self.start.rem_euclid(soonest))) as i64)
    }

    fn solve2(&self) -> Result<i64, Box<dyn std::error::Error>> {
        let sequence = self
            .buses
            .iter()
            .reduce(|acc, elem| modified_lcm(*acc.1, *elem.1, *acc.0 as u32, *elem.0 as u32))
            .unwrap();
        Ok(self.modified_lcm() as i64)
    }
}

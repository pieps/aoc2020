use itertools::Itertools;

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
    }
}

pub struct Day13 {
    start: u32,
    buses: Vec<u32>,
}

impl Day13 {
    pub fn new(lines: Vec<&str>) -> Box<dyn Day> {
        let start = lines.get(0).unwrap().parse().unwrap();
        let buses = lines
            .get(1)
            .unwrap()
            .split(',')
            .filter(|b| *b != "x")
            .map(str::parse)
            .map(Result::unwrap)
            .sorted()
            .collect();
        Box::new(Day13 { start, buses })
    }
}

impl Day for Day13 {
    fn solve1(&self) -> Result<i64, Box<dyn std::error::Error>> {
        let mut soonest = *self.buses.first().unwrap();
        for &bus in &self.buses {
            if self.start.rem_euclid(bus) > self.start.rem_euclid(soonest) {
                soonest = bus;
            }
        }
        Ok((soonest * (soonest - self.start.rem_euclid(soonest))) as i64)
    }

    fn solve2(&self) -> Result<i64, Box<dyn std::error::Error>> {
        todo!()
    }
}

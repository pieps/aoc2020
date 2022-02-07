use crate::Day;

use self::instruction::Instruction;

mod instruction;
mod waypoint;

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "F10
N3
F7
R90
F11";

    #[test]
    fn day12_1_sample() {
        let lines = crate::split_input(SAMPLE);
        let d = Day12::new(lines);
        assert_eq!(25, d.solve1().unwrap());
    }

    #[test]
    fn day12_2_sample() {
        let lines = crate::split_input(SAMPLE);
        let d = Day12::new(lines);
        assert_eq!(286, d.solve2().unwrap());
    }
}

pub struct Day12 {
    instructions: Vec<Instruction>,
}

impl Day12 {
    pub fn new(lines: Vec<&str>) -> Box<dyn Day> {
        Box::new(Day12 {
            instructions: lines.into_iter().map(Instruction::parse).collect(),
        })
    }
}

impl Day for Day12 {
    fn solve1(&self) -> Result<i64, Box<dyn std::error::Error>> {
        let (mut vertical, mut horizontal, mut direction) = (0i64, 0i64, 90i64);

        for instruction in &self.instructions {
            match instruction {
                Instruction::N(d) => vertical = vertical + *d as i64,
                Instruction::S(d) => vertical = vertical - *d as i64,
                Instruction::E(d) => horizontal = horizontal + *d as i64,
                Instruction::W(d) => horizontal = horizontal - *d as i64,
                Instruction::R(d) => direction = (direction + *d as i64).rem_euclid(360),
                Instruction::L(d) => direction = (direction - *d as i64).rem_euclid(360),
                Instruction::F(d) => match direction {
                    0 => vertical = vertical + *d as i64,
                    90 => horizontal = horizontal + *d as i64,
                    180 => vertical = vertical - *d as i64,
                    270 => horizontal = horizontal - *d as i64,
                    _ => unimplemented!("Only cardinal directions supported!"),
                },
            }
        }
        Ok(vertical.abs() + horizontal.abs())
    }

    fn solve2(&self) -> Result<i64, Box<dyn std::error::Error>> {
        let mut pos = (0i64, 0i64);
        let mut waypoint = waypoint::Waypoint(1i64, 10i64);

        for instruction in &self.instructions {
            if let Instruction::F(d) = instruction {
                pos = (
                    pos.0 + waypoint.0 * *d as i64,
                    pos.1 + waypoint.1 * *d as i64,
                );
            } else {
                waypoint = waypoint.handle(instruction);
            }
        }
        Ok(pos.0.abs() + pos.1.abs())
    }
}

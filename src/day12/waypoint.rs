use crate::day12::Instruction;

pub struct Waypoint(pub i64, pub i64);

impl Waypoint {
    pub fn handle(self, instruction: &Instruction) -> Waypoint {
        match instruction {
            Instruction::N(d) => Waypoint(self.0 + *d as i64, self.1),
            Instruction::S(d) => Waypoint(self.0 - *d as i64, self.1),
            Instruction::E(d) => Waypoint(self.0, self.1 + *d as i64),
            Instruction::W(d) => Waypoint(self.0, self.1 - *d as i64),
            Instruction::R(d) => match d {
                0 => Waypoint(self.0, self.1),
                90 => Waypoint(-self.1, self.0),
                180 => Waypoint(-self.0, -self.1),
                270 => Waypoint(self.1, -self.0),
                _ => unimplemented!("Only cardinal directions supported!"),
            },
            Instruction::L(d) => match d {
                0 => Waypoint(self.0, self.1),
                90 => Waypoint(self.1, -self.0),
                180 => Waypoint(-self.0, -self.1),
                270 => Waypoint(-self.1, self.0),
                _ => unimplemented!("Only cardinal directions supported!"),
            },
            _ => panic!("Waypoints can't be moved forward!"),
        }
    }
}

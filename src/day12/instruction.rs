pub enum Instruction {
    N(u32),
    S(u32),
    E(u32),
    W(u32),
    R(u32),
    L(u32),
    F(u32),
}

impl Instruction {
    pub fn parse(line: &str) -> Instruction {
        let (dir, dist) = line.split_at(1);
        match dir {
            "N" => Instruction::N(dist.parse().unwrap()),
            "S" => Instruction::S(dist.parse().unwrap()),
            "E" => Instruction::E(dist.parse().unwrap()),
            "W" => Instruction::W(dist.parse().unwrap()),
            "R" => Instruction::R(dist.parse().unwrap()),
            "L" => Instruction::L(dist.parse().unwrap()),
            "F" => Instruction::F(dist.parse().unwrap()),
            _ => panic!("Invalid instruction: {}", dir),
        }
    }
}

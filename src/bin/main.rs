use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let f = fs::read_to_string("data.txt")?;
    let lines: Vec<&str> = f.split('\n').filter(|l| !l.is_empty()).collect();
    let d = day11::Day11::new(lines);
    let part1 = d.solve_part1();
    println!("{:?}", part1);
    let part2 = d.solve_part2();
    println!("{:?}", part2);
    Ok(())
}

use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let f = fs::read_to_string("data.txt")?;
    let lines: Vec<&str> = f.split('\n').filter(|l| !l.is_empty()).collect();
    let acc = day8::Solver::new(lines).solve();
    println!("{:?}", acc);
    Ok(())
}

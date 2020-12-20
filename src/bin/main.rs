use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("data.txt")?;
    let reader = BufReader::new(f);
    let lines: Vec<String> = reader.lines().flatten().collect();
    let acc = day8::Program::new(lines).run();
    println!("{}", acc);
    Ok(())
}

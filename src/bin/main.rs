use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("data.txt")?;
    let reader = BufReader::new(f);
    let lines = reader.lines().flatten();
    let max: u32 = lines.map(day5_1::parse_pass).max().unwrap();
    println!("{}", max);
    Ok(())
}

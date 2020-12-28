use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("data.txt")?;
    let reader = BufReader::new(f);
    let lines = reader.lines().flatten();
    let total: usize = day6::partition(lines).iter().map(day6::process_group).sum();
    println!("{}", total);
    Ok(())
}

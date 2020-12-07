use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("data.txt")?;
    let reader = BufReader::new(f);
    let bf: day4_1::BatchFile = reader.lines().collect();
    let passports: Vec<day4_1::Passport> = bf.collect();
    println!("Valid passports: {}", passports.len());
    Ok(())
}

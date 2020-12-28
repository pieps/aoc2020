use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("data.txt")?;
    let reader = BufReader::new(f);
    let lines = reader.lines().flatten();
    let used_seats: Vec<u32> = lines.map(day5_1::parse_pass).collect();
    println!("{}", day5_1::find_needle(&used_seats));
    Ok(())
}

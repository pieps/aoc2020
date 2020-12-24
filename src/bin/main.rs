use std::error::Error;
use std::fs;

const WINDOW_SIZE: usize = 25;

fn main() -> Result<(), Box<dyn Error>> {
    let f = fs::read_to_string("data.txt")?;
    let numbers: Vec<u32> = f
        .split('\n')
        .filter(|l| !l.is_empty())
        .flat_map(|l| l.parse())
        .collect();
    let needle = day9::Day9::new(&numbers, WINDOW_SIZE).find_needle();
    println!("{:?}", needle);
    Ok(())
}

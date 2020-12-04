use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("data.txt")?;
    let reader = BufReader::new(f);
    let v: Vec<u32> = reader
        .lines()
        .flatten()
        .flat_map(|l| l.trim().parse())
        .collect();

    for first in v.iter() {
        for second in v.iter().skip(1) {
            for third in v.iter().skip(2) {
                if first + second + third == 2020 {
                    println!("{}", first * second * third);
                }
            }
        }
    }
    Ok(())
}

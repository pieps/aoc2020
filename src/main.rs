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

    for i in 0..v.len() - 2 {
        for j in i + 1..v.len() - 1 {
            for k in j + 1..v.len() {
                let first = v.get(i).unwrap();
                let second = v.get(j).unwrap();
                let third = v.get(k).unwrap();
                if first + second + third == 2020 {
                    println!("{}", first * second * third);
                }
            }
        }
    }
    Ok(())
}

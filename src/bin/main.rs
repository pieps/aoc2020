use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let f = fs::read_to_string("data.txt")?;
    let numbers: Vec<u32> = f
        .split('\n')
        .filter(|l| !l.is_empty())
        .flat_map(|l| l.parse())
        .collect();
    let d = day10::Day10::new(numbers);
    let solution = d.solve();
    println!("{:?}", solution);
    let reachable = d.reachable_from(0);
    println!("{:?}", reachable);
    Ok(())
}

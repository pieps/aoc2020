use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("data.txt")?;
    let reader = BufReader::new(f);
    let lines = reader.lines().flatten();
    let rules = day7::Rules::new(
        lines
            .map(day7::parse_rule)
            .map(|r| (r.id.clone(), r))
            .collect(),
    );
    println!("{}", rules.count_bags("shiny gold") - 1);
    Ok(())
}

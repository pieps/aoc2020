use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("data.txt")?;
    let reader = BufReader::new(f);
    let nodes: Vec<Vec<day3_1::Node>> = reader
        .lines()
        .flatten()
        .map(|l| l.chars().flat_map(to_node).collect())
        .collect();
    let g: day3_1::Graph = day3_1::Graph::new(nodes, 3, 1);
    let trees: u32 = g.sum();
    print!("{}", trees);

    Ok(())
}

fn to_node(c: char) -> Option<day3_1::Node> {
    match c {
        '.' => Some(0),
        '#' => Some(1),
        _ => None,
    }
}

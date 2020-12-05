use day3_2::Graph;
use day3_2::Node;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("data.txt")?;
    let reader = BufReader::new(f);
    let nodes: Vec<Vec<Node>> = reader
        .lines()
        .flatten()
        .map(|l| l.chars().flat_map(to_node).collect())
        .collect();
    let params: Vec<(usize, usize)> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let trees: Vec<u32> = params
        .iter()
        .map(|(x, y)| Graph::new(&nodes, *x, *y))
        .map(Iterator::sum)
        .collect();
    print!("{}", trees.iter().fold(1, |t, u| t * u));

    Ok(())
}

fn to_node(c: char) -> Option<Node> {
    match c {
        '.' => Some(0),
        '#' => Some(1),
        _ => None,
    }
}

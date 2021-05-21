use crate::Day;
use std::iter::Iterator;

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#
";

    #[test]
    fn day3_1_sample() {
        let lines: Vec<&str> = crate::split_input(SAMPLE);
        let day3 = Day3::new(lines);
        assert_eq!(7, day3.solve1().unwrap());
    }

    #[test]
    fn day3_2_sample() {
        let lines: Vec<&str> = crate::split_input(SAMPLE);
        let day3 = Day3::new(lines);
        assert_eq!(336, day3.solve2().unwrap());
    }

    #[test]
    fn day3_1() {
        let file = std::fs::read_to_string("data/day3.txt").unwrap();
        let lines: Vec<&str> = crate::split_input(&file);
        let day3 = Day3::new(lines);
        assert_eq!(268, day3.solve1().unwrap());
    }

    #[test]
    fn day3_2() {
        let file = std::fs::read_to_string("data/day3.txt").unwrap();
        let lines: Vec<&str> = crate::split_input(&file);
        let day3 = Day3::new(lines);
        assert_eq!(3093068400, day3.solve2().unwrap());
    }
}

pub type Node = u32;

pub struct Graph<'a> {
    pub nodes: &'a Vec<Vec<Node>>,

    iter_x: usize,
    iter_y: usize,
    x_step: usize,
    y_step: usize,
}

impl<'a> Graph<'a> {
    pub fn new(nodes: &'a Vec<Vec<Node>>, x_step: usize, y_step: usize) -> Graph<'a> {
        Graph {
            nodes,
            iter_x: 0,
            iter_y: 0,
            x_step,
            y_step,
        }
    }
}

impl<'a> Iterator for Graph<'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter_y += self.y_step;
        if self.iter_y >= self.nodes.len() {
            return None;
        }
        self.iter_x = (self.iter_x + self.x_step) % self.nodes.get(0).unwrap().len();
        Some(&self.nodes[self.iter_y][self.iter_x])
    }
}

fn to_node(c: char) -> Option<Node> {
    match c {
        '.' => Some(0),
        '#' => Some(1),
        _ => None,
    }
}

pub struct Day3 {
    nodes: Vec<Vec<Node>>,
}

impl Day3 {
    pub fn new(lines: Vec<&str>) -> Box<Day3> {
        Box::new(Day3 {
            nodes: lines
                .iter()
                .map(|l| l.chars().flat_map(to_node).collect())
                .collect(),
        })
    }
}

impl Day for Day3 {
    fn solve1(&self) -> Result<u64, Box<dyn std::error::Error>> {
        let g: Graph = Graph::new(&self.nodes, 3, 1);
        Ok(g.sum::<u32>() as u64)
    }

    fn solve2(&self) -> Result<u64, Box<dyn std::error::Error>> {
        let params: Vec<(usize, usize)> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
        let trees: Vec<u32> = params
            .iter()
            .map(|(x, y)| Graph::new(&self.nodes, *x, *y))
            .map(Iterator::sum)
            .collect();
        Ok(trees.iter().fold(1, |t, u| t * u) as u64)
    }
}

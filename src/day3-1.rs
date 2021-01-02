use std::iter::{Iterator, Sum};
use std::ops::Add;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub type Node = u32;

pub struct Graph {
    pub nodes: Vec<Vec<Node>>,

    iter_row: usize,
    iter_col: usize,
    row_step: usize,
    col_step: usize,
}

impl Graph {
    pub fn new(nodes: Vec<Vec<Node>>, row_step: usize, col_step: usize) -> Graph {
        Graph {
            nodes,
            iter_row: 0,
            iter_col: 0,
            row_step,
            col_step,
        }
    }
}

impl Iterator for Graph {
    type Item = Node;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter_col += self.col_step;
        if self.iter_col >= self.nodes.len() {
            return None;
        }
        self.iter_row = (self.iter_row + self.row_step) % self.nodes.get(0).unwrap().len();
        Some(self.nodes[self.iter_col][self.iter_row])
    }
}

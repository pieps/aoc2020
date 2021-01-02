use std::iter::Iterator;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
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

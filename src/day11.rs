mod layout;
mod seat;
mod solver;

use crate::Day;
use layout::Layout;
use solver::{ImmediateNeighborsSolver, Solver, VisibleNeighborsSolver};

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
";

    #[test]
    fn day11_1_sample() {
        let lines = crate::split_input(SAMPLE);
        let d = Day11::new(lines);
        assert_eq!(37, d.solve1().unwrap());
    }

    #[test]
    fn day11_2_sample() {
        let lines = crate::split_input(SAMPLE);
        let d = Day11::new(lines);
        assert_eq!(26, d.solve2().unwrap());
    }
}

pub struct Day11 {
    layout: Layout,
}

impl Day for Day11 {
    fn solve1(&self) -> Result<i64, Box<dyn std::error::Error>> {
        Ok(Day11::solve(self.layout.clone(), ImmediateNeighborsSolver::new()) as i64)
    }

    fn solve2(&self) -> Result<i64, Box<dyn std::error::Error>> {
        Ok(Day11::solve(self.layout.clone(), VisibleNeighborsSolver::new()) as i64)
    }
}

impl Day11 {
    pub fn new(input: Vec<&str>) -> Box<dyn Day> {
        Box::new(Day11 {
            layout: Layout::parse(input),
        })
    }

    fn solve<const N: usize>(mut layout: Layout, solver: Box<dyn Solver<N>>) -> usize {
        loop {
            let layout_copy = solver.one_iteration(&layout);
            if layout_copy == layout {
                break;
            } else {
                layout = layout_copy;
            }
        }
        layout.count_occupied()
    }
}

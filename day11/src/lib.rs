#![feature(min_const_generics)]
mod layout;
mod seat;
mod solver;

use layout::Layout;
use solver::{ImmediateNeighborsSolver, Solver, VisibleNeighborsSolver};

#[cfg(test)]
mod tests {
    use super::*;

    const LAYOUT: [&str; 10] = [
        "L.LL.LL.LL",
        "LLLLLLL.LL",
        "L.L.L..L..",
        "LLLL.LL.LL",
        "L.LL.LL.LL",
        "L.LLLLL.LL",
        "..L.L.....",
        "LLLLLLLLLL",
        "L.LLLLLL.L",
        "L.LLLLL.LL",
    ];

    #[test]
    fn part1() {
        let d = Day11::new(Vec::from(LAYOUT));
        assert_eq!(d.solve_part1(), 37);
    }

    #[test]
    fn part2() {
        let d = Day11::new(Vec::from(LAYOUT));
        assert_eq!(d.solve_part2(), 26);
    }
}

pub struct Day11 {
    layout: Layout,
}

impl Day11 {
    pub fn new(input: Vec<&str>) -> Day11 {
        Day11 {
            layout: Layout::parse(input),
        }
    }

    pub fn solve_part1(&self) -> usize {
        Day11::solve(self.layout.clone(), ImmediateNeighborsSolver::new())
    }

    pub fn solve_part2(&self) -> usize {
        Day11::solve(self.layout.clone(), VisibleNeighborsSolver::new())
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

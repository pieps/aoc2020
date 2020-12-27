mod layout;
mod seat;

use layout::Layout;
use layout::Neighbors;

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
        let mut layout = self.layout.clone();
        loop {
            let layout_copy = layout.one_iteration(Neighbors::Immediate, 4);
            if layout_copy == layout {
                break;
            } else {
                layout = layout_copy;
            }
        }
        layout.count_occupied()
    }

    pub fn solve_part2(&self) -> usize {
        let mut layout = self.layout.clone();
        loop {
            let layout_copy = layout.one_iteration(Neighbors::Visible, 5);
            if layout_copy == layout {
                break;
            } else {
                layout = layout_copy;
            }
        }
        layout.count_occupied()
    }
}

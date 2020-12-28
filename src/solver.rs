use crate::layout::Layout;
use crate::seat::Seat;
pub trait Solver<const N: usize> {
    fn empty_trigger(&self) -> usize {
        N
    }

    fn one_iteration(&self, layout: &Layout) -> Layout {
        let mut seats_copy = layout.seats.clone();
        for row in 0..layout.rows {
            for col in 0..layout.cols {
                let seat = layout.seats[row][col];
                let occupied_neighbors = self
                    .neighbors(row, col, &layout)
                    .into_iter()
                    .filter(|s| s.is_occupied())
                    .count();
                seats_copy[row][col] = match seat {
                    Seat::Empty if occupied_neighbors == 0 => Seat::Occupied,
                    Seat::Occupied if occupied_neighbors >= self.empty_trigger() => Seat::Empty,
                    s => s,
                };
            }
        }
        let layout = Layout::new(seats_copy);
        eprintln!("{}\n", layout);
        layout
    }
    fn neighbors(&self, x: usize, y: usize, layout: &Layout) -> Vec<Seat>;
}

pub struct ImmediateNeighborsSolver {}

impl ImmediateNeighborsSolver {
    pub fn new() -> Box<ImmediateNeighborsSolver> {
        Box::new(ImmediateNeighborsSolver {})
    }
}

impl Solver<4> for ImmediateNeighborsSolver {
    fn neighbors(&self, x: usize, y: usize, layout: &Layout) -> Vec<Seat> {
        assert!(x < layout.rows && y < layout.cols);
        let mut neighbors: Vec<Seat> = Vec::new();
        for row in x.saturating_sub(1)..=(x + 1).min(layout.rows - 1) {
            for col in y.saturating_sub(1)..=(y + 1).min(layout.cols - 1) {
                if row == x && col == y {
                    continue;
                }
                neighbors.push(layout.seats[row][col]);
            }
        }
        neighbors
    }
}

pub struct VisibleNeighborsSolver {}

impl VisibleNeighborsSolver {
    pub fn new() -> Box<VisibleNeighborsSolver> {
        Box::new(VisibleNeighborsSolver {})
    }
}

impl Solver<5> for VisibleNeighborsSolver {
    fn neighbors(&self, x: usize, y: usize, layout: &Layout) -> Vec<Seat> {
        assert!(x < layout.rows && y < layout.cols);
        let mut neighbors: Vec<Seat> = Vec::new();
        for (x_step, y_step) in itertools::iproduct!(-1..=1, -1..=1).filter(|&p| p != (0, 0)) {
            let mut row = (x as isize) + x_step;
            let mut col = (y as isize) + y_step;
            while row >= 0 && row < layout.rows as isize && col >= 0 && col < layout.cols as isize {
                let s = layout.seats[row as usize][col as usize];
                if let Seat::Floor = s {
                    row += x_step;
                    col += y_step;
                } else {
                    neighbors.push(s);
                    break;
                }
            }
        }
        neighbors
    }
}

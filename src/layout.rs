use crate::seat::Seat;

use std::fmt::{self, Debug, Display, Formatter};

#[derive(Debug, Clone, Copy)]
pub enum Neighbors {
    Immediate,
    Visible,
}

#[derive(Clone, PartialEq, Eq)]
pub struct Layout {
    seats: Vec<Vec<Seat>>,
    rows: usize,
    cols: usize,
}

impl Display for Layout {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for row in self.seats.iter() {
            for col in row.iter() {
                write!(f, "{}", col)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Debug for Layout {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl Layout {
    pub fn parse(input: Vec<&str>) -> Layout {
        let mut seats: Vec<Vec<Seat>> = Vec::new();
        seats.reserve(input.len());
        for line in input {
            seats.push(line.chars().map(Seat::parse).collect::<Vec<Seat>>());
        }

        Layout::new(seats)
    }

    pub fn new(seats: Vec<Vec<Seat>>) -> Layout {
        let rows = seats.len();
        let cols = seats[0].len();
        Layout { seats, rows, cols }
    }

    pub fn one_iteration(&self, neighbors: Neighbors, empty_trigger: usize) -> Layout {
        let mut seats_copy = self.seats.clone();
        for row in 0..self.rows {
            for col in 0..self.cols {
                let seat = self.seats[row][col];
                let occupied_neighbors = self
                    .neighbors(row, col, neighbors)
                    .into_iter()
                    .filter(|s| s.is_occupied())
                    .count();
                seats_copy[row][col] = match seat {
                    Seat::Empty if occupied_neighbors == 0 => Seat::Occupied,
                    Seat::Occupied if occupied_neighbors >= empty_trigger => Seat::Empty,
                    s => s,
                };
            }
        }
        let l = Layout::new(seats_copy);
        eprintln!("{}\n", l);
        l
    }

    pub fn count_occupied(&self) -> usize {
        self.seats
            .iter()
            .flat_map(|r| r.iter().filter(|s| s.is_occupied()))
            .count()
    }

    fn neighbors(&self, x: usize, y: usize, neighbors: Neighbors) -> Vec<Seat> {
        match neighbors {
            Neighbors::Immediate => self.immediate_neighbors(x, y),
            Neighbors::Visible => self.visible_neighbors(x, y),
        }
    }

    fn immediate_neighbors(&self, x: usize, y: usize) -> Vec<Seat> {
        assert!(
            x < self.rows && y < self.cols,
            "Index ({}, {}) is out of range (0-{}, 0-{}).",
            x,
            y,
            self.rows,
            self.cols
        );
        let mut neighbors: Vec<Seat> = Vec::new();
        for row in x.saturating_sub(1)..=(x + 1).min(self.rows - 1) {
            for col in y.saturating_sub(1)..=(y + 1).min(self.cols - 1) {
                if row == x && col == y {
                    continue;
                }
                neighbors.push(self.seats[row][col]);
            }
        }
        neighbors
    }

    fn visible_neighbors(&self, x: usize, y: usize) -> Vec<Seat> {
        assert!(
            x < self.rows && y < self.cols,
            "Index ({}, {}) is out of range (0-{}, 0-{}).",
            x,
            y,
            self.rows,
            self.cols
        );
        let mut neighbors: Vec<Seat> = Vec::new();
        for (x_step, y_step) in itertools::iproduct!(-1..1, -1..1).filter(|&p| p != (0, 0)) {
            let mut row = (x as isize) + x_step;
            let mut col = (y as isize) + y_step;
            while row >= 0 && row < self.rows as isize && col >= 0 && col < self.cols as isize {
                let s = self.seats[row as usize][col as usize];
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

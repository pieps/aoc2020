use crate::day11::seat::Seat;

use std::fmt::{self, Debug, Display, Formatter};

#[derive(Clone, PartialEq, Eq)]
pub struct Layout {
    pub seats: Vec<Vec<Seat>>,
    pub rows: usize,
    pub cols: usize,
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

    pub fn count_occupied(&self) -> usize {
        self.seats
            .iter()
            .flat_map(|r| r.iter().filter(|s| s.is_occupied()))
            .count()
    }
}

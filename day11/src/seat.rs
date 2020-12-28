use std::fmt::{self, Debug, Display, Formatter};

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Seat {
    Floor,
    Empty,
    Occupied,
}

impl Seat {
    pub fn parse(c: char) -> Seat {
        match c {
            '.' => Seat::Floor,
            'L' => Seat::Empty,
            '#' => Seat::Occupied,
            _ => panic!("Invalid state {}. Expected one of [., L, #]", c),
        }
    }

    pub const fn is_occupied(self) -> bool {
        match self {
            Seat::Occupied => true,
            _ => false,
        }
    }
}

impl Display for Seat {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let disp = match self {
            Seat::Floor => '.',
            Seat::Empty => 'L',
            Seat::Occupied => '#',
        };
        write!(f, "{}", disp)
    }
}

impl Debug for Seat {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(self, f)
    }
}

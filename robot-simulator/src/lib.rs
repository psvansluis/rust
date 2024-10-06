use std::ops::{Add, Sub};

// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.
use Direction::*;

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    x: i32,
    y: i32,
    d: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self { x, y, d }
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        Self {
            d: match self.d {
                North => East,
                East => South,
                South => West,
                West => North,
            },
            ..self
        }
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        self.turn_right().turn_right().turn_right()
    }

    #[must_use]
    pub fn advance(self) -> Self {
        let next: fn(i32) -> i32 = match self.d {
            North | East => |n| n + 1,
            _ => |n| n - 1,
        };

        match self.d {
            North | South => Self {
                y: next(self.y),
                ..self
            },
            _ => Self {
                x: next(self.x),
                ..self
            },
        }
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        instructions.chars().fold(self, |r, ch| match ch {
            'A' => r.advance(),
            'L' => r.turn_left(),
            'R' => r.turn_right(),
            _ => r,
        })
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}

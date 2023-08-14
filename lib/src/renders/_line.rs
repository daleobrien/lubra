use crate::renders::_point::Point;
use std::fmt;
use std::ops::{Add, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Line {
    pub start: Point,
    pub delta: Point,
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "M {} {} l {} {} z ",
            self.start.x, self.start.y, self.delta.x, self.delta.y
        )
    }
}

impl Add<Point> for Line {
    type Output = Self;

    fn add(self, other: Point) -> Self {
        Self {
            start: Point {
                x: self.start.x + other.x,
                y: self.start.y + other.y,
            },
            delta: self.delta,
        }
    }
}

impl Sub<Point> for Line {
    type Output = Self;

    fn sub(self, other: Point) -> Self {
        Self {
            start: Point {
                x: self.start.x - other.x,
                y: self.start.y - other.y,
            },
            delta: self.delta,
        }
    }
}

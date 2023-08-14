use crate::renders::_point::Point;
use crate::renders::svg::Path;
use std::fmt;
use std::ops::{Add, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Rect {
    pub origin: Point,
    pub delta1: Point,
    pub delta2: Point,
    pub delta3: Point,
}

impl fmt::Display for Rect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "M {} {} l {} {} l {} {} l {} {} z m",
            self.origin.x,
            self.origin.y,
            self.delta1.x,
            self.delta1.y,
            self.delta2.x,
            self.delta2.y,
            self.delta3.x,
            self.delta3.y
        )
    }
}

impl Add<Point> for Rect {
    type Output = Self;

    fn add(self, other: Point) -> Self {
        Self {
            origin: Point {
                x: self.origin.x + other.x,
                y: self.origin.y + other.y,
            },
            delta1: self.delta1,
            delta2: self.delta2,
            delta3: self.delta3,
        }
    }
}

impl Sub<Point> for Rect {
    type Output = Self;

    fn sub(self, other: Point) -> Self {
        Self {
            origin: Point {
                x: self.origin.x - other.x,
                y: self.origin.y - other.y,
            },
            delta1: self.delta1,
            delta2: self.delta2,
            delta3: self.delta3,
        }
    }
}

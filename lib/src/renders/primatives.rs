use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Line {
    pub start: Point,
    pub delta: Point,
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

pub struct Coordinate {
    pub i: u32,
    pub j: u32,
}

pub struct Segment {
    pub centre: Point,
    pub radius: f64,
    pub segment: u64, // Can be much smaller, but now this structure is alligned
}

impl Add<Point> for Segment {
    type Output = Self;

    fn add(self, other: Point) -> Self {
        Self {
            centre: Point {
                x: self.centre.x + other.x,
                y: self.centre.y + other.y,
            },
            radius: self.radius,
            segment: self.segment,
        }
    }
}

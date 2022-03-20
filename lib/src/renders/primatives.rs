use std::ops::{Add, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Radius {
    pub r: f64,
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

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
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

pub struct Coordinate {
    pub i: u32,
    pub j: u32,
}

pub struct SegmentOffset {
    pub offset: u64,
}

#[derive(Clone, Copy)]
pub struct Segment {
    pub centre: Point,
    pub radius: f64,
    pub segment: u64, // Can be much smaller, but now this structure is aligned
}

impl Add<SegmentOffset> for Segment {
    type Output = Self;
    fn add(self, other: SegmentOffset) -> Self {
        Self {
            centre: Point {
                x: self.centre.x,
                y: self.centre.y,
            },
            radius: self.radius,
            segment: self.segment + other.offset,
        }
    }
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

impl Add<Radius> for Segment {
    type Output = Self;

    fn add(self, other: Radius) -> Self {
        Self {
            radius: self.radius + other.r,
            centre: Point {
                x: self.centre.x,
                y: self.centre.y,
            },
            segment: self.segment,
        }
    }
}

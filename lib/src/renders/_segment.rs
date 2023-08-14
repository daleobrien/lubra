use crate::renders::_point::{Point, Radius};
use crate::renders::math::polar;
use std::fmt;
use std::ops::Add;

const PI_6: f64 = 2.0 * std::f64::consts::FRAC_PI_6;

const LARGE_ANGLES_FROM_SEGMENT: [[f64; 2]; 7] = [
    [0f64, 0f64],
    [0.0 * PI_6, 3.0 * PI_6],
    [1.0 * PI_6, 4.0 * PI_6],
    [2.0 * PI_6, 5.0 * PI_6],
    [3.0 * PI_6, 6.0 * PI_6],
    [4.0 * PI_6, 7.0 * PI_6],
    [5.0 * PI_6, 8.0 * PI_6],
];

pub struct SegmentOffset {
    pub offset: usize,
}

#[derive(Clone, Copy)]
pub struct Segment {
    pub centre: Point,
    pub radius: f64,
    pub segment: usize,
}

impl fmt::Display for Segment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ccw = "1";
        let x = self.centre.x;
        let y = self.centre.y;
        let r = self.radius;
        let segment = self.segment;

        // assert!(segment <= 6 && segment > 0);

        let [start, end] = LARGE_ANGLES_FROM_SEGMENT[&segment % 7];
        let (a, b) = polar(x, y, r, start);
        let (c, d) = polar(x, y, r, end);
        write!(
            f,
            "M {a} {b} A {r} {r} 0 0 {ccw} {c} {d} ",
            a = a,
            b = b,
            c = c,
            d = d,
            r = r,
            ccw = ccw,
        )
    }
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

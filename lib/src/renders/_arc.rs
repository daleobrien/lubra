use crate::renders::_point::{Point, Radius};
use crate::renders::math::polar;
use crate::renders::strokes::SMALL_ANGLE;
use std::fmt;
use std::ops::Add;

const PI_6: f64 = 2.0 * std::f64::consts::FRAC_PI_6;

const SMALL_ANGLES_FROM_SEGMENT: [[f64; 2]; 19] = [
    [0f64, 0f64],             //  0
    [0.0 * PI_6, 1.0 * PI_6], //  1
    [1.0 * PI_6, 2.0 * PI_6], //  2
    [2.0 * PI_6, 3.0 * PI_6], //  3
    [3.0 * PI_6, 4.0 * PI_6], //  4
    [4.0 * PI_6, 5.0 * PI_6], //  5
    [5.0 * PI_6, 6.0 * PI_6], //  6
    [0.0 * PI_6, 1.0 * PI_6], //  7
    [1.0 * PI_6, 2.0 * PI_6], //  8
    [2.0 * PI_6, 3.0 * PI_6], //  9
    [3.0 * PI_6, 4.0 * PI_6], // 10
    [4.0 * PI_6, 5.0 * PI_6], // 11
    [5.0 * PI_6, 6.0 * PI_6], // 12
    [0.0 * PI_6, 1.0 * PI_6], // 13
    [1.0 * PI_6, 2.0 * PI_6], // 14
    [2.0 * PI_6, 3.0 * PI_6], // 15
    [3.0 * PI_6, 4.0 * PI_6], // 16
    [4.0 * PI_6, 5.0 * PI_6], // 17
    [5.0 * PI_6, 6.0 * PI_6], // 18  (19 items)
];

pub struct ArcOffset {
    pub offset: usize,
}

pub struct Arc {
    pub centre: Point,
    pub radius: f64,
    pub segment: usize,
}

impl fmt::Display for Arc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ccw = "1";
        let x = self.centre.x;
        let y = self.centre.y;
        let r = self.radius;
        let segment = self.segment;

        let [start, end] = SMALL_ANGLES_FROM_SEGMENT[segment % 19];

        if segment > 12 {
            // indicates two smaller arcs
            let (a, b) = polar(x, y, r, start);
            let (c, d) = polar(x, y, r, start + SMALL_ANGLE);
            let (e, s) = polar(x, y, r, end - SMALL_ANGLE);
            let (g, h) = polar(x, y, r, end);
            write!(
                f,
                "M {a} {b} A {r} {r} 0 0 {ccw} {c} {d} M {e} {s} A {r} {r} 0 0 {ccw} {g} {h}",
                a = a,
                b = b,
                c = c,
                d = d,
                e = e,
                s = s,
                g = g,
                h = h,
                r = r,
                ccw = ccw,
            )
        } else if segment > 6 {
            // indicates a smaller arc centered with what would have been a full size 1/6th arc
            let (a, b) = polar(x, y, r, start + SMALL_ANGLE);
            let (c, d) = polar(x, y, r, end - SMALL_ANGLE);
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
        } else {
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
}

impl Add<ArcOffset> for Arc {
    type Output = Self;
    fn add(self, other: ArcOffset) -> Self {
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

impl Add<Point> for Arc {
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

impl Add<Radius> for Arc {
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

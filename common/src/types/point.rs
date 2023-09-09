use crate::types::format::trimf64;
use std::fmt;
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

pub const S3: f64 = 0.8660254037844386467637231707529362f64; /* sqrt(3)/2 */
pub const S4: f64 = 0.5773502691896257645091487805019575f64; /* 1/sqrt(3) */
pub const S5: f64 = 0.2886751345948128822545743902509787f64; /* 1/(2 sqrt(3)) */

pub const TILE_WIDTH: f64 = 120f64;
pub const TILE_HEIGHT: f64 = 120f64 * S3;
// pub const PATH_WIDTH: f64 = 0.11f64;
pub const PATH_WIDTH: f64 = 0.115f64;

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

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<f64> for Point {
    type Output = Point;

    fn mul(self, scalar: f64) -> Point {
        Point {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl Mul<Point> for f64 {
    type Output = Point;

    fn mul(self, other: Point) -> Point {
        Point {
            x: other.x * self,
            y: other.y * self,
        }
    }
}

impl Point {
    pub fn trim(self) -> String {
        return format!("{} {}", trimf64(self.x), trimf64(self.y));
    }

    fn rotate(&self, invert: bool, index: usize, offset_rotations: Vec<usize>, ccw: bool) -> Point {
        let c = -0.5;
        let s = if invert ^ ccw { S3 } else { -S3 };

        if offset_rotations.contains(&index) {
            let pivot = Point {
                x: 0.5f64,
                y: if invert { S3 - S5 } else { S5 },
            };
            // Centre point of our tile
            let translated = *self - pivot;

            return pivot
                + Point {
                    x: translated.x * c - translated.y * s,
                    y: translated.x * s + translated.y * c,
                };
        }
        // relative rotation, these points are diffs, so don't need a pivot point

        return Point {
            x: self.x * c - self.y * s,
            y: self.x * s + self.y * c,
        };
    }
    pub fn rotate_ccw(&self, invert: bool, index: usize, offset_rotations: Vec<usize>) -> Point {
        // (Sin(120º), Cos(120º))
        return self.rotate(invert, index, offset_rotations, true);
    }

    pub fn rotate_cw(&self, invert: bool, index: usize, offset_rotations: Vec<usize>) -> Point {
        // (Sin(-120º), Cos(-120º))
        return self.rotate(invert, index, offset_rotations, false);
    }

    pub fn invert(&self) -> Point {
        Point {
            x: self.x,
            y: S3 - self.y,
        }
    }
    pub fn reflect(&self) -> Point {
        Point {
            x: 1.0 - self.x,
            y: self.y,
        }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

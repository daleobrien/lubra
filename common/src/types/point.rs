use std::fmt;
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

pub const S3: f64 = 0.8660254037844386467637231707529362f64; /* sqrt(3)/2 */
// pub const S4: f64 = 0.4330127018922193233818615853764681f64; /* sqrt(3)/4 */
pub const S5: f64 = 0.2886751345948128822545743902509787f64; /* 1/2/sqrt(3) */

pub const TILE_WIDTH: f64 = 120f64;
pub const TILE_HEIGHT: f64 = 120f64 * S3;
pub const PATH_WIDTH: f64 = 0.20f64;

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
    fn rotate(&self, index: usize, offset_rotations: Vec<usize>, s: f64, c: f64) -> Point {
        if offset_rotations.contains(&index) {
            // Centre point of our tile
            let pivot = TILE_WIDTH * Point { x: 0.5f64, y: S5 };
            let translated = *self - pivot;

            return pivot
                + Point {
                    x: translated.x * c - translated.y * s,
                    y: translated.x * s + translated.y * c,
                };
        }

        return Point {
            x: self.x * c - self.y * s,
            y: self.x * s + self.y * c,
        };
    }
    pub fn rotate_ccw(&self, index: usize, offset_rotations: Vec<usize>) -> Point {
        // (Sin(120º), Cos(120º))
        return self.rotate(index, offset_rotations, S3, -0.5);
    }

    pub fn rotate_cw(&self, index: usize, offset_rotations: Vec<usize>) -> Point {
        // (Sin(-120º), Cos(-120º))
        return self.rotate(index, offset_rotations, -S3, -0.5);
    }
}

pub fn diff(points: Vec<Point>, invert: bool) -> Vec<Point> {
    // Convert 4 points into the first point followed by series of differences
    let mut seq = vec![points[0]];

    let diffs = points
        .windows(2)
        .map(|p| {
            TILE_WIDTH
                * if !invert {
                    p[1] - p[0]
                } else {
                    Point {
                        x: p[1].x - p[0].x,
                        y: p[0].y - p[1].y, // diff in the other direction
                    }
                }
        })
        .collect::<Vec<_>>();

    if invert {
        seq[0].y = TILE_WIDTH * (S3 - points[0].y);
    }

    for d in diffs.iter() {
        seq.push(*d);
    }
    return seq;
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

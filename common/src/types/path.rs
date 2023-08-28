use crate::types::point::{Point, TILE_WIDTH};
use itertools::concat;
use std::ops::{Deref, Mul};

pub struct Path {
    points: Vec<Point>,
}

impl Path {
    pub fn new(points: Vec<Point>) -> Path {
        Path { points }
    }

    pub fn diff(self, invert: bool) -> Path {
        // Convert a series of points into the first point followed by series of differences
        let pts: Vec<Point> = self
            .points
            .iter()
            .map(|p| if invert { p.invert() } else { *p })
            .collect();

        let diffs = pts.windows(2).map(|p| (p[1] - p[0])).collect::<Vec<_>>();

        let pts = concat(vec![vec![pts[0]], diffs]);

        return Path { points: pts };
    }

    // pub fn rotate_cw(self, index: usize, offset_rotations: Vec<usize>) -> Path {
    //     Path {
    //         points: self.iter().map(|p| p.rotate_cw(index)).collect(),
    //     }
    // }
}

impl Deref for Path {
    type Target = Vec<Point>;

    fn deref(&self) -> &Self::Target {
        &self.points
    }
}

// Multiply all points in a path by a scalar
impl Mul<f64> for Path {
    type Output = Path;
    fn mul(self, other: f64) -> Self::Output {
        Path {
            points: self.iter().map(|p| *p * other).collect(),
        }
    }
}

impl Mul<Path> for f64 {
    type Output = Path;
    fn mul(self, other: Path) -> Self::Output {
        Path {
            points: other.iter().map(|p| *p * self).collect(),
        }
    }
}

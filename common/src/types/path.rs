use crate::types::point::{Point, TILE_WIDTH};
use itertools::concat;
use std::ops::{Deref, DerefMut, Mul};

#[derive(Debug, Clone)]
pub struct Path {
    points: Vec<Point>,
}

impl Path {
    pub fn new(points: Vec<Point>) -> Path {
        Path { points }
    }

    pub fn scale(self) -> Path {
        return Path {
            points: self.iter().map(|p| *p * TILE_WIDTH).collect(),
        };
    }

    pub fn trim(self) -> Vec<String> {
        return self.iter().map(|p| p.trim()).collect();
    }

    pub fn str(self) -> Vec<String> {
        return self.iter().map(|p| (*p * TILE_WIDTH).trim()).collect();
    }

    pub fn invert(self) -> Path {
        Path {
            points: self.iter().map(|p| p.invert()).collect(),
        }
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

    pub fn rotate_cw(self, invert: bool, offset_rotations: Vec<usize>) -> Path {
        Path {
            points: self
                .iter()
                .enumerate()
                .map(|(index, p)| p.rotate_cw(invert, index, offset_rotations.clone()))
                .collect(),
        }
    }

    pub fn rotate_ccw(self, invert: bool, offset_rotations: Vec<usize>) -> Path {
        Path {
            points: self
                .iter()
                .enumerate()
                .map(|(index, p)| p.rotate_ccw(invert, index, offset_rotations.clone()))
                .collect(),
        }
    }
}

impl Deref for Path {
    type Target = Vec<Point>;

    fn deref(&self) -> &Self::Target {
        &self.points
    }
}

impl DerefMut for Path {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.points
    }
}

impl Default for Path {
    fn default() -> Self {
        Path::new(Vec::new())
    }
}

impl Extend<Point> for Path {
    fn extend<T: IntoIterator<Item = Point>>(&mut self, iter: T) {
        for elem in iter {
            self.points.push(elem);
        }
    }
}

impl IntoIterator for Path {
    type Item = Point;
    type IntoIter = std::vec::IntoIter<Point>;

    fn into_iter(self) -> Self::IntoIter {
        self.points.into_iter()
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

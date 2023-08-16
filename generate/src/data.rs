use std::ops::{Add, Mul, Sub};

#[derive(Debug, Copy, Clone)]
struct Point {
    x: f64,
    y: f64,
}

const SQRT_3: f64 = 1.732050807568877293527446341505872f64;

const P14: Point = Point {
    x: 1f64 / 3f64,
    y: SQRT_3 / 2f64,
};

const P11: Point = Point {
    x: 1f64 / 2f64,
    y: SQRT_3 / 2f64,
};
const P15: Point = Point {
    x: 2f64 / 3f64,
    y: SQRT_3 / 2f64,
};

const P21: Point = Point {
    x: -1f64 / 12f64,
    y: 1f64 / SQRT_3 / 4f64,
};
const P22: Point = Point { x: 0f64, y: 0f64 };
const P23: Point = Point {
    x: 1f64 / 12f64,
    y: -1f64 / SQRT_3 / 4f64,
};

const P35: Point = Point {
    x: 13f64 / 12f64,
    y: 1f64 / SQRT_3 / 4f64,
};
const P33: Point = Point { x: 1f64, y: 0f64 };
const P36: Point = Point {
    x: 11f64 / 12f64,
    y: -1f64 / SQRT_3 / 4f64,
};

const P41: Point = Point {
    x: 1f64 / 3f64,
    y: 1f64 / SQRT_3,
};
const P42: Point = Point {
    x: 1f64 / 6f64,
    y: 1f64 / SQRT_3 / 2f64,
};
const P40: Point = Point {
    x: 1f64 / 4f64,
    y: SQRT_3 / 4f64,
};
const P44: Point = Point {
    x: 0f64,
    y: 1f64 / SQRT_3,
};

const P51: Point = Point {
    x: 2f64 / 3f64,
    y: 1f64 / SQRT_3,
};
const P50: Point = Point {
    x: 3f64 / 4f64,
    y: SQRT_3 / 4f64,
};
const P53: Point = Point {
    x: 6f64 / 6f64,
    y: 1f64 / SQRT_3 / 2f64,
};
const P55: Point = Point {
    x: 1f64,
    y: 1f64 / SQRT_3,
};

const P60: Point = Point {
    x: 0f64,
    y: 1f64 / 2f64,
};
const P62: Point = Point {
    x: 1f64 / 3f64,
    y: 0f64,
};
const P63: Point = Point {
    x: 2f64 / 3f64,
    y: 0f64,
};
const P66: Point = Point {
    x: 0f64,
    y: 1f64 / SQRT_3,
};

const SCALE: f64 = 120f64;
const SVG_FILL: &str = "fill=\"black\" stroke=\"transparent\" stroke-width=\"3\"";
const SVG_LINE: &str = "fill=\"transparent\" stroke=\"black\" fill-opacity=\"0.03\"";

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

// Point * x
impl Mul<f64> for Point {
    type Output = Point;

    fn mul(self, scalar: f64) -> Point {
        Point {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

// x * Point
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
    fn rotate_ccw(&self) -> Point {
        let angle_radians = 120f64.to_radians();

        let s = angle_radians.sin();
        let c = angle_radians.cos();

        // Centre point of our tile
        let pivot = Point {
            x: 0.5f64,
            y: 1f64 / 2f64 / SQRT_3,
        };
        let translated = *self - pivot;

        return pivot
            + Point {
                x: translated.x * c - translated.y * s,
                y: translated.x * s + translated.y * c,
            };
    }

    fn rotate_cw(&self) -> Point {
        let angle_radians = -120f64.to_radians();

        let s = angle_radians.sin();
        let c = angle_radians.cos();

        let pivot = Point {
            x: 0.5f64,
            y: 1f64 / 2f64 / SQRT_3,
        };
        let translated = *self - pivot;

        return pivot
            + Point {
                x: translated.x * c - translated.y * s,
                y: translated.x * s + translated.y * c,
            };
    }
}

fn diff(points: Vec<Point>) -> Vec<Point> {
    // Convert 4 points into the first point followed by series of differences
    let mut seq = vec![points[0] * SCALE];

    let diffs = points
        .windows(2)
        .map(|p| SCALE * (p[1] - p[0]))
        .collect::<Vec<_>>();

    for d in diffs.iter() {
        seq.push(*d);
    }
    return seq;
    //let results = first.iter().chain(diffs);
    // return results;
}

fn wrap_format_with_function(name: &str, svg: String) -> String {
    format!(
        "pub fn {}(offset: Point) -> String \n{{\n  writeln!(\"{}\", x=offset.x, y=offset.y)\n}}",
        name, svg
    )
}

fn generate_L1_fn() -> String {
    let d = diff(vec![P14, P62, P63, P15]);
    // relative to the 0,0 of each tile?, or the centre of the tile?
    return format!(
        "<path {line} d=\"M {{x}} {{y}} m {a:.2} {b:.2} l {c:.2} {d:.2} m {e:.2} {f:.2} l {g:.2} {h:.2} z\" />\
         <path {fill} d=\"M {{x}} {{y}} m {a:.2} {b:.2} m {c:.2} {d:.2} m {e:.2} {f:.2} l {g:.2} {h:.2} z\" />\
        ",
        a = d[0].x,
        b = d[0].y,
        c = d[1].x,
        d = d[1].y,
        e = d[2].x,
        f = d[2].y,
        g = d[3].x,
        h = d[3].y,
        line=SVG_LINE,
        fill=SVG_FILL,
    );
}

pub fn generate_all_shapes() {
    let function = wrap_format_with_function("L1", generate_L1_fn());
    println!("{}", function);
}

#[cfg(test)]

mod tests {
    use approx::{assert_ulps_eq, relative_eq, ulps_eq};
    extern crate approx;

    #[test]
    fn test_rotate() {
        use super::*;
        use approx;
        let p = Point { x: 1f64, y: 0f64 };
        let r = p.rotate_ccw();
        assert_ulps_eq!(r.x, 0.5f64);
        assert_ulps_eq!(r.y, SQRT_3 / 2f64);

        let q = r.rotate_cw();
        assert_ulps_eq!(q.x, 1f64);
        assert_ulps_eq!(q.y, 0f64);
    }
}

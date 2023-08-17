use std::ops::{Add, Mul, Sub};

#[derive(Debug, Copy, Clone)]
struct Point {
    x: f64,
    y: f64,
}

const S3: f64 = 0.8660254037844386467637231707529362f64; // sqrt(3)/2
const S4: f64 = 0.4330127018922193233818615853764681f64; // sqrt(3)/4
const S5: f64 = 0.2886751345948128822545743902509787f64; // 1/2/sqrt(3)

const P1: Point = Point { x: 0.5, y: S3 };
const P2: Point = Point { x: 0.0, y: 0.0 };
const P3: Point = Point { x: 1.1, y: 0.0 };
const P4: Point = Point { x: 0.25, y: S4 };
const P5: Point = Point { x: 0.75, y: S4 };
const P6: Point = Point { x: 0.5, y: 0.0 };

const STROKE_WIDTH: f64 = 3f64;
const SCALE: f64 = 120f64;
const SVG_FILL: &str = "stroke=\"transparent\"";
const SVG_LINE: &str = "fill=\"transparent\"";
const PATH_WIDTH: f64 = 0.05f64;

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
            let pivot = SCALE * Point { x: 0.5f64, y: S5 };
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
    fn rotate_ccw(&self, index: usize, offset_rotations: Vec<usize>) -> Point {
        // (Sin(120º), Cos(120º))
        return self.rotate(index, offset_rotations, S3, -0.5);
    }

    fn rotate_cw(&self, index: usize, offset_rotations: Vec<usize>) -> Point {
        // (Sin(-120º), Cos(-120º))
        return self.rotate(index, offset_rotations, -S3, -0.5);
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

fn generate_Lx() -> Vec<Point> {
    let w = Point {
        x: PATH_WIDTH,
        y: 0f64,
    };
    let mut d = diff(vec![P1 - w, P6 - w, P6 + w, P1 + w]);

    d.push(
        d[0] + Point {
            x: STROKE_WIDTH / 2f64,
            y: 0f64,
        },
    );
    d.push(
        d[2] - Point {
            x: STROKE_WIDTH,
            y: 0f64,
        },
    );
    return d;
}

fn format_rectangle(p: Vec<Point>) -> String {
    // TODO: Put {{x}} {{y}} back in
    // TODO: Format positive number prefix with a space, negative with a minus, then I can remove some space
    // Maybe make my own format function, can also remove trailing zeros
    return format!(
        "<path {line} d=\"M0 0 m{a:.4} {b:.4} l{c:.4} {d:.4}m {e:.4} {f:.4} l{g:.4} {h:.4}z\" />\n\
     <path {fill} d=\"M0 0 m{ai:.4} {bi:.4} l{ci:.4} {d:.4}l {ei:.4} {fi:.4} l{g:.4} {h:.4}z\" />",
        a = p[0].x,
        ai = p[4].x,
        b = p[0].y,
        bi = p[4].y,
        c = p[1].x,
        ci = p[1].x,
        d = p[1].y,
        e = p[2].x,
        ei = p[5].x,
        f = p[2].y,
        fi = p[5].y,
        g = p[3].x,
        h = p[3].y,
        line = SVG_LINE,
        fill = SVG_FILL,
    );
}

fn generate_L16_fn() -> String {
    return format_rectangle(generate_Lx());
}

fn generate_L25_fn() -> String {
    // Only the first points is rotated about the center, the rest are about (0,0) since they are relative moves
    let pts = generate_Lx()
        .iter()
        .enumerate()
        .map(|(i, p)| p.rotate_cw(i, vec![0, 4]))
        .collect::<Vec<_>>();
    return format_rectangle(pts);
}

fn generate_L34_fn() -> String {
    // Only the first points is rotated about the center, the rest are about (0,0) since they are relative moves
    let pts = generate_Lx()
        .iter()
        .enumerate()
        .map(|(i, p)| p.rotate_ccw(i, vec![0, 4]))
        .collect::<Vec<_>>();
    return format_rectangle(pts);
}

pub fn generate_all_shapes() {
    //println!("{}", wrap_format_with_function("L1", generate_L1_fn()));
    //println!("{}", wrap_format_with_function("L2", generate_L2_fn()));
    println!("{}", generate_L16_fn());
    println!("{}", generate_L25_fn());
    println!("{}", generate_L34_fn());
}

#[cfg(test)]
mod tests {
    use approx::assert_ulps_eq;

    extern crate approx;

    #[test]
    fn test_rotate() {
        use super::*;
        let p = SCALE * Point { x: 1f64, y: 0f64 };
        let r = p.rotate_ccw(0, vec![0]);
        assert_ulps_eq!(r.x, SCALE * 0.5f64);
        assert_ulps_eq!(r.y, SCALE * SQRT_3 / 2f64);

        let q = r.rotate_cw(0, vec![0]);
        assert_ulps_eq!(q.x, SCALE * 1f64);
        assert_ulps_eq!(q.y, SCALE * 0f64, epsilon = 0.0000000000001f64);
    }
}

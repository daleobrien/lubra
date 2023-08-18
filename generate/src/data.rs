use std::f64::consts::FRAC_PI_6;
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Copy, Clone)]
struct Point {
    x: f64,
    y: f64,
}

const S3: f64 = 0.8660254037844386467637231707529362f64; /* sqrt(3)/2 */
const S4: f64 = 0.4330127018922193233818615853764681f64; /* sqrt(3)/4 */
const S5: f64 = 0.2886751345948128822545743902509787f64; /* 1/2/sqrt(3) */

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
const PATH_WIDTH: f64 = 0.10f64;

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
    return diff(vec![P1 - w, P6 - w, P6 + w, P1 + w]);
}

fn trimf64(num: f64) -> String {
    let space = if num < 0f64 { "" } else { " " };
    let formatted = format!("{space}{:.*}", 4, num);
    let trimmed = formatted.trim_end_matches('0').trim_end_matches('.');
    return trimmed.to_string();
}

fn trim(point: Point) -> String {
    return format!("{} {}", trimf64(point.x), trimf64(point.y));
}

fn format_rectangle(pts: Vec<Point>) -> String {
    // TODO: Put {{x}} {{y}} back in
    let s = pts.iter().map(|p| trim(*p)).collect::<Vec<_>>();
    return format!(
    "<path {SVG_FILL} d=\"M0 0m{a}l{b}l{c}l{d}z\"/><path {SVG_LINE} d=\"M0 0m{a}l{b}m{c}l{d}z\"/>\n",
    a = s[0],
    b = s[1],
    c = s[2],
    d = s[3],
  );
}

fn generate_L16_fn() -> String {
    return format_rectangle(generate_Lx());
}

fn generate_L25_fn() -> String {
    let pts = generate_Lx()
        .iter()
        .enumerate()
        .map(|(i, p)| p.rotate_cw(i, vec![0, 4]))
        .collect::<Vec<_>>();
    return format_rectangle(pts);
}

fn generate_L34_fn() -> String {
    let pts = generate_Lx()
        .iter()
        .enumerate()
        .map(|(i, p)| p.rotate_ccw(i, vec![0, 4]))
        .collect::<Vec<_>>();
    return format_rectangle(pts);
}

pub fn polar(x: f64, y: f64, radius: f64, angle: f64) -> (f64, f64) {
    let (a, b) = f64::sin_cos(angle);
    return (x + (radius * b), y + (radius * a));
}

fn generate_A1_fn() -> String {
    let [start, end] = [12.0 * FRAC_PI_6, 18.0 * FRAC_PI_6];
    let [x, y] = [0.5 * SCALE, S3 * SCALE];
    let r = PATH_WIDTH * SCALE;
    let (a, b) = polar(x, y, r, start);
    let (c, d) = polar(x, y, r, end);
    // TODO: rotate ...
    return format!(
        "<path d=\"M{a}A{r} 0 0 {ccw} {b}\" />",
        a = trim(Point { x: a, y: b }),
        b = trim(Point { x: c, y: d }),
        r = trim(Point { x: r, y: r }),
        ccw = "0",
    );
}

pub fn generate_all_shapes() {
    //println!("{}", wrap_format_with_function("L1", generate_L1_fn()));
    //println!("{}", wrap_format_with_function("L2", generate_L2_fn()));
    println!("{}", generate_L16_fn());
    println!("{}", generate_L25_fn());
    println!("{}", generate_L34_fn());
    println!("{}", generate_A1_fn());
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

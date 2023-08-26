use common::types::point::{diff, Point, PATH_WIDTH, S3, TILE_WIDTH};
use std::collections::BTreeMap;
use std::f64::consts::FRAC_PI_6;
use std::fs::File;
use std::io::Write;

fn generate_lx(invert: bool) -> Vec<Point> {
    let w = Point {
        x: PATH_WIDTH,
        y: 0f64,
    };
    let a = Point { x: 0.5, y: S3 };
    let b = Point { x: 0.5, y: 0.0 };
    let points = vec![a - w, b - w, b + w, a + w];
    return diff(points, invert);
}

fn walls(invert: bool) -> Vec<Point> {
    let a = Point { x: 0.0, y: 0.0 };
    let b = Point { x: 0.5, y: S3 };
    let c = Point { x: 1.0, y: 0.0 };
    let points = vec![a, b, c];
    return diff(points, invert);
}

fn l_wall(invert: bool) -> String {
    let pts = walls(invert);
    let s = pts.iter().map(|p| trim(*p)).collect::<Vec<_>>();
    return format!(
        "<path fill=\\\"transparent\\\" stroke=\\\"red\\\" d=\\\"M{a}l{b}l{c}z \\\"/>",
        a = s[0],
        b = s[1],
        c = s[2]
    );
}

fn trimf64(num: f64) -> String {
    let space = if num < 0f64 { "" } else { " " };
    let formatted = format!("{space}{:.*}", 2, num);
    let trimmed = formatted.trim_end_matches('0').trim_end_matches('.');
    return trimmed.to_string();
}

fn trim(point: Point) -> String {
    return format!("{} {}", trimf64(point.x), trimf64(point.y));
}

fn l_a_to_b(pts: Vec<Point>) -> String {
    let s = pts.iter().map(|p| trim(*p)).collect::<Vec<_>>();
    return format!(
        "<path d=\\\"M{a}l{b}l{c}l{d}z \\\"/> \
         <path  fill=\\\"transparent\\\" d=\\\"M{a}l{b}m{c}l{d}z\\\"/>",
        a = s[0],
        b = s[1],
        c = s[2],
        d = s[3]
    );
}

fn l1_to_6(invert: bool) -> String {
    return l_a_to_b(generate_lx(invert));
}

fn l2_to_5(invert: bool) -> String {
    let pts = generate_lx(invert)
        .iter()
        .enumerate()
        .map(|(i, p)| p.rotate_cw(i, vec![0, 4]))
        .collect::<Vec<_>>();
    return l_a_to_b(pts);
}

fn l3_to_4(invert: bool) -> String {
    let pts = generate_lx(invert)
        .iter()
        .enumerate()
        .map(|(i, p)| p.rotate_ccw(i, vec![0, 4]))
        .collect::<Vec<_>>();
    return l_a_to_b(pts);
}

pub fn polar(p: Point, radius: f64, angle: f64) -> Point {
    let (a, b) = f64::sin_cos(angle);
    return Point {
        x: p.x + radius * b,
        y: p.y + radius * a,
    };
}

fn ax(index: usize, invert: bool) -> [Point; 3] {
    //

    let h1 = if invert { 0.0 } else { S3 * TILE_WIDTH };
    let h2 = if invert { S3 * TILE_WIDTH } else { 0.0 };

    let [start, end, r, y] = match index {
        0 => [
            12.0 * FRAC_PI_6,
            18.0 * FRAC_PI_6,
            PATH_WIDTH * TILE_WIDTH,
            h1,
        ],
        1 => [
            18.0 * FRAC_PI_6,
            12.0 * FRAC_PI_6,
            PATH_WIDTH * TILE_WIDTH,
            h2,
        ],
        2 => [
            8.0 * FRAC_PI_6,
            10.0 * FRAC_PI_6,
            (0.5 - PATH_WIDTH) * TILE_WIDTH,
            h1,
        ],
        3 => [
            10.0 * FRAC_PI_6,
            8.0 * FRAC_PI_6,
            (0.5 + PATH_WIDTH) * TILE_WIDTH,
            h1,
        ],
        _ => panic!("Invalid index"),
    };
    let x = 0.5 * TILE_WIDTH;
    let a = polar(Point { x, y }, r, start);
    let b = polar(Point { x, y }, r, end);
    return [a, b, Point { x: r, y: r }];
}

fn a1_or_6(index: usize, invert: bool) -> String {
    let [a, b, r] = ax(index, invert).map(|p| trim(p));
    return format!("<path d=\\\"M {a} A {r} 0 0 0 {b}\\\" />");
}

fn a1(invert: bool) -> String {
    return a1_or_6(0, invert);
}

fn a6(invert: bool) -> String {
    return a1_or_6(1, invert);
}

fn a4_to_5(invert: bool) -> String {
    // With this and others, I don't need to draw a shape, just a really fat line!
    let [a, b, r1] = ax(2, invert);
    let [c, d, r2] = ax(3, invert);
    let [a, b, c, d, r1, r2] = [a, b, c, d, r1, r2].map(|p| trim(p));

    return format!(
        "<path d=\\\"M {a} A {r1} 0 0 1 {b} L {c} A {r2} 0 0 0 {d} z \\\" /> \
         <path  fill=\\\"transparent\\\" d=\\\"M {a} A {r1} 0 0 1 {b} M {c} A {r2} 0 0 0 {d} \\\" />"
    );
}

fn a4_to_6(invert: bool) -> String {
    // With this and others, I don't need to draw a shape, just a really fat line!
    let [a, b, r1] = ax(2, invert);
    let [c, d, r2] = ax(3, invert);
    let [a, b, c, d] = [a, b, c, d].map(|p| p.rotate_cw(0, vec![0]));
    let [a, b, c, d, r1, r2] = [a, b, c, d, r1, r2].map(|p| trim(p));

    return format!(
        "<path d=\\\"M {a} A {r1} 0 0 1  {b} L {c} A {r2} 0 0 0  {d} z \\\"/> \
         <path  fill=\\\"transparent\\\" d=\\\"M {a} A {r1} 0 0 1  {b} M {c} A {r2} 0 0 0  {d} \\\"/>"
    );
}

fn a5_to_6(invert: bool) -> String {
    // With this and others, I don't need to draw a shape, just a really fat line!
    let [a, b, r1] = ax(2, invert);
    let [c, d, r2] = ax(3, invert);
    let [a, b, c, d] = [a, b, c, d].map(|p| p.rotate_ccw(0, vec![0]));
    let [a, b, c, d, r1, r2] = [a, b, c, d, r1, r2].map(|p| trim(p));

    return format!(
        "<path d=\\\"M {a} A {r1} 0 0 1 {b} L {c} A {r2} 0 0 0  {d} z\\\" /> \
         <path fill=\\\"transparent\\\" d=\\\"M {a} A {r1} 0 0 1 {b} M {c} A {r2} 0 0 0 {d}\\\" />"
    );
}

fn a2_or_5(index: usize, invert: bool) -> String {
    let [a, b, r] = ax(index, invert);
    let [a, b] = [a, b].map(|p| p.rotate_cw(0, vec![0]));
    let [a, b, r] = [a, b, r].map(|p| trim(p));
    return format!("<path d=\\\"M {a} A {r} 0 0 0 {b}\\\" />");
}
fn a2(invert: bool) -> String {
    return a2_or_5(0, invert);
}
fn a4(invert: bool) -> String {
    return a2_or_5(1, invert);
}

fn a3_or_5(index: usize, invert: bool) -> String {
    let [a, b, r] = ax(index, invert);
    let [a, b] = [a, b].map(|p| p.rotate_ccw(0, vec![0]));
    let [a, b, r] = [a, b, r].map(|p| trim(p));
    return format!("<path d=\\\"M{a}A{r} 0 0 0{b}\\\"/>");
}
fn a3(invert: bool) -> String {
    return a3_or_5(0usize, invert);
}
fn a5(invert: bool) -> String {
    return a3_or_5(1usize, invert);
}

pub fn generate_all_shapes() {
    let mut paths: BTreeMap<usize, String> = BTreeMap::new();

    for (key, path) in [
        (0b00_100_001, l1_to_6(false)),
        (0b01_100_001, l1_to_6(true)),
        (0b00_010_010, l2_to_5(false)),
        (0b01_010_010, l2_to_5(true)),
        (0b00_001_100, l3_to_4(false)),
        (0b01_001_100, l3_to_4(true)),
        (0b00_000_001, a1(false)),
        (0b01_000_001, a1(true)),
        (0b00_000_010, a2(false)),
        (0b00_000_100, a3(false)),
        (0b00_001_000, a4(false)),
        (0b00_010_000, a5(false)),
        (0b00_100_000, a6(false)),
        (0b00_011_000, a4_to_5(false)),
        (0b00_010_100, a4_to_6(false)),
        (0b00_110_000, a5_to_6(false)),
        (0b10_000_000, l_wall(false)),
        (0b11_000_000, l_wall(true)),
    ] {
        paths.insert(key, path);
    }

    // Change this to a look up function rather than a class with a HashMap.
    let mut file = File::create("../lib/src/renders/fragment.rs").expect("File Creation failed");
    let _ = file.write(b"/* This file is generated. */\n");
    let _ = file.write(b"use ::phf::{Map, phf_map};\n\n");
    let _ = file.write(b"pub static MAP_PATHS: Map<u8, &'static str> = phf_map! {\n");
    for (key, value) in paths {
        let _ = write!(file, "  {}u8 =>  \"{}\",\n", key, value);
    }
    let _ = file.write(b"};\n\n");

    let _ = file.flush();
}

#[cfg(test)]
mod tests {
    use approx::assert_ulps_eq;
    use common::types::point::{Point, TILE_WIDTH};
    use std::vec;

    extern crate approx;

    #[test]
    fn test_rotate() {
        use super::*;
        let p = TILE_WIDTH * Point { x: 1f64, y: 0f64 };
        let r = p.rotate_ccw(0, vec![0]);
        assert_ulps_eq!(r.x, TILE_WIDTH * 0.5f64);
        assert_ulps_eq!(r.y, TILE_WIDTH * S3);

        let q = r.rotate_cw(0, vec![0]);
        assert_ulps_eq!(q.x, TILE_WIDTH * 1f64);
        assert_ulps_eq!(q.y, TILE_WIDTH * 0f64, epsilon = 0.0000000000001f64);
    }
}

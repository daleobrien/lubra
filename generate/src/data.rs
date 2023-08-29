use common::types::path::Path;
use common::types::point::{Point, PATH_WIDTH, S3, TILE_WIDTH};
use itertools::concat;
use std::collections::BTreeMap;
use std::f64::consts::FRAC_PI_6;
use std::fs::File;
use std::io::Write;

fn generate_lx(invert: bool) -> Path {
    let w = Point {
        x: PATH_WIDTH,
        y: 0f64,
    };
    let a = Point { x: 0.5, y: S3 };
    let b = Point { x: 0.5, y: 0.0 };

    return Path::new(vec![a - w, b - w, b + w, a + w]).diff(invert);
}

fn walls(invert: bool) -> Path {
    let a = Point { x: 0.0, y: 0.0 };
    let b = Point { x: 0.5, y: S3 };
    let c = Point { x: 1.0, y: 0.0 };
    TILE_WIDTH * Path::new(vec![a, b, c]).diff(invert)
}

fn l_wall(invert: bool) -> String {
    let pts = walls(invert);
    let s = pts.iter().map(|p| trim(*p)).collect::<Vec<_>>();

    let a = TILE_WIDTH
        * Point {
            x: 0.0,
            y: if invert { S3 } else { 0.0 },
        };
    let b = TILE_WIDTH
        * Point {
            x: 0.5,
            y: if invert { 0.0 } else { S3 },
        };
    let c = TILE_WIDTH
        * Point {
            x: 1.0,
            y: if invert { S3 } else { 0.0 },
        };

    let colour = if invert { "red" } else { "green" };
    return format!(
        "<path stroke=\\\"{colour}\\\" d=\\\"M{a}l{b}l{c}z \\\"/> \
        <text stroke=\\\"{colour}\\\" x=\\\"{ax}\\\" y=\\\"{ay}\\\">1</text>\
        <text stroke=\\\"{colour}\\\" x=\\\"{bx}\\\" y=\\\"{by}\\\">2</text>\
        <text stroke=\\\"{colour}\\\" x=\\\"{cx}\\\" y=\\\"{cy}\\\">3</text>\
        ",
        colour = colour,
        a = s[0],
        b = s[1],
        c = s[2],
        ax = a.x,
        ay = a.y,
        bx = b.x,
        by = b.y,
        cx = c.x,
        cy = c.y,
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

fn l_a_to_b(path: Path) -> String {
    let s = path.iter().map(|p| trim(*p)).collect::<Vec<_>>();
    return format!(
        "<path fill=\\\"black\\\" d=\\\"M{a}l{b}l{c}l{d}z \\\"/> \
         <path stroke=\\\"black\\\" d=\\\"M{a}l{b}m{c}l{d}z\\\"/>",
        a = s[0],
        b = s[1],
        c = s[2],
        d = s[3]
    );
}

fn l1_to_6(invert: bool) -> String {
    return l_a_to_b(TILE_WIDTH * generate_lx(invert));
}

fn l2_to_5(invert: bool) -> String {
    let pts = TILE_WIDTH * generate_lx(invert).rotate_cw(invert, vec![0, 4]);
    return l_a_to_b(pts);
}

fn l3_to_4(invert: bool) -> String {
    let pts = TILE_WIDTH * generate_lx(invert).rotate_ccw(invert, vec![0, 4]);
    return l_a_to_b(pts);
}

pub fn polar(p: Point, radius: f64, angle: f64) -> Point {
    let (a, b) = f64::sin_cos(angle);
    return Point {
        x: p.x + radius * b,
        y: p.y + radius * a,
    };
}

fn ax(index: usize, invert: bool) -> Path {
    //

    let [start, end, r, y] = match (invert, index) {
        (false, 0) => [12.0 * FRAC_PI_6, 18.0 * FRAC_PI_6, PATH_WIDTH, S3],
        (true, 0) => [18.0 * FRAC_PI_6, 12.0 * FRAC_PI_6, PATH_WIDTH, 0.0],
        (false, 1) => [18.0 * FRAC_PI_6, 12.0 * FRAC_PI_6, PATH_WIDTH, 0.0],
        (true, 1) => [18.0 * FRAC_PI_6, 12.0 * FRAC_PI_6, PATH_WIDTH, 0.0],
        (false, 2) => [8.0 * FRAC_PI_6, 10.0 * FRAC_PI_6, (0.5 - PATH_WIDTH), S3],
        (true, 2) => [
            8.0 * FRAC_PI_6,
            10.0 * FRAC_PI_6,
            (0.5 - PATH_WIDTH),
            S3 * TILE_WIDTH,
        ],
        (false, 3) => [10.0 * FRAC_PI_6, 8.0 * FRAC_PI_6, (0.5 + PATH_WIDTH), S3],
        (true, 3) => [10.0 * FRAC_PI_6, 8.0 * FRAC_PI_6, (0.5 + PATH_WIDTH), S3],
        _ => panic!("Invalid index"),
    };
    let x = 0.5;
    let a = polar(Point { x, y }, r, start);
    let b = polar(Point { x, y }, r, end);
    return Path::new(vec![a, b, Point { x: r, y: r }]);
}

fn a1_or_6(index: usize, invert: bool) -> String {
    let pts: Vec<String> = ax(index, invert)
        .iter()
        .map(|p| trim(*p * TILE_WIDTH))
        .collect();
    return format!(
        "<path stroke=\\\"black\\\" fill=\\\"black\\\" d=\\\"M {a} A {r} 0 0 0 {b}\\\" />",
        a = pts[0],
        b = pts[1],
        r = pts[2]
    );
}

fn a1(invert: bool) -> String {
    return a1_or_6(0, invert);
}

fn a6(invert: bool) -> String {
    return a1_or_6(1, invert);
}

fn a4_to_5(invert: bool) -> String {
    // With this and others, I don't need to draw a shape, just a really fat line!

    let pts = concat(vec![ax(2, invert), ax(3, invert)]);
    let pts: Vec<String> = pts.iter().map(|p| trim(*p * TILE_WIDTH)).collect();

    return format!(
        "<path fill=\\\"black\\\" d=\\\"M {a} A {r1} 0 0 1 {b} L {c} A {r2} 0 0 0 {d} z \\\" /> \
         <path stroke=\\\"black\\\" d=\\\"M {a} A {r1} 0 0 1 {b} M {c} A {r2} 0 0 0 {d} \\\" />",
        a = pts[0],
        b = pts[1],
        r1 = pts[2],
        c = pts[3],
        d = pts[4],
        r2 = pts[5]
    );
}

fn a4_to_6(invert: bool) -> String {
    let mut arc1 = ax(2, invert);
    let r1 = arc1.pop().unwrap();
    let mut arc2 = ax(3, invert);
    let r2 = arc2.pop().unwrap();
    let pts = concat(vec![arc1, arc2]).rotate_cw(invert, vec![0]);

    let pts: Vec<String> = concat(vec![pts, Path::new(vec![r1, r2])])
        .iter()
        .map(|p| trim(*p * TILE_WIDTH))
        .collect();

    return format!(
        "<path fill=\\\"black\\\" d=\\\"M {a} A {r1} 0 0 1  {b} L {c} A {r2} 0 0 0  {d} z \\\"/> \
         <path stroke=\\\"black\\\" d=\\\"M {a} A {r1} 0 0 1  {b} M {c} A {r2} 0 0 0  {d} \\\"/>",
        a = pts[0],
        b = pts[1],
        c = pts[2],
        d = pts[3],
        r1 = pts[4],
        r2 = pts[5],
    );
}

fn a5_to_6(invert: bool) -> String {
    let mut arc1 = ax(2, invert);
    let r1 = arc1.pop().unwrap();
    let mut arc2 = ax(3, invert);
    let r2 = arc2.pop().unwrap();
    let pts = concat(vec![arc1, arc2]).rotate_ccw(invert, vec![0]);

    let pts: Vec<String> = concat(vec![pts, Path::new(vec![r1, r2])])
        .iter()
        .map(|p| trim(*p * TILE_WIDTH))
        .collect();

    return format!(
        "<path fill=\\\"black\\\" d=\\\"M {a} A {r1} 0 0 1  {b} L {c} A {r2} 0 0 0  {d} z \\\"/> \
         <path stroke=\\\"black\\\" d=\\\"M {a} A {r1} 0 0 1  {b} M {c} A {r2} 0 0 0  {d} \\\"/>",
        a = pts[0],
        b = pts[1],
        c = pts[2],
        d = pts[3],
        r1 = pts[4],
        r2 = pts[5],
    );
}

fn a2_or_5(index: usize, invert: bool) -> String {
    let mut pts = ax(index, invert);
    let r = pts.pop().unwrap();
    let pts = pts.rotate_cw(invert, vec![0]);
    let pts: Vec<String> = concat(vec![pts, Path::new(vec![r])])
        .iter()
        .map(|p| trim(*p * TILE_WIDTH))
        .collect();

    return format!(
        "<path fill=\\\"black\\\" stroke=\\\"black\\\" d=\\\"M {a} A {r} 0 0 0 {b}\\\" />",
        a = pts[0],
        b = pts[1],
        r = pts[2]
    );
}
fn a2(invert: bool) -> String {
    return a2_or_5(0, invert);
}
fn a4(invert: bool) -> String {
    return a2_or_5(1, invert);
}

fn a3_or_5(index: usize, invert: bool) -> String {
    let mut pts = ax(index, invert);
    let r = pts.pop().unwrap();
    let pts = pts.rotate_ccw(invert, vec![0]);
    let pts: Vec<String> = concat(vec![pts, Path::new(vec![r])])
        .iter()
        .map(|p| trim(*p * TILE_WIDTH))
        .collect();

    return format!(
        "<path fill=\\\"black\\\" stroke=\\\"black\\\" d=\\\"M {a} A {r} 0 0 0 {b}\\\" />",
        a = pts[0],
        b = pts[1],
        r = pts[2]
    );
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
        let r = p.rotate_ccw(false, 0, vec![0]);
        assert_ulps_eq!(r.x, TILE_WIDTH * 0.5f64);
        assert_ulps_eq!(r.y, TILE_WIDTH * S3);

        let q = r.rotate_cw(false, 0, vec![0]);
        assert_ulps_eq!(q.x, TILE_WIDTH * 1f64);
        assert_ulps_eq!(q.y, TILE_WIDTH * 0f64, epsilon = 0.0000000000001f64);
    }
}

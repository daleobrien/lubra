use common::types::format::trimf64;
use common::types::path::Path;
use common::types::point::{Point, PATH_WIDTH, S3, TILE_WIDTH};
use itertools::concat;
use std::collections::BTreeMap;
use std::f64::consts::FRAC_PI_3;
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

fn triangle() -> Path {
    let a = Point { x: 0.0, y: 0.0 };
    let b = Point { x: 0.5, y: S3 };
    let c = Point { x: 1.0, y: 0.0 };
    Path::new(vec![a, b, c])
}

fn l_wall(invert: bool) -> String {
    let mut t = triangle();
    if invert {
        t = t.invert();
    }
    t = TILE_WIDTH * t;
    let s = t.clone().diff(false).trim();

    // TODO: text will need to rotate with i,j
    let colour = if invert { "red" } else { "blue" };
    return format!(
        "  <path stroke=\\\"{colour}\\\" d=\\\"M{a}l{b}l{c}z \\\"/>\n \
           <text stroke=\\\"{colour}\\\" x=\\\"{ax}\\\" y=\\\"{ay}\\\">2</text>\n\
           <text stroke=\\\"{colour}\\\" x=\\\"{bx}\\\" y=\\\"{by}\\\">1</text>\n\
           <text stroke=\\\"{colour}\\\" x=\\\"{cx}\\\" y=\\\"{cy}\\\">3</text>\n\
        ",
        a = s[0],
        b = s[1],
        c = s[2],
        ax = trimf64(t[0].x),
        ay = trimf64(t[0].y),
        bx = trimf64(t[1].x),
        by = trimf64(t[1].y),
        cx = trimf64(t[2].x),
        cy = trimf64(t[2].y),
    );
}

fn l_a_to_b(path: Path) -> String {
    let s = path.trim();
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

fn ax(index: usize, invert: bool) -> (Path, String) {
    //

    let [start, end, r, y] = match (invert, index) {
        // Small Arc on Node 1
        (false, 1) => [0.0, 6.0, PATH_WIDTH, S3],
        (true, 1) => [3.0, 0.0, PATH_WIDTH, 0.0],

        // Inner Arc on Node 1 (Node 4-5)
        (false, 2) => [4.0, 5.0, (0.5 - PATH_WIDTH), S3],
        (true, 2) => [1.0, 2.0, (0.5 - PATH_WIDTH), 0.0],

        // Outer Arc on Node 1 (Node 4-5)
        (false, 3) => [5.0, 4.0, (0.5 + PATH_WIDTH), S3],
        (true, 3) => [2.0, 1.0, (0.5 + PATH_WIDTH), 0.0],

        // Small Arc on Node 6
        (false, 6) => [3.0, 0.0, PATH_WIDTH, 0.0],
        (true, 6) => [0.0, 3.0, PATH_WIDTH, S3],
        _ => panic!("Invalid index"),
    };
    let x = 0.5;
    let a = polar(Point { x, y }, r, start * FRAC_PI_3);
    let b = polar(Point { x, y }, r, end * FRAC_PI_3);

    let r = trimf64(TILE_WIDTH * r);
    return (Path::new(vec![a, b]), r);
}

fn a1(invert: bool) -> String {
    return a1_or_6(1, invert);
}

fn a2(invert: bool) -> String {
    return a2_or_5(1, invert);
}

fn a3(invert: bool) -> String {
    return a3_or_4(1, invert);
}

fn a4(invert: bool) -> String {
    return a3_or_4(6, invert);
}

fn a5(invert: bool) -> String {
    return a2_or_5(6, invert);
}

fn a6(invert: bool) -> String {
    return a1_or_6(6, invert);
}

fn a_x_to_y(pts: Path, radii: (String, String)) -> String {
    let pts: Vec<String> = pts.scale().trim();

    return format!(
        "<path fill=\\\"black\\\" d=\\\"M {a} A {r1} {r1} 0 0 1 {b} L {c} A {r2} {r2} 0 0 0 {d} z \\\" /> \
         <path stroke=\\\"black\\\" d=\\\"M {a} A {r1} {r1} 0 0 1 {b} M {c} A {r2} {r2} 0 0 0 {d} \\\" />",
        a = pts[0],
        b = pts[1],
        c = pts[2],
        d = pts[3],
        r1=radii.0,
        r2=radii.1
    );
}

fn a4_to_5(invert: bool) -> String {
    // With this and others, I don't need to draw a shape, just a really fat line!

    let (arc1, r1) = ax(2, invert);
    let (arc2, r2) = ax(3, invert);
    let pts = concat(vec![arc1, arc2]);
    return a_x_to_y(pts, (r1, r2));
}
fn a4_to_6(invert: bool) -> String {
    let (arc1, r1) = ax(2, invert);
    let (arc2, r2) = ax(3, invert);
    let pts = concat(vec![arc1, arc2]).rotate_ccw(invert, vec![0, 1, 2, 3]);
    return a_x_to_y(pts, (r1, r2));
}

fn a5_to_6(invert: bool) -> String {
    let (arc1, r1) = ax(2, invert);
    let (arc2, r2) = ax(3, invert);
    let pts = concat(vec![arc1, arc2]).rotate_cw(invert, vec![0, 1, 2, 3]);
    return a_x_to_y(pts, (r1, r2));
}

fn a456(invert: bool) -> String {
    let (pts1, r) = ax(2, invert);
    let pts2 = pts1.clone().rotate_ccw(invert, vec![0, 1]);
    let pts3 = pts1.clone().rotate_cw(invert, vec![0, 1]);

    let pts1: Vec<String> = pts1.scale().trim();
    let pts2: Vec<String> = pts2.scale().trim();
    let pts3: Vec<String> = pts3.scale().trim();

    return format!(
        "<path fill=\\\"black\\\" d=\\\"M {a} A {r} {r} 0 0 1 {b} L {c} A {r} {r} 0 0 1 {d} L {e} A {r} {r} 0 0 1 {f} z \\\" />\
        <path stroke=\\\"black\\\" d=\\\"M {a} A {r} {r} 0 0 1 {b} M {c} A {r} {r} 0 0 1 {d} M {e} A {r} {r} 0 0 1 {f} \\\" />",
        a = pts1[0],
        b = pts1[1],
         c = pts2[0],
         d = pts2[1],
         e = pts3[0],
         f = pts3[1]
    );
}

fn a_x_or_y(pts: Path, r: String) -> String {
    let pts = pts.scale().trim();
    return format!(
        "<path stroke=\\\"black\\\" fill=\\\"black\\\" d=\\\"M {a} A {r} {r} 0 0 0 {b}\\\" />",
        a = pts[0],
        b = pts[1],
        r = r
    );
}

fn a1_or_6(index: usize, invert: bool) -> String {
    let (pts, r) = ax(index, invert);
    return a_x_or_y(pts, r);
}

fn a2_or_5(index: usize, invert: bool) -> String {
    let (pts, r) = ax(index, invert);
    let pts = pts.rotate_ccw(invert, vec![0, 1]);
    return a_x_or_y(pts, r);
}

fn a3_or_4(index: usize, invert: bool) -> String {
    let (pts, r) = ax(index, invert);
    let pts = pts.rotate_cw(invert, vec![0, 1]);
    return a_x_or_y(pts, r);
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
        (0b01_000_010, a2(true)),
        (0b00_000_100, a3(false)),
        (0b01_000_100, a3(true)),
        (0b00_001_000, a4(false)),
        (0b01_001_000, a4(true)),
        (0b00_010_000, a5(false)),
        (0b01_010_000, a5(true)),
        (0b00_100_000, a6(false)),
        (0b01_100_000, a6(true)),
        (0b00_011_000, a4_to_5(false)),
        (0b01_011_000, a4_to_5(true)),
        (0b00_101_000, a4_to_6(false)),
        (0b01_101_000, a4_to_6(true)),
        (0b00_110_000, a5_to_6(false)),
        (0b01_110_000, a5_to_6(true)),
        (0b00_111_000, a456(false)),
        (0b01_111_000, a456(true)),
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

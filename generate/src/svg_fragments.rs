use common::consts::{INVERT, NODE_1, NODE_2, NODE_3, NODE_4, NODE_5, NODE_6, NORMAL, WALL};
use common::types::format::trimf64;
use common::types::path::Path;
use common::types::point::{Point, PATH_WIDTH, S3, S4, TILE_WIDTH};
use itertools::concat;
use num_traits::ToPrimitive;
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

    return format!(
        "<path fill-opacity=\\\"0.0\\\" stroke-opacity=\\\"0.1\\\" stroke=\\\"red\\\" d=\\\"M{a}l{b}l{c}z\\\" />",
        a = s[0],
        b = s[1],
        c = s[2],
    );
}

fn l_a_to_b(path: Path) -> String {
    let s = path.trim();
    // TODO: Replace so that we fill and stroke at the same time, like I do for A15
    return format!(
        "<path fill=\\\"white\\\" d=\\\"M{a}l{b}l{c}l{d}z \\\"/> \
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
    let pts = TILE_WIDTH * generate_lx(invert).rotate_ccw(invert, vec![0, 4]);
    return l_a_to_b(pts);
}

fn l3_to_4(invert: bool) -> String {
    let pts = TILE_WIDTH * generate_lx(invert).rotate_cw(invert, vec![0, 4]);
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
        (false, 1) => [0.0, 3.0, PATH_WIDTH, S3],
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

fn a123pts() -> (f64, Path) {
    let r = PATH_WIDTH * 2.0;
    let w = PATH_WIDTH;
    let x = 0.5 - w;
    let y = (0.5 + w + r) * S4;
    let a = Point { x: x, y: S3 };
    let b = Point { x: x, y: y };
    let c = polar(Point { x, y }, r, 4.0 * FRAC_PI_3);
    let d = w * Point { x: -0.5, y: S3 };
    return (r, Path::new(vec![a, b, c, d]));
}

fn a123(invert: bool) -> String {
    let (r, pts) = a123pts();
    let pts = if invert { pts.invert() } else { pts };

    let dir = if invert { 1 } else { 0 };

    let pts1 = pts.clone().rotate_ccw(invert, vec![0, 1, 2, 3]).str();
    let pts2 = pts.clone().rotate_cw(invert, vec![0, 1, 2, 3]).str();
    let pts3 = pts.clone().str();

    return format!(
        "<path fill=\\\"white\\\" d=\\\"\
          M {a1} L {b1} A {r} {r} 0 0 {dir} {c1} L {d1} \
          L {a2} L {b2} A {r} {r} 0 0 {dir} {c2} L {d2} \
          L {a3} L {b3} A {r} {r} 0 0 {dir} {c3} L {d3} z\\\"/>\
        <path stroke=\\\"black\\\" d=\\\"\
          M {a1} L {b1} A {r} {r} 0 0 {dir} {c1} L {d1} \
          M {a2} L {b2} A {r} {r} 0 0 {dir} {c2} L {d2} \
          M {a3} L {b3} A {r} {r} 0 0 {dir} {c3} L {d3}\\\"/>",
        a1 = pts1[0],
        b1 = pts1[1],
        c1 = pts1[2],
        d1 = pts1[3],
        a2 = pts2[0],
        b2 = pts2[1],
        c2 = pts2[2],
        d2 = pts2[3],
        a3 = pts3[0],
        b3 = pts3[1],
        c3 = pts3[2],
        d3 = pts3[3],
        r = TILE_WIDTH * r,
    );
}

fn a23pts(invert: bool) -> (String, String, Path) {
    let w = PATH_WIDTH;
    let a = w * Point { x: -0.5, y: S3 };
    let b = w * Point { x: 0.5, y: S3 } + Point { x: 1.0, y: 0.0 };
    let c = w * Point { x: -0.5, y: -S3 } + Point { x: 1.0, y: 0.0 };
    let d = w * Point { x: 0.5, y: -S3 };

    let r1 = trimf64(TILE_WIDTH * (1.0 + PATH_WIDTH));
    let r2 = trimf64(TILE_WIDTH * (1.0 - PATH_WIDTH));
    let pts = Path::new(vec![a, b, c, d]);
    let pts = if invert { pts.invert() } else { pts };
    return (r1, r2, pts);
}

fn axy_to_str(invert: bool, r1: String, r2: String, pts: Path) -> String {
    let pts = pts.str();
    let dir1 = if invert { 1 } else { 0 };
    let dir2 = if invert { 0 } else { 1 };
    return format!(
        "<path fill=\\\"white\\\" d=\\\"M {a} A {r1} {r1} 0 0 {dir1} {b} L {c} A {r2} {r2} 0 0 {dir2} {d} z \\\" /> \
         <path stroke=\\\"black\\\" d=\\\"M {a} A {r1} {r1} 0 0 {dir1} {b} M {c} A {r2} {r2} 0 0 {dir2} {d} \\\" />",
        a = pts[0],
        b = pts[1],
        c = pts[2],
        d = pts[3],
    );
}
fn a23(invert: bool) -> String {
    let (r1, r2, pts) = a23pts(invert);
    return axy_to_str(invert, r1, r2, pts);
}

fn a13(invert: bool) -> String {
    let (r1, r2, pts) = a23pts(invert);
    let pts = pts.rotate_ccw(invert, vec![0, 1, 2, 3]);
    return axy_to_str(invert, r1, r2, pts);
}

fn a12(invert: bool) -> String {
    let (r1, r2, pts) = a23pts(invert);
    let pts = pts.rotate_cw(invert, vec![0, 1, 2, 3]);
    return axy_to_str(invert, r1, r2, pts);
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

fn a15_base(invert: bool) -> (String, String, Path) {
    let w = PATH_WIDTH;
    let r1 = trimf64(TILE_WIDTH * (0.25 + w));
    let r2 = trimf64(TILE_WIDTH * (0.25 - w));
    let x1 = 0.5 - w;
    let x2 = 1.0 + w;
    let x3 = 1.0 - w;
    let x4 = 0.5 + w;
    let y = S3;
    let a = Point { x: x1, y };
    let b = Point { x: x2, y };
    let c = Point { x: x3, y };
    let d = Point { x: x4, y };

    let pts = Path::new(vec![a, b, c, d]);
    let pts = if invert {
        pts.invert() + Point { x: 0f64, y: 0f64 }
    } else {
        pts
    };
    return (r1, r2, pts);
}
fn a15_str(invert: bool, pts: Path, r1: String, r2: String) -> String {
    let (d1, d2) = if invert { (0, 1) } else { (1, 0) };
    let pts = pts.scale();
    return format!(
        "<path stroke=\\\"black\\\" fill=\\\"white\\\" d=\\\"M {a} A {r1} {r1} 0 0 {d1} {b} M {c} A {r2} {r2} 0 0 {d2} {d}\\\" />",
        a = pts[0],
        b = pts[1],
        c = pts[2],
        d = pts[3],
    );
}

fn a15(invert: bool) -> String {
    let (r1, r2, pts) = a15_base(invert);

    return a15_str(invert, pts, r1, r2);
}

fn a14(invert: bool) -> String {
    let (r1, r2, pts) = a15_base(invert);
    let pts = pts.reflect();
    return a15_str(invert, pts, r1, r2);
}

fn a24(invert: bool) -> String {
    let (r1, r2, pts) = a15_base(invert);
    let pts = pts.rotate_ccw(invert, vec![0, 1, 2, 3]);
    return a15_str(invert, pts, r1, r2);
}
fn a26(invert: bool) -> String {
    let (r1, r2, pts) = a15_base(invert);
    let pts = pts.reflect().rotate_ccw(invert, vec![0, 1, 2, 3]);
    return a15_str(!invert, pts, r1, r2);
}

fn a36(invert: bool) -> String {
    let (r1, r2, pts) = a15_base(invert);
    let pts = pts.rotate_cw(invert, vec![0, 1, 2, 3]);
    return a15_str(invert, pts, r1, r2);
}

fn a35(invert: bool) -> String {
    let (r1, r2, pts) = a15_base(invert);
    let pts = pts.reflect().rotate_cw(invert, vec![0, 1, 2, 3]);
    return a15_str(!invert, pts, r1, r2);
}

fn a_x_to_y(pts: Path, radii: (String, String)) -> String {
    let pts: Vec<String> = pts.str();

    return format!(
    "<path fill=\\\"white\\\" d=\\\"M {a} A {r1} {r1} 0 0 1 {b} L {c} A {r2} {r2} 0 0 0 {d} z \\\" /> \
         <path stroke=\\\"black\\\" d=\\\"M {a} A {r1} {r1} 0 0 1 {b} M {c} A {r2} {r2} 0 0 0 {d} \\\" />",
    a = pts[0],
    b = pts[1],
    c = pts[2],
    d = pts[3],
    r1 = radii.0,
    r2 = radii.1
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

    let p1: Vec<String> = pts1.str();
    let (p2, p3) = match invert {
        false => (pts3, pts2),
        true => (pts2, pts3),
    };
    let p2 = p2.str();
    let p3 = p3.str();

    return format!(
    "<path fill=\\\"white\\\" d=\\\"M {a} A {r} {r} 0 0 1 {b} L {c} A {r} {r} 0 0 1 {d} L {e} A {r} {r} 0 0 1 {f} z \\\" />\
        <path stroke=\\\"black\\\" d=\\\"M {a} A {r} {r} 0 0 1 {b} M {c} A {r} {r} 0 0 1 {d} M {e} A {r} {r} 0 0 1 {f} \\\" />",
    a = p1[0], b = p1[1], c = p2[0], d = p2[1], e = p3[0], f = p3[1]);
}

fn a_x_or_y(pts: Path, r: String) -> String {
    let pts = pts.str();
    return format!(
        "<path stroke=\\\"black\\\" fill=\\\"white\\\" d=\\\"M {a} A {r} {r} 0 0 0 {b}\\\" />",
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

    for direction in [INVERT, NORMAL] {
        let invert = direction == INVERT;
        for (key, path) in [
            (direction | NODE_1 | NODE_6, l1_to_6(invert)),
            (direction | NODE_2 | NODE_5, l2_to_5(invert)),
            (direction | NODE_3 | NODE_4, l3_to_4(invert)),
            (direction | NODE_1, a1(invert)),
            (direction | NODE_2, a2(invert)),
            (direction | NODE_3, a3(invert)),
            (direction | NODE_4, a4(invert)),
            (direction | NODE_5, a5(invert)),
            (direction | NODE_6, a6(invert)),
            (direction | NODE_1 | NODE_2 | NODE_3, a123(invert)),
            (direction | NODE_2 | NODE_3, a23(invert)),
            (direction | NODE_1 | NODE_2, a12(invert)),
            (direction | NODE_1 | NODE_3, a13(invert)),
            (direction | NODE_1 | NODE_4, a14(invert)),
            (direction | NODE_1 | NODE_5, a15(invert)),
            (direction | NODE_2 | NODE_4, a24(invert)),
            (direction | NODE_2 | NODE_6, a26(invert)),
            (direction | NODE_3 | NODE_5, a35(invert)),
            (direction | NODE_3 | NODE_6, a36(invert)),
            (direction | NODE_4 | NODE_5, a4_to_5(invert)),
            (direction | NODE_4 | NODE_6, a4_to_6(invert)),
            (direction | NODE_5 | NODE_6, a5_to_6(invert)),
            (direction | NODE_4 | NODE_5 | NODE_6, a456(invert)),
            (direction | WALL, l_wall(invert)),
        ] {
            paths.insert(key.to_usize().unwrap_or_default(), path);
        }
    }

    // Change this to a look up function rather than a class with a HashMap.
    let mut file = File::create("../lib/src/renders/fragment.rs").expect("File Creation failed");
    let _ = file.write(b"/* This file is generated. */\n");
    let _ = file.write(b"use ::phf::{Map, phf_map};\n\n");
    let _ = file.write(b"pub static MAP_PATHS: Map<u32, &'static str> = phf_map! {\n");
    for (key, value) in paths {
        let _ = write!(file, "  {}u32 =>  \"{}\",\n", key, value);
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

    // #[test]
    // fn test_rotate() {
    //     use super::*;
    //     let p = TILE_WIDTH * Point { x: 1f64, y: 0f64 };
    //     let r = p.rotate_cw(false, 0, vec![0]);
    //     assert_ulps_eq!(r.x, TILE_WIDTH * 0.5f64);
    //     assert_ulps_eq!(r.y, TILE_WIDTH * S3);
    //
    //     let q = r.rotate_ccw(false, 0, vec![0]);
    //     assert_ulps_eq!(q.x, TILE_WIDTH * 1f64);
    //     assert_ulps_eq!(q.y, TILE_WIDTH * 0f64, epsilon = 0.0000000000001f64);
    // }
}

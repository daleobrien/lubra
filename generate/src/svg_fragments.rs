use crate::frag_12::{a12, a13, a23};
use crate::frag_123::a123;
use crate::frag_15_long::{a14_24, a15_35, a24_14, a26_36};
use crate::frag_15_tight::{a14_45, a15_45, a24_46, a26_46, a35_56, a36_56};
use crate::frag_16::{l1_to_6, l2_to_5, l3_to_4};
use common::consts::{
    ADJ_1, ADJ_2, ADJ_3, ADJ_4, ADJ_5, ADJ_6, FILL, INVERT, NODE_1, NODE_2, NODE_3, NODE_4, NODE_5,
    NODE_6, NORMAL, WALL,
};
use common::types::format::trimf64;
use common::types::path::Path;
use common::types::point::{Point, PATH_WIDTH, S3, TILE_WIDTH};
use itertools::concat;
use num_traits::ToPrimitive;
use std::collections::BTreeMap;
use std::f64::consts::FRAC_PI_3;
use std::fs::File;
use std::io::Write;

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
    let pts: Vec<String> = pts.str();

    return format!(
    "<path fill=\\\"{fill}\\\" d=\\\"M {a} A {r1} {r1} 0 0 1 {b} L {c} A {r2} {r2} 0 0 0 {d} z \\\" /> \
         <path stroke=\\\"black\\\" d=\\\"M {a} A {r1} {r1} 0 0 1 {b} M {c} A {r2} {r2} 0 0 0 {d} \\\" />",
    a = pts[0],
    b = pts[1],
    c = pts[2],
    d = pts[3],
    r1 = radii.0,
    r2 = radii.1,
    fill=FILL
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
    "<path fill=\\\"{FILL}\\\" d=\\\"M {a} A {r} {r} 0 0 1 {b} L {c} A {r} {r} 0 0 1 {d} L {e} A {r} {r} 0 0 1 {f} z \\\" />\
        <path stroke=\\\"black\\\" d=\\\"M {a} A {r} {r} 0 0 1 {b} M {c} A {r} {r} 0 0 1 {d} M {e} A {r} {r} 0 0 1 {f} \\\" />",
    a = p1[0], b = p1[1], c = p2[0], d = p2[1], e = p3[0], f = p3[1]);
}

fn a_x_or_y(pts: Path, r: String) -> String {
    let pts = pts.str();
    return format!(
        "<path stroke=\\\"black\\\" fill=\\\"{FILL}\\\" d=\\\"M {a} A {r} {r} 0 0 0 {b}\\\" />",
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
            (direction | NODE_1 | NODE_4 | ADJ_2 | ADJ_4, a14_24(invert)),
            (direction | NODE_1 | NODE_4 | ADJ_4 | ADJ_5, a14_45(invert)),
            (direction | NODE_1 | NODE_5 | ADJ_3 | ADJ_5, a15_35(invert)),
            (direction | NODE_1 | NODE_5 | ADJ_4 | ADJ_5, a15_45(invert)),
            (direction | NODE_2 | NODE_4 | ADJ_4 | ADJ_6, a24_46(invert)),
            (direction | NODE_2 | NODE_4 | ADJ_1 | ADJ_4, a24_14(invert)),
            (direction | NODE_2 | NODE_6 | ADJ_4 | ADJ_6, a26_46(invert)),
            (direction | NODE_2 | NODE_6 | ADJ_3 | ADJ_6, a26_36(invert)),
            (direction | NODE_3 | NODE_5 | ADJ_5 | ADJ_6, a35_56(invert)),
            (direction | NODE_3 | NODE_6 | ADJ_5 | ADJ_6, a36_56(invert)),
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

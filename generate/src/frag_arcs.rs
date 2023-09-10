use crate::frag_math;
use common::consts::FILL;
use common::types::path::Path;
use itertools::concat;

pub fn a1(invert: bool) -> String {
    return a1_or_6(1, invert);
}

pub fn a2(invert: bool) -> String {
    return a2_or_5(1, invert);
}

pub fn a3(invert: bool) -> String {
    return a3_or_4(1, invert);
}

pub fn a4(invert: bool) -> String {
    return a3_or_4(6, invert);
}

pub fn a5(invert: bool) -> String {
    return a2_or_5(6, invert);
}

pub fn a6(invert: bool) -> String {
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

pub fn a4_to_5(invert: bool) -> String {
    // With this and others, I don't need to draw a shape, just a really fat line!
    let (arc1, r1) = frag_math::ax(2, invert);
    let (arc2, r2) = frag_math::ax(3, invert);
    let pts = concat(vec![arc1, arc2]);
    return a_x_to_y(pts, (r1, r2));
}

pub fn a4_to_6(invert: bool) -> String {
    let (arc1, r1) = frag_math::ax(2, invert);
    let (arc2, r2) = frag_math::ax(3, invert);
    let pts = concat(vec![arc1, arc2]).rotate_ccw(invert, vec![0, 1, 2, 3]);
    return a_x_to_y(pts, (r1, r2));
}

pub fn a5_to_6(invert: bool) -> String {
    let (arc1, r1) = frag_math::ax(2, invert);
    let (arc2, r2) = frag_math::ax(3, invert);
    let pts = concat(vec![arc1, arc2]).rotate_cw(invert, vec![0, 1, 2, 3]);
    return a_x_to_y(pts, (r1, r2));
}

pub fn a456(invert: bool) -> String {
    let (pts1, r) = frag_math::ax(2, invert);
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
    let (pts, r) = frag_math::ax(index, invert);
    return a_x_or_y(pts, r);
}

fn a2_or_5(index: usize, invert: bool) -> String {
    let (pts, r) = frag_math::ax(index, invert);
    let pts = pts.rotate_ccw(invert, vec![0, 1]);
    return a_x_or_y(pts, r);
}

fn a3_or_4(index: usize, invert: bool) -> String {
    let (pts, r) = frag_math::ax(index, invert);
    let pts = pts.rotate_cw(invert, vec![0, 1]);
    return a_x_or_y(pts, r);
}

use itertools::concat;

use common::consts::FILL;
use common::types::path::Path;

use crate::frag_math;

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

pub fn n45(invert: bool) -> String {
    // With this and others, I don't need to draw a shape, just a really fat line!
    let (arc1, r1) = frag_math::ax(2, invert);
    let (arc2, r2) = frag_math::ax(3, invert);
    let pts = concat(vec![arc1, arc2]);
    return a_x_to_y(pts, (r1, r2));
}

pub fn a46(invert: bool) -> String {
    let (arc1, r1) = frag_math::ax(2, invert);
    let (arc2, r2) = frag_math::ax(3, invert);
    let pts = concat(vec![arc1, arc2]).rotate_ccw(invert, vec![0, 1, 2, 3]);
    return a_x_to_y(pts, (r1, r2));
}

pub fn n56(invert: bool) -> String {
    let (arc1, r1) = frag_math::ax(2, invert);
    let (arc2, r2) = frag_math::ax(3, invert);
    let pts = concat(vec![arc1, arc2]).rotate_cw(invert, vec![0, 1, 2, 3]);
    return a_x_to_y(pts, (r1, r2));
}

pub fn n456(invert: bool) -> String {
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

use itertools::concat;

use common::consts::FILL;
use common::types::format::trimf64;
use common::types::path::Path;
use common::types::point::{PATH_WIDTH, Point, S3, TILE_WIDTH};

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

fn a1_base(invert: bool) -> (String, Path) {
    let w = PATH_WIDTH;

    let r = w / 2.;

    let x1 = 0.5 - w;
    let x2 = 0.5 + w;

    let y1 = S3;
    let y2 = S3 - w;

    let a = Point { x: x1, y: y1 };
    let b = Point { x: x1, y: y2 };
    let c = Point { x: x2, y: y2 };
    let d = Point { x: x2, y: y1 };

    let pts = Path::new(vec![a, b, c, d]);

    let pts = if invert { pts.invert() } else { pts };
    let r = trimf64(TILE_WIDTH * r);
    return (r, pts);
}

pub fn a1_str(invert: bool, pts: Path, r: String) -> String {
    let d1 = if invert { 0 } else { 1 };
    let pts = pts.str();
    return format!(
        "<path  stroke=\\\"black\\\" fill=\\\"{fill}\\\" d=\\\"\
      M {a} L {b} \
      A {r} {r} 0 0 {d1} {c} \
      L {d} \
      \\\" />\
   ",
        a = pts[0],
        b = pts[1],
        c = pts[2],
        d = pts[3],
        fill = FILL,
    );
}

pub fn n1(invert: bool) -> String {
    let (r, pts) = a1_base(invert);
    return a1_str(invert, pts, r);
}

pub fn n2(invert: bool) -> String {
    let (r, pts) = a1_base(invert);
    let pts = pts.rotate_ccw(invert, vec![0, 1, 2, 3]);
    return a1_str(invert, pts, r);
}

pub fn n3(invert: bool) -> String {
    let (r, pts) = a1_base(invert);
    let pts = pts.rotate_cw(invert, vec![0, 1, 2, 3]);
    return a1_str(invert, pts, r);
}

pub fn n4(invert: bool) -> String {
    let (r, pts) = a1_base(!invert);
    let pts = pts.reflect();
    let pts = pts.rotate_cw(invert, vec![0, 1, 2, 3]);
    return a1_str(invert, pts, r);
}

pub fn n5(invert: bool) -> String {
    let (r, pts) = a1_base(!invert);
    let pts = pts.reflect();
    let pts = pts.rotate_ccw(invert, vec![0, 1, 2, 3]);
    return a1_str(invert, pts, r);
}

pub fn n6(invert: bool) -> String {
    let (r, pts) = a1_base(!invert);
    let pts = pts.reflect();
    return a1_str(invert, pts, r);
}

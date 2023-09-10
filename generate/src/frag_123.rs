use crate::svg_fragments;
use common::consts::FILL;
use common::types::path::Path;
use common::types::point::{Point, PATH_WIDTH, S3, S4, TILE_WIDTH};
use std::f64::consts::FRAC_PI_3;

fn a123pts() -> (f64, Path) {
    let r = PATH_WIDTH * 2.0;
    let w = PATH_WIDTH;
    let x = 0.5 - w;
    let y = (0.5 + w + r) * S4;
    let a = Point { x: x, y: S3 };
    let b = Point { x: x, y: y };
    let c = svg_fragments::polar(Point { x, y }, r, 4.0 * FRAC_PI_3);
    let d = w * Point { x: -0.5, y: S3 };
    return (r, Path::new(vec![a, b, c, d]));
}

pub fn a123(invert: bool) -> String {
    let (r, pts) = a123pts();
    let pts = if invert { pts.invert() } else { pts };

    let dir = if invert { 1 } else { 0 };

    let pts1 = pts.clone().rotate_ccw(invert, vec![0, 1, 2, 3]).str();
    let pts2 = pts.clone().rotate_cw(invert, vec![0, 1, 2, 3]).str();
    let pts3 = pts.clone().str();

    return format!(
        "<path fill=\\\"{fill}\\\" d=\\\"\
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
        fill = FILL,
    );
}

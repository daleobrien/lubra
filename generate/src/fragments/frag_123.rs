use crate::fragments::frag_math;
use common::consts::FILL;
use common::types::format::trimf64;
use common::types::path::Path;
use common::types::point::{Point, PATH_WIDTH, S3, S4, TILE_WIDTH};
use std::f64::consts::FRAC_PI_3;

fn base(_invert: bool) -> (Vec<String>, Path) {
    let r = PATH_WIDTH * 2.0;
    let w = PATH_WIDTH;
    let x = 0.5 - w;
    let y = (0.5 + w + r) * S4;
    let a = Point { x, y: S3 };
    let b = Point { x, y };
    let c = frag_math::polar(Point { x, y }, r, 4.0 * FRAC_PI_3);
    let d = w * Point { x: -0.5, y: S3 };

    let pts = Path::new(vec![a, b, c, d]);

    let r = trimf64(r * TILE_WIDTH);

    (vec![r], pts)
}

fn svg(invert: bool, radii: Vec<String>, pts: Path) -> String {
    let dir = if invert { 1 } else { 0 };
    let pts = pts.str();

    format!(
        "<path fill='{fill}' d='\
          M{a1}L{b1}A{r} {r} 0 0 {dir} {c1}L{d1}\
          L{a2}L{b2}A{r} {r} 0 0 {dir} {c2}L{d2}\
          L{a3}L{b3}A{r} {r} 0 0 {dir} {c3}L{d3}z'/>\
        <path stroke='black' d='\
          M{a1}L{b1}A{r} {r} 0 0 {dir} {c1}L{d1}\
          M{a2}L{b2}A{r} {r} 0 0 {dir} {c2}L{d2}\
          M{a3}L{b3}A{r} {r} 0 0 {dir} {c3}L{d3}'/>",
        a1 = pts[0],
        b1 = pts[1],
        c1 = pts[2],
        d1 = pts[3],
        a2 = pts[4],
        b2 = pts[5],
        c2 = pts[6],
        d2 = pts[7],
        a3 = pts[8],
        b3 = pts[9],
        c3 = pts[10],
        d3 = pts[11],
        r = radii[0],
        fill = FILL,
    )
}

pub fn n123(invert: bool) -> String {
    let (radii, pts) = base(invert);

    let pts = if invert { pts.invert() } else { pts };
    let pts1 = pts.clone().rotate_ccw(invert, vec![0, 1, 2, 3]);
    let pts2 = pts.clone().rotate_cw(invert, vec![0, 1, 2, 3]);
    let pts3 = pts.clone();

    let mut pts = Path::new(Vec::new());
    pts.extend(pts1);
    pts.extend(pts2);
    pts.extend(pts3);
    svg(invert, radii, pts)
}

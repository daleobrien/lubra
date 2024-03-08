use common::consts::FILL;
use common::types::format::trimf64;
use common::types::path::Path;
use common::types::point::{Point, PATH_WIDTH, S3, TILE_WIDTH};

fn a15_base(invert: bool) -> (String, String, Path) {
    let w = PATH_WIDTH;
    let r = 1.0 / 6.0;
    let x1 = 0.5 - w;
    let x2 = 0.75 + w / 2.0;
    let x3 = 0.75 - w / 2.0;
    let x4 = 0.5 + w;

    let y1 = S3;
    let y2 = S3 - S3 / 3.0;

    let y3 = (0.5 - w) * S3;
    let y4 = (0.5 + w) * S3;

    let a = Point { x: x1, y: y1 };
    let b = Point { x: x1, y: y2 };
    let c = Point { x: x2, y: y3 };

    let d = Point { x: x3, y: y4 };
    let e = Point { x: x4, y: y2 };
    let f = Point { x: x4, y: y1 };

    let pts = Path::new(vec![a, b, c, d, e, f]);

    let pts = if invert { pts.invert() } else { pts };
    let r1 = trimf64(TILE_WIDTH * (r + w));
    let r2 = trimf64(TILE_WIDTH * (r - w));
    return (r1, r2, pts);
}

pub fn a15_str(invert: bool, pts: Path, r1: String, r2: String) -> String {
    let (d1, d2) = if invert { (0, 1) } else { (1, 0) };
    let pts = pts.str();
    return format!(
    "<path stroke=\\\"black\\\" fill=\\\"{fill}\\\" d=\\\"M {a} L {b} A {r1} {r1} 0 0 {d1} {c} M {d} A {r2} {r2} 0 0 {d2} {e} L {f} \\\" />",
    a = pts[0],
    b = pts[1],
    c = pts[2],
    d = pts[3],
    e = pts[4],
    f = pts[5],
    fill=FILL,
  );
}

pub fn n15(invert: bool) -> String {
    let (r1, r2, pts) = a15_base(invert);
    return a15_str(invert, pts, r1, r2);
}

pub fn n24(invert: bool) -> String {
    let (r1, r2, pts) = a15_base(invert);
    let pts = pts.rotate_ccw(invert, vec![0, 1, 2, 3, 4, 5]);
    return a15_str(invert, pts, r1, r2);
}
pub fn n36(invert: bool) -> String {
    let (r1, r2, pts) = a15_base(invert);
    let pts = pts.rotate_cw(invert, vec![0, 1, 2, 3, 4, 5]);
    return a15_str(invert, pts, r1, r2);
}

pub fn n14(invert: bool) -> String {
    let (r1, r2, pts) = a15_base(invert);
    let pts = pts.reflect();
    return a15_str(!invert, pts, r1, r2);
}

pub fn n26(invert: bool) -> String {
    let (r1, r2, pts) = a15_base(invert);
    let pts = pts.reflect().rotate_ccw(invert, vec![0, 1, 2, 3, 4, 5]);
    return a15_str(!invert, pts, r1, r2);
}
pub fn n35(invert: bool) -> String {
    let (r1, r2, pts) = a15_base(invert);
    let pts = pts.reflect().rotate_cw(invert, vec![0, 1, 2, 3, 4, 5]);
    return a15_str(!invert, pts, r1, r2);
}

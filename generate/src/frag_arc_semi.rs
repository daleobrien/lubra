use common::consts::FILL;
use common::types::format::trimf64;
use common::types::path::Path;
use common::types::point::{Point, PATH_WIDTH, S3, TILE_WIDTH};

fn a15_base(invert: bool) -> (String, String, Path) {
    let w = PATH_WIDTH;
    let r1 = trimf64(TILE_WIDTH * (0.25 + w));
    let r2 = trimf64(TILE_WIDTH * (0.25 - w));

    let a = Point::new(0.5 - w / 2.0, S3 + S3 * w);
    let b = Point::new(0.75 + w / 2.0, S3 / 2.0 - S3 * w);
    let c = Point::new(0.75 - w / 2.0, S3 / 2.0 + S3 * w);
    let d = Point::new(0.5 + w / 2.0, S3 - S3 * w);

    let pts = Path::new(vec![a, b, c, d]);
    let pts = if invert { pts.invert() } else { pts };
    return (r1, r2, pts);
}

pub fn a15_str(invert: bool, pts: Path, r1: String, r2: String) -> String {
    let (d1, d2) = if invert { (0, 1) } else { (1, 0) };
    let pts = pts.str();
    return format!(
        "<path stroke=\\\"black\\\" fill=\\\"{fill}\\\" d=\\\"M {a} A {r1} {r1} 0 0 {d1} {b} M {c} A {r2} {r2} 0 0 {d2} {d}\\\" />",
        a = pts[0],
        b = pts[1],
        c = pts[2],
        d = pts[3],
        fill=FILL,
    );
}

pub fn n45_14(invert: bool) -> String {
    let (r1, r2, pts) = a15_base(invert);
    return a15_str(invert, pts, r1, r2);
}

pub fn n45_15(invert: bool) -> String {
    let (r1, r2, pts) = a15_base(invert);
    let pts = pts.reflect();
    return a15_str(!invert, pts, r1, r2);
}
//
pub fn n46_26(invert: bool) -> String {
    let (r1, r2, pts) = a15_base(invert);
    let pts = pts.rotate_ccw(invert, vec![0, 1, 2, 3]);
    return a15_str(invert, pts, r1, r2);
}
pub fn n46_24(invert: bool) -> String {
    let (r1, r2, pts) = a15_base(invert);
    let pts = pts.reflect().rotate_ccw(invert, vec![0, 1, 2, 3]);
    return a15_str(!invert, pts, r1, r2);
}

pub fn n56_35(invert: bool) -> String {
    let (r1, r2, pts) = a15_base(invert);
    let pts = pts.rotate_cw(invert, vec![0, 1, 2, 3]);
    return a15_str(invert, pts, r1, r2);
}

pub fn n56_36(invert: bool) -> String {
    let (r1, r2, pts) = a15_base(invert);
    let pts = pts.reflect().rotate_cw(invert, vec![0, 1, 2, 3]);
    return a15_str(!invert, pts, r1, r2);
}

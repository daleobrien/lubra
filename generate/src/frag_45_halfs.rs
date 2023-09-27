use common::consts::FILL;
use common::types::format::trimf64;
use common::types::path::Path;
use common::types::point::{Point, PATH_WIDTH, S3, TILE_WIDTH};

pub fn fn_pts() -> (String, String, Path) {
    let w = PATH_WIDTH;

    let r1 = 0.5 + w;
    let r2 = 0.5 - w;

    let a = Point {
        x: 0.25 - w / 2.0,
        y: 0.5 * S3 - w * S3,
    };

    let b = Point {
        x: 0.5,
        y: (S3 - r1),
    };
    let c = Point {
        x: 0.5,
        y: (S3 - r2),
    };

    let d = Point {
        x: 0.25 + w / 2.0,
        y: 0.5 * S3 + w * S3,
    };

    let r1 = trimf64(TILE_WIDTH * r1);
    let r2 = trimf64(TILE_WIDTH * r2);

    let pts = Path::new(vec![a, b, c, d]);
    return (r1, r2, pts);
}

pub fn a45_35(invert: bool) -> String {
    let (r1, r2, pts) = fn_pts();
    return fn_str(invert, pts, r1, r2);
}

pub fn a46_14(invert: bool) -> String {
    let (r1, r2, pts) = fn_pts();
    let pts = pts.rotate_ccw(invert, vec![0, 1, 2, 3]);
    return fn_str(invert, pts, r1, r2);
}

pub fn a56_26(invert: bool) -> String {
    let (r1, r2, pts) = fn_pts();
    let pts = pts.rotate_cw(invert, vec![0, 1, 2, 3]);
    return fn_str(invert, pts, r1, r2);
}

pub fn a45_24(invert: bool) -> String {
    let (r1, r2, pts) = fn_pts();
    let pts = pts.reflect();
    return fn_str(!invert, pts, r1, r2);
}

pub fn a46_36(invert: bool) -> String {
    let (r1, r2, pts) = fn_pts();
    let pts = pts.reflect().rotate_ccw(invert, vec![0, 1, 2, 3]);
    return fn_str(!invert, pts, r1, r2);
}

pub fn a56_15(invert: bool) -> String {
    let (r1, r2, pts) = fn_pts();
    let pts = pts.reflect().rotate_cw(invert, vec![0, 1, 2, 3]);
    return fn_str(!invert, pts, r1, r2);
}

pub fn fn_str(invert: bool, pts: Path, r1: String, r2: String) -> String {
    let (d1, d2) = if invert { (0, 1) } else { (1, 0) };
    let pts = pts.str();
    return format!(
        "<path fill=\\\"{fill}\\\" d=\\\"M {a} A {r1} {r1} 0 0 {d1} {b} L {c} A {r2} {r2} 0 0 {d2} {d} \\\" />\
        <path stroke=\\\"black\\\" d=\\\"M {a} A {r1} {r1} 0 0 {d1} {b} M {c} A {r2} {r2} 0 0 {d2} {d} \\\" />",
        a = pts[0],
        b = pts[1],
        c = pts[2],
        d = pts[3],
        fill=FILL,
    );
}

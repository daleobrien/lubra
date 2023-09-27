use common::consts::FILL;
use common::types::format::trimf64;
use common::types::path::Path;
use common::types::point::{Point, PATH_WIDTH, S3, TILE_WIDTH};

pub fn fn_pts() -> (String, String, Path) {
    let w = PATH_WIDTH;
    let r = S3 - 0.5;

    let r1 = 0.5 + w;
    let r2 = 0.5 - w;
    let a = Point { x: 0.5 - w, y: 0.0 };
    let d = Point { x: 0.5 + w, y: 0.0 };

    let b = Point {
        x: 1.0 - r1 * S3,
        y: r1 / 2.0,
    };
    let c = Point {
        x: 1.0 - r2 * S3,
        y: r2 / 2.0,
    };

    let r1 = trimf64(TILE_WIDTH * r1);
    let r2 = trimf64(TILE_WIDTH * r2);

    let pts = Path::new(vec![a, b, c, d]);
    return (r1, r2, pts);
}

pub fn a56_15(invert: bool) -> String {
    let (r1, r2, pts) = fn_pts();
    return fn_str(!invert, pts, r1, r2);
}

// pub fn a56_26(invert: bool) -> String {
//     let (r1, r2, pts) = fn_pts(invert);
//     let pts = pts.reflect();
//     return fn_str(invert, pts, r1, r2);
// }

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

use common::consts::FILL;
use common::types::format::trimf64;
use common::types::path::Path;
use common::types::point::{Point, PATH_WIDTH, S2, S3, TILE_WIDTH};

pub fn fn_pts(invert: bool) -> (String, String, Path) {
    let w = PATH_WIDTH;

    let r = S2 - 3.0 / 2.0; // Sqrt[3] - (3/2)
    let o = 1f64 - S3; //  1 - Sqrt[3]/2

    let r1 = r + w;
    let r2 = r - w;

    let a = Point {
        x: 0.5 - w / 2.0,
        y: S3 + w * S3,
    };

    let b = a - Point {
        x: o * S3,
        y: o / 2.0,
    };

    let f = Point {
        x: 0.5 + w / 2.0,
        y: S3 - w * S3,
    };

    let e = f - Point {
        x: o * S3,
        y: o / 2.0,
    };

    let c = Point {
        x: 0.5,
        y: S3 - 0.5 - w,
    };

    let d = Point {
        x: 0.5,
        y: S3 - 0.5 + w,
    };

    let r1 = trimf64(TILE_WIDTH * r1);
    let r2 = trimf64(TILE_WIDTH * r2);

    let pts = Path::new(vec![a, b, c, d, e, f]);
    let pts = if invert { pts.invert() } else { pts };
    return (r1, r2, pts);
}

pub fn a144(invert: bool) -> String {
    let (r1, r2, pts) = fn_pts(invert);
    return fn_str(invert, pts, r1, r2);
}

pub fn fn_str(invert: bool, pts: Path, r1: String, r2: String) -> String {
    let (d1, d2) = if invert { (0, 1) } else { (1, 0) };
    let pts = pts.str();
    return format!(
        "<path fill=\\\"{fill}\\\" d=\\\"M {a} L {b} A {r1} {r1} 0 0 {d1} {c} L {d} A {r2} {r2} 0 0 {d2} {e} L {f} \\\" />\
        <path stroke=\\\"black\\\" d=\\\"M {a} L {b} A {r1} {r1} 0 0 {d1} {c} M {d} A {r2} {r2} 0 0 {d2} {e} L {f} \\\" />",
        a = pts[0],
        b = pts[1],
        c = pts[2],
        d = pts[3],
        e = pts[4],
        f = pts[5],
        fill=FILL,
    );
}

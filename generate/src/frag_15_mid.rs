use common::consts::FILL;
use common::types::format::trimf64;
use common::types::path::Path;
use common::types::point::{Point, PATH_WIDTH, S3, TILE_WIDTH};

pub fn a15_56(invert: bool) -> String {
    let (r1, r2, pts) = fn_pts(invert);
    return fn_str(invert, pts, r1, r2);
}

pub fn a24_45(invert: bool) -> String {
    let (r1, r2, pts) = fn_pts(invert);
    let pts = pts.rotate_cw(invert, vec![0, 1, 2, 3, 4, 5]);
    return fn_str(invert, pts, r1, r2);
}

pub fn a36_46(invert: bool) -> String {
    let (r1, r2, pts) = fn_pts(invert);
    let pts = pts.rotate_ccw(invert, vec![0, 1, 2, 3, 4, 5]);
    return fn_str(invert, pts, r1, r2);
}

pub fn a14_46(invert: bool) -> String {
    let (r1, r2, pts) = fn_pts(invert);
    let pts = pts.reflect();
    return fn_str(!invert, pts, r1, r2);
}

pub fn a26_56(invert: bool) -> String {
    let (r1, r2, pts) = fn_pts(invert);
    let pts = pts.reflect().rotate_cw(invert, vec![0, 1, 2, 3, 4, 5]);
    return fn_str(!invert, pts, r1, r2);
}

pub fn a35_45(invert: bool) -> String {
    let (r1, r2, pts) = fn_pts(invert);
    let pts = pts.reflect().rotate_ccw(invert, vec![0, 1, 2, 3, 4, 5]);
    return fn_str(!invert, pts, r1, r2);
}

fn fn_pts(invert: bool) -> (String, String, Path) {
    let w = PATH_WIDTH;
    let r = S3 - 0.5;

    let x2 = 0.5 + r;
    let x3 = 1.0;

    let y2 = 0.5 + w;
    let y3 = 0.5 - w;

    let a = Point { x: 0.5 - w, y: S3 };
    let b = Point { x: x2, y: y3 };
    let c = Point { x: x3, y: y3 };

    let d = Point { x: x3, y: y2 };
    let e = Point { x: x2, y: y2 };
    let f = Point { x: 0.5 + w, y: S3 };

    let pts = Path::new(vec![a, b, c, d, e, f]);

    let pts = if invert { pts.invert() } else { pts };
    let r1 = trimf64(TILE_WIDTH * (r + w));
    let r2 = trimf64(TILE_WIDTH * (r - w));
    return (r1, r2, pts);
}

pub fn fn_str(invert: bool, pts: Path, r1: String, r2: String) -> String {
    let (d1, d2) = if invert { (0, 1) } else { (1, 0) };
    let pts = pts.str();
    return format!(
        "<path fill=\\\"{fill}\\\" d=\\\"M {a} A {r1} {r1} 0 0 {d1} {b} L {c} L {d} L {e} A {r2} {r2} 0 0 {d2} {f} \\\" />\
         <path stroke=\\\"black\\\" d=\\\"M {a} A {r1} {r1} 0 0 {d1} {b} L {c} M {d} L {e} A {r2} {r2} 0 0 {d2} {f} \\\" />
        ",
        a = pts[0],
        b = pts[1],
        c = pts[2],
        d = pts[3],
        e = pts[4],
        f = pts[5],
        fill=FILL,
    );
}

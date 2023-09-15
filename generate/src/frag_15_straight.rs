use common::consts::FILL;
use common::types::format::trimf64;
use common::types::path::Path;
use common::types::point::{Point, PATH_WIDTH, S3, TILE_WIDTH};

fn fn_pts(invert: bool) -> (f64, f64, Path) {
    let w = PATH_WIDTH;

    let r = 1.0f64 / 3.0f64; //

    let a = Point {
        x: 0.5f64 - w,
        y: S3,
    };

    let f = Point {
        x: 0.5f64 + w,
        y: S3,
    };

    let b = Point {
        x: 0.5f64 + r + (r + w) / 2f64,
        y: S3 - S3 * (r + w),
    };

    let e = Point {
        x: 0.5f64 + r + (r - w) / 2f64,
        y: S3 - S3 * (r - w),
    };

    let c = Point {
        x: 1.5f64 + w / 2.0,
        y: S3 - S3 * w,
    };

    let d = Point {
        x: 1.5f64 - w / 2.0,
        y: S3 + S3 * w,
    };

    let pts = Path::new(vec![a, b, c, d, e, f]);
    let pts = if invert { pts.invert() } else { pts };
    return (r + w, r - w, pts);
}

pub fn a15_25(invert: bool) -> String {
    let (r1, r2, pts) = fn_pts(invert);
    return fn_str(invert, r1, r2, pts);
}

pub fn a24_34(invert: bool) -> String {
    let (r1, r2, pts) = fn_pts(invert);
    let pts = pts.rotate_ccw(invert, vec![0, 1, 2, 3, 4, 5]);
    return fn_str(invert, r1, r2, pts);
}

pub fn a36_16(invert: bool) -> String {
    let (r1, r2, pts) = fn_pts(invert);
    let pts = pts.rotate_cw(invert, vec![0, 1, 2, 3, 4, 5]);
    return fn_str(invert, r1, r2, pts);
}

pub fn a14_34(invert: bool) -> String {
    let (r1, r2, pts) = fn_pts(invert);
    let pts = pts.reflect();
    return fn_str(!invert, r1, r2, pts);
}

pub fn a26_16(invert: bool) -> String {
    let (r1, r2, pts) = fn_pts(invert);
    let pts = pts.reflect().rotate_ccw(invert, vec![0, 1, 2, 3, 4, 5]);
    return fn_str(!invert, r1, r2, pts);
}

pub fn a35_25(invert: bool) -> String {
    let (r1, r2, pts) = fn_pts(invert);
    let pts = pts.reflect().rotate_cw(invert, vec![0, 1, 2, 3, 4, 5]);
    return fn_str(!invert, r1, r2, pts);
}

fn fn_str(invert: bool, r1: f64, r2: f64, pts: Path) -> String {
    let pts = pts.str();
    let (d1, d2) = if invert { (0, 1) } else { (1, 0) };

    return format!(
        "<path fill=\\\"{fill}\\\" d=\\\"\
           M {a} A {r1} {r1} 0 0 {d1} {b} \
                L {c} L {d}
            L {e} A {r2} {r2} 0 0 {d2} {f} \
          \\\" />\
          <path stroke=\\\"black\\\" d=\\\"\
           M {a} A {r1} {r1} 0 0 {d1} {b} \
                L {c} M {d}
            L {e} A {r2} {r2} 0 0 {d2} {f} \
          \\\" />",
        a = pts[0],
        b = pts[1],
        c = pts[2],
        d = pts[3],
        e = pts[4],
        f = pts[5],
        d1 = d1,
        d2 = d2,
        r1 = trimf64(r1 * TILE_WIDTH),
        r2 = trimf64(r2 * TILE_WIDTH),
        fill = FILL
    );
}

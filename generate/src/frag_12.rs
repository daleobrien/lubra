use common::consts::FILL;
use common::types::format::trimf64;
use common::types::path::Path;
use common::types::point::{Point, PATH_WIDTH, S3, TILE_WIDTH};

fn a23pts(invert: bool) -> (String, String, Path) {
    let w = PATH_WIDTH;
    let a = w * Point { x: -0.5, y: S3 };
    let b = w * Point { x: 0.5, y: S3 } + Point { x: 1.0, y: 0.0 };
    let c = w * Point { x: -0.5, y: -S3 } + Point { x: 1.0, y: 0.0 };
    let d = w * Point { x: 0.5, y: -S3 };

    let r1 = trimf64(TILE_WIDTH * (1.0 + PATH_WIDTH));
    let r2 = trimf64(TILE_WIDTH * (1.0 - PATH_WIDTH));
    let pts = Path::new(vec![a, b, c, d]);
    let pts = if invert { pts.invert() } else { pts };
    return (r1, r2, pts);
}

fn axy_to_str(invert: bool, r1: String, r2: String, pts: Path) -> String {
    let pts = pts.str();
    let dir1 = if invert { 1 } else { 0 };
    let dir2 = if invert { 0 } else { 1 };
    return format!(
        "<path fill='{fill}' d='M {a} A {r1} {r1} 0 0 {dir1} {b} L {c} A {r2} {r2} 0 0 {dir2} {d} z ' /> \
         <path stroke='black' d='M {a} A {r1} {r1} 0 0 {dir1} {b} M {c} A {r2} {r2} 0 0 {dir2} {d} ' />",
        a = pts[0],
        b = pts[1],
        c = pts[2],
        d = pts[3],
        fill=FILL,
    );
}

pub fn n23(invert: bool) -> String {
    let (r1, r2, pts) = a23pts(invert);
    return axy_to_str(invert, r1, r2, pts);
}

pub fn n13(invert: bool) -> String {
    let (r1, r2, pts) = a23pts(invert);
    let pts = pts.rotate_ccw(invert, vec![0, 1, 2, 3]);
    return axy_to_str(invert, r1, r2, pts);
}

pub fn n12(invert: bool) -> String {
    let (r1, r2, pts) = a23pts(invert);
    let pts = pts.rotate_cw(invert, vec![0, 1, 2, 3]);
    return axy_to_str(invert, r1, r2, pts);
}

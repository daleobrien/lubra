use common::consts::FILL;
use common::types::format::trimf64;
use common::types::path::Path;
use common::types::point::{Point, PATH_WIDTH, S3, TILE_WIDTH};

fn a1_base(invert: bool) -> (String, Path) {
    let w = PATH_WIDTH;
    let h = PATH_WIDTH / 2.;

    let r = w / 2.;

    let x1 = 0.5 - w;
    let x2 = 0.5 + w;

    let y1 = S3;
    let y2 = S3 - h;

    let a = Point { x: x1, y: y1 };
    let b = Point { x: x1, y: y2 };
    let c = Point { x: x2, y: y2 };
    let d = Point { x: x2, y: y1 };

    let pts = Path::new(vec![a, b, c, d]);

    let pts = if invert { pts.invert() } else { pts };
    let r = trimf64(TILE_WIDTH * r);
    return (r, pts);
}

fn a1_str(invert: bool, pts: Path, r: String) -> String {
    let d1 = if invert { 0 } else { 1 };
    let pts = pts.str();
    return format!(
        "<path  stroke=\\\"black\\\" fill=\\\"{fill}\\\" d=\\\"\
      M {a} L {b} \
      A {r} {r} 0 0 {d1} {c} \
      L {d} \
      \\\" />\
   ",
        a = pts[0],
        b = pts[1],
        c = pts[2],
        d = pts[3],
        fill = FILL,
    );
}

pub fn n1(invert: bool) -> String {
    let (r, pts) = a1_base(invert);
    return a1_str(invert, pts, r);
}

pub fn n2(invert: bool) -> String {
    let (r, pts) = a1_base(invert);
    let pts = pts.rotate_ccw(invert, vec![0, 1, 2, 3]);
    return a1_str(invert, pts, r);
}

pub fn n3(invert: bool) -> String {
    let (r, pts) = a1_base(invert);
    let pts = pts.rotate_cw(invert, vec![0, 1, 2, 3]);
    return a1_str(invert, pts, r);
}

pub fn n4(invert: bool) -> String {
    let (r, pts) = a1_base(!invert);
    let pts = pts.reflect();
    let pts = pts.rotate_cw(invert, vec![0, 1, 2, 3]);
    return a1_str(invert, pts, r);
}

pub fn n5(invert: bool) -> String {
    let (r, pts) = a1_base(!invert);
    let pts = pts.reflect();
    let pts = pts.rotate_ccw(invert, vec![0, 1, 2, 3]);
    return a1_str(invert, pts, r);
}

pub fn n6(invert: bool) -> String {
    let (r, pts) = a1_base(!invert);
    let pts = pts.reflect();
    return a1_str(invert, pts, r);
}

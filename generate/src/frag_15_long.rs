use common::consts::FILL;
use common::types::format::trimf64;
use common::types::path::Path;
use common::types::point::{Point, PATH_WIDTH, S3, TILE_WIDTH};

pub fn a15_35(invert: bool) -> String {
    let w = PATH_WIDTH;
    let r1 = 0.25 + w;
    let r2 = 0.25 - w;

    let a = Point { x: 0.5 - w, y: S3 };
    let b = Point {
        x: 0.5 - w + r1 * (1.0 - S3),
        y: S3 - r1 / 2.0,
    };
    let c = Point { x: 0.5 + w, y: S3 };
    let d = Point {
        x: 0.5 + w + r2 * (1.0 - S3),
        y: S3 - r2 / 2.0,
    };

    let e = Point {
        x: 1.0 - w - r2 * (1.0 - S3),
        y: r2 / 2.0,
    };
    let f = Point {
        x: 1.0 + w - r1 * (1.0 - S3),
        y: r1 / 2.0,
    };
    let g = Point {
        x: 1.0 - w,
        y: 0f64,
    };
    let h = Point {
        x: 1.0 + w,
        y: 0f64,
    };

    let pts = Path::new(vec![a, b, c, d, e, f, g, h]);

    let pts = if invert {
        pts.invert() + Point { x: 0f64, y: 0f64 }
    } else {
        pts
    };
    let r1 = trimf64(TILE_WIDTH * r1);
    let r2 = trimf64(TILE_WIDTH * r2);

    let (d1, d2) = if invert { (0, 1) } else { (1, 0) };
    let pts = pts.str();
    return format!(
        "<path  fill=\\\"{FILL}\\\" d=\\\"\
           M {a} A {r1} {r1} 0 0 {d1} {b} \
           L {e} A {r2} {r2} 0 0 {d2} {g} \
           L {h} A {r1} {r1} 0 0 {d1} {f} \
           L {d} A {r2} {r2} 0 0 {d2} {c}\\\"  />\
        <path stroke=\\\"black\\\"  d=\\\"\
           M {a} A {r1} {r1} 0 0 {d1} {b} \
           L {e} A {r2} {r2} 0 0 {d2} {g} \
           M {h} A {r1} {r1} 0 0 {d1} {f} \
           L {d} A {r2} {r2} 0 0 {d2} {c}\\\"  />\
           ",
        a = pts[0],
        b = pts[1],
        c = pts[2],
        d = pts[3],
        e = pts[4],
        f = pts[5],
        g = pts[6],
        h = pts[7],
    );
}

pub fn a14_24(invert: bool) -> String {
    return a15_35(invert);
}

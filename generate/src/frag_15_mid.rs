use common::consts::FILL;
use common::types::format::trimf64;
use common::types::path::Path;
use common::types::point::{Point, PATH_WIDTH, S3, TILE_WIDTH};

pub fn a15_56(invert: bool) -> String {
    let w = PATH_WIDTH;

    let r1 = 0.3169872981077806766181384146235319082643f64; // 1/4 (3 - Sqrt(3)
    let r2 = S3; // Sqrt(3)/2

    let r1_small = r1 - w;
    let r1_large = r1 + w;

    let r2_small = r2 - w;
    let r2_large = r2 + w;

    let a = Point {
        x: 0.5f64 - w,
        y: S3,
    };
    let f = Point {
        x: 0.5f64 + w,
        y: S3,
    };
    let e = Point {
        x: 0.5f64 + r1,
        y: S3 - r1_small,
    };
    let b = Point {
        x: 0.5f64 + r1,
        y: S3 - r1_large,
    };

    let d = Point {
        x: 1.5f64 - (0.5f64 - w) / 2.0,
        y: S3 - (0.5f64 - w) * S3,
    };
    let c = Point {
        x: 1.5f64 - (0.5f64 + w) / 2.0,
        y: S3 - (0.5f64 + w) * S3,
    };

    let pts = Path::new(vec![a, b, c, d, e, f]);
    let pts = if invert { pts.invert() } else { pts };

    let pts = pts.str();

    let (d1, d2) = if invert { (0, 1) } else { (1, 0) };

    return format!(
        "<path fill=\\\"{fill}\\\" d=\\\"\
           M {a} A {r1_l} {r1_l} 0 0 {d1} {b} \
                 A {r2_s} {r2_s} 0 0 {d2} {c} \
           L {d} A {r2_l} {r2_l} 0 0 {d1} {e} \
                 A {r1_s} {r1_s} 0 0 {d2} {f} \
          \\\" />\
          <path stroke=\\\"black\\\" d=\\\"\
           M {a} A {r1_l} {r1_l} 0 0 {d1} {b} \
                 A {r2_s} {r2_s} 0 0 {d2} {c} \
           M {d} A {r2_l} {r2_l} 0 0 {d1} {e} \
                 A {r1_s} {r1_s} 0 0 {d2} {f} \
          \\\" />",
        a = pts[0],
        b = pts[1],
        c = pts[2],
        d = pts[3],
        e = pts[4],
        f = pts[5],
        r1_s = trimf64(r1_small * TILE_WIDTH),
        r1_l = trimf64(r1_large * TILE_WIDTH),
        r2_s = trimf64(r2_small * TILE_WIDTH),
        r2_l = trimf64(r2_large * TILE_WIDTH),
        fill = FILL
    );
}

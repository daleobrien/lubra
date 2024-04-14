use common::consts::FILL;
use common::types::format::trimf64;
use common::types::path::Path;
use common::types::point::{Point, PATH_WIDTH, S2, S3, S4, S5, TILE_WIDTH};

fn a145_base(invert: bool) -> (String, String, Path) {
    let w = PATH_WIDTH;

    let pt1 = Point {
        x: 0.5f64 + ((-1.) * w),
        y: S3,
    };
    let pt2 = Point {
        x: 0.5f64 + ((-1.) * w),
        y: S4,
    };
    let pt3 = Point {
        x: 0.25f64 * (1. + (2. * w)),
        y: 0.25f64 * S2 * (1. + (2. * w)),
    };
    let pt4 = Point {
        x: 0.25f64 + ((-0.5f64) * w),
        y: (0.25f64 * S2) + ((-0.5f64) * S2 * w),
    };
    let pt5 = Point {
        x: 0.166667f64
            * (2. + (3. * (0.166667f64 + w) * (((1. + (6. * w)) * (1. + (6. * w))).powf(-0.5f64)))),
        y: S5
            * (2.
                + ((-3.)
                    * (0.166667f64 + w)
                    * (((1. + (6. * w)) * (1. + (6. * w))).powf(-0.5f64))
                    * ((1. + (16. * w) + (48. * (w * w))).sqrt()))),
    };
    let pt6 = Point {
        x: 1.
            + (0.166667f64
                * ((-2.)
                    + ((-3.)
                        * (0.166667f64 + w)
                        * (((1. + (6. * w)) * (1. + (6. * w))).powf(-0.5f64))))),
        y: S5
            * (2.
                + ((-3.)
                    * (0.166667f64 + w)
                    * (((1. + (6. * w)) * (1. + (6. * w))).powf(-0.5f64))
                    * ((1. + (16. * w) + (48. * (w * w))).sqrt()))),
    };
    let pt7 = Point {
        x: 0.75f64 + (0.5f64 * w),
        y: (0.25f64 * S2) + ((-0.5f64) * S2 * w),
    };
    let pt8 = Point {
        x: 1. + (0.25f64 * ((-1.) + ((-2.) * w))),
        y: 0.25f64 * S2 * (1. + (2. * w)),
    };
    let pt9 = Point {
        x: 0.5f64 + w,
        y: S4,
    };
    let pt10 = Point {
        x: 0.5f64 + w,
        y: S3,
    };

    let pts = Path::new(vec![pt1, pt2, pt3, pt4, pt5, pt6, pt7, pt8, pt9, pt10]);

    let pts = if invert { pts.invert() } else { pts };
    let r1s = trimf64(TILE_WIDTH * (1. / 6. - w));
    let r1l = trimf64(TILE_WIDTH * (1. / 6. + w));
    return (r1s, r1l, pts);
}

fn a145_str(invert: bool, pts: Path, r1s: String, r1l: String) -> String {
    let (d1, d2) = if invert { (0, 1) } else { (1, 0) };
    let pts = pts.str();
    return format!(
        "<path fill=\\\"{fill}\\\" d=\\\"\
      M {a} \
      L {b} \
      A {r1s} {r1s} 0 0 {d1} {c} \
      L {d} \
      A {r1l} {r1l} 0 0 {d2} {e} \
      A {r1l} {r1l} 0 0 {d1} {f} \
      A {r1l} {r1l} 0 0 {d2} {g} \
      L {h} \
      A {r1s} {r1s} 0 0 {d1} {i} \
      L {j}
      \\\" />\
      <path stroke=\\\"black\\\" d=\\\"\
      M {a} \
      L {b} \
      A {r1s} {r1s} 0 0 {d1} {c} \
      M {d} \
      A {r1l} {r1l} 0 0 {d2} {e} \
      A {r1l} {r1l} 0 0 {d1} {f} \
      A {r1l} {r1l} 0 0 {d2} {g} \
      M {h} \
      A {r1s} {r1s} 0 0 {d1} {i} \
      L {j}
      \\\" />",
        a = pts[0],
        b = pts[1],
        c = pts[2],
        d = pts[3],
        e = pts[4],
        f = pts[5],
        g = pts[6],
        h = pts[7],
        i = pts[8],
        j = pts[9],
        fill = FILL,
    );
}

pub fn n145(invert: bool) -> String {
    let (r1s, r1l, pts) = a145_base(invert);
    let pts = pts;
    return a145_str(!invert, pts, r1s, r1l);
}

pub fn n246(invert: bool) -> String {
    let (r1s, r1l, pts) = a145_base(invert);
    let pts = pts.rotate_ccw(invert, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    return a145_str(!invert, pts, r1s, r1l);
}

pub fn n356(invert: bool) -> String {
    let (r1s, r1l, pts) = a145_base(invert);
    let pts = pts.rotate_cw(invert, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    return a145_str(!invert, pts, r1s, r1l);
}

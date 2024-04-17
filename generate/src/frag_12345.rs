use common::consts::FILL;
use common::types::format::trimf64;
use common::types::path::Path;
use common::types::point::{Point, PATH_WIDTH, S2, S3, S4, S5, TILE_WIDTH};

fn base(invert: bool) -> (String, String, Path) {
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
        x: 0.25f64 * (1. + ((-2.) * w)),
        y: 0.25f64 * S2 * (1. + ((-2.) * w)),
    };
    let pt5 = Point {
        x: 0.25f64 * (1. + ((-2.) * w)),
        y: 0.25f64 * S4 * (1. + (6. * w)),
    };
    let pt6 = Point {
        x: (-0.5f64) * w,
        y: S3 * w,
    };
    let pt7 = Point {
        x: 0.5f64 * w,
        y: (-0.5f64) * S2 * w,
    };
    let pt8 = Point {
        x: 0.25f64 * (1. + (2. * w)),
        y: 0.25f64 * S4 * (1. + ((-6.) * w)),
    };
    let pt9 = Point {
        x: 0.333333f64 + w,
        y: S5,
    };
    let pt10 = Point {
        x: 0.666667f64 + ((-1.) * w),
        y: S5,
    };
    let pt11 = Point {
        x: 0.25f64 * (3. + ((-2.) * w)),
        y: 0.25f64 * S4 * (1. + ((-6.) * w)),
    };
    let pt12 = Point {
        x: 1. + ((-0.5f64) * w),
        y: (-0.5f64) * S2 * w,
    };
    let pt13 = Point {
        x: 0.5f64 * (2. + w),
        y: S3 * w,
    };
    let pt14 = Point {
        x: 0.25f64 * (3. + (2. * w)),
        y: 0.25f64 * S4 * (1. + (6. * w)),
    };
    let pt15 = Point {
        x: 0.25f64 * (3. + (2. * w)),
        y: 0.25f64 * S2 * (1. + ((-2.) * w)),
    };
    let pt16 = Point {
        x: 0.25f64 * (3. + ((-2.) * w)),
        y: 0.25f64 * S2 * (1. + (2. * w)),
    };
    let pt17 = Point {
        x: 0.5f64 + w,
        y: S4,
    };
    let pt18 = Point {
        x: 0.5f64 + w,
        y: S3,
    };

    let pts = Path::new(vec![
        pt1, pt2, pt3, pt4, pt5, pt6, pt7, pt8, pt9, pt10, pt11, pt12, pt13, pt14, pt15, pt16,
        pt17, pt18,
    ]);

    let pts = if invert { pts.invert() } else { pts };
    let r1s = trimf64(TILE_WIDTH * (1. / 6. - w));
    let r1l = trimf64(TILE_WIDTH * (1. / 6. + w));
    return (r1s, r1l, pts);
}

fn to_str(invert: bool, pts: Path, r1s: String, r1l: String) -> String {
    let (d1, d2) = if invert { (0, 1) } else { (1, 0) };
    let pts = pts.str();
    return format!(
        "<path fill=\\\"{fill}\\\" d=\\\"\
      M {a} \
      L {b} \
      A {r1s} {r1s} 0 0 {d1} {c} \
      L {d} \
      A {r1s} {r1s} 0 0 {d1} {e} \
      L {f} \
      L {g} \
      L {h} \
      A {r1l} {r1l} 0 0 {d2} {i} \
      A {r1s} {r1s} 0 0 {d1} {j} \
      A {r1l} {r1l} 0 0 {d2} {k} \
      L {l} \
      L {m} \
      L {n} \
      A {r1s} {r1s} 0 0 {d1} {o} \
      L {p} \
      A {r1s} {r1s} 0 0 {d1} {q} \
      L {r}
      \\\" />
      <path stroke=\\\"black\\\" d=\\\"\
      M {a} \
      L {b} \
      A {r1s} {r1s} 0 0 {d1} {c} \
      M {d} \
      A {r1s} {r1s} 0 0 {d1} {e} \
      L {f} \
      M {g} \
      L {h} \
      A {r1l} {r1l} 0 0 {d2} {i} \
      A {r1s} {r1s} 0 0 {d1} {j} \
      A {r1l} {r1l} 0 0 {d2} {k} \
      L {l} \
      M {m} \
      L {n} \
      A {r1s} {r1s} 0 0 {d1} {o} \
      M {p} \
      A {r1s} {r1s} 0 0 {d1} {q} \
      L {r}
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
        k = pts[10],
        l = pts[11],
        m = pts[12],
        n = pts[13],
        o = pts[14],
        p = pts[15],
        q = pts[16],
        r = pts[17],
        fill = FILL,
    );
}

pub fn n12345(invert: bool) -> String {
    let (r1s, r1l, pts) = base(invert);
    return to_str(!invert, pts, r1s, r1l);
}

pub fn n12346(invert: bool) -> String {
    let (r1s, r1l, pts) = base(invert);
    let pts = pts.rotate_ccw(
        invert,
        vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17],
    );
    return to_str(!invert, pts, r1s, r1l);
}

pub fn n12356(invert: bool) -> String {
    let (r1s, r1l, pts) = base(invert);
    let pts = pts.rotate_cw(
        invert,
        vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17],
    );
    return to_str(!invert, pts, r1s, r1l);
}

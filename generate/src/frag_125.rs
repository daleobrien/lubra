use common::consts::FILL;
use common::define_shape_family;
use common::macros::{identity, reflect, reflect_then_rotate_ccw, reflect_then_rotate_cw, rotate_ccw, rotate_cw};
use common::types::format::trimf64;
use common::types::path::Path;
use common::types::point::{Point, PATH_WIDTH, S2, S3, TILE_WIDTH};

fn base(invert: bool) -> (Vec<String>, Path) {
    let w = PATH_WIDTH;
    let r1 = 0.3333; // Adjustable

    let r2: f64 = (1. + (2. * r1)) / (4. + (12. * r1));

    let pt1 = Point {
        x: 0.5f64 + ((-1.) * w),
        y: S3,
    };
    let pt2 = Point {
        x: (1. + ((r1 * r1) * (12. + ((-24.) * w))) + (r1 * (5. + ((-12.) * w))) + ((-1.) * w))
            / (2. + (12. * r1) + (24. * (r1 * r1))),
        y: (S2 * (1. + (8. * (r1 * r1)) + w + (r1 * (5. + (4. * w)))))
            / (2. + (12. * r1) + (24. * (r1 * r1))),
    };
    let pt3 = Point {
        x: 0.25f64 * (3. + ((-2.) * r2) + ((-2.) * S2 * (r2 + w))),
        y: 0.25f64 * (S2 + (2. * ((-1.) + S2) * r2) + ((-2.) * w)),
    };
    let pt4 = Point {
        x: 0.25f64 * (3. + ((-4.) * S2 * r2) + ((-2.) * w)),
        y: ((-1.) * r2) + (0.25f64 * S2 * (1. + (2. * w))),
    };
    let pt5 = Point {
        x: (-0.5f64) * w,
        y: S3 * w,
    };
    let pt6 = Point {
        x: 0.5f64 * w,
        y: (-0.5f64) * S2 * w,
    };
    let pt7 = Point {
        x: 0.25f64 * (3. + ((-4.) * S2 * r2) + (2. * w)),
        y: ((-1.) * r2) + (0.25f64 * S2 * (1. + ((-2.) * w))),
    };
    let pt8 = Point {
        x: (5. + ((-1.) * S2) + (4. * S2 * w) + (2. * r1 * (8. + ((-1.) * S2) + (6. * S2 * w))))
            / (8. + (24. * r1)),
        y: ((-3.) + (3. * S2) + (2. * r1 * ((-3.) + (4. * S2) + ((-6.) * w))) + ((-4.) * w))
            / (8. + (24. * r1)),
    };
    let pt9 = Point {
        x: (5. + (16. * r1)) / (8. + (24. * r1)),
        y: ((-2.) + (3. * S2) + (4. * r1 * ((-1.) + (2. * S2) + ((-6.) * w))) + ((-8.) * w))
            / (8. + (24. * r1)),
    };
    let pt10 = Point {
        x: 0.75f64 + (0.5f64 * w),
        y: (0.25f64 * S2) + ((-0.5f64) * S2 * w),
    };
    let pt11 = Point {
        x: 0.25f64 * (3. + ((-2.) * w)),
        y: 0.25f64 * S2 * (1. + (2. * w)),
    };
    let pt12 = Point {
        x: (1. + (5. * r1) + (12. * (r1 * r1)) + w + (12. * r1 * w) + (24. * (r1 * r1) * w))
            / (2. + (12. * r1) + (24. * (r1 * r1))),
        y: (((-1.5f64)
            * (1. + (4. * r1))
            * (1. + (5. * r1) + (8. * (r1 * r1)) + ((-1.) * w) + ((-4.) * r1 * w)))
            / (1. + (6. * r1) + (12. * (r1 * r1))))
            / (((-1.) * S2) + ((-4.) * S2 * r1)),
    };
    let pt13 = Point {
        x: 0.5f64 + w,
        y: S3,
    };

    let pts = Path::new(vec![
        pt1, pt2, pt3, pt4, pt5, pt6, pt7, pt8, pt9, pt10, pt11, pt12, pt13,
    ]);

    let pts = if invert { pts.invert() } else { pts };
    let r1s = trimf64(TILE_WIDTH * (r1 - w));
    let r1l = trimf64(TILE_WIDTH * (r1 + w));
    let r2s = trimf64(TILE_WIDTH * (r2 - w));
    let r2l = trimf64(TILE_WIDTH * (r2 + w));

    (vec![r1s, r1l, r2s, r2l], pts)
}

fn svg(invert: bool, radii: Vec<String>, pts: Path) -> String {
    let (d1, d2) = if invert { (0, 1) } else { (1, 0) };
    let pts = pts.str();
    return format!(
        "<path fill=\\\"{fill}\\\" d=\\\"\
      M {a} \
      A {r1s} {r1s} 0 0 {d1} {b} \
      A {r2l} {r2l} 0 0 {d2} {c} \
      A {r2s} {r2s} 0 0 {d1} {d} \
      L {e} \
      L {f} \
      L {g} \
      A {r2l} {r2l} 0 0 {d2} {h} \
      A {r2s} {r2s} 0 0 {d1} {i} \
      A {r2l} {r2l} 0 0 {d2} {j} \
      L {k}
      A {r2s} {r2s} 0 0 {d1} {l} \
      A {r1l} {r1l} 0 0 {d2} {m} \
      Z \
      \\\" /> \
      <path stroke=\\\"black\\\" d=\\\"\
      M {a} \
      A {r1s} {r1s} 0 0 {d1} {b} \
      A {r2l} {r2l} 0 0 {d2} {c} \
      A {r2s} {r2s} 0 0 {d1} {d} \
      L {e} \
      M {f} \
      L {g} \
      A {r2l} {r2l} 0 0 {d2} {h} \
      A {r2s} {r2s} 0 0 {d1} {i} \
      A {r2l} {r2l} 0 0 {d2} {j} \
      M {k}
      A {r2s} {r2s} 0 0 {d1} {l} \
      A {r1l} {r1l} 0 0 {d2} {m} \
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
        fill = FILL,
        r1s = radii[0],
        r1l = radii[1],
        r2s = radii[2],
        r2l = radii[3],
    );
}

define_shape_family! {
    base: base,
    svg: svg,
    offset_rotations: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
    variants: {
        n125: false, true => identity,
        n134: false, false => reflect,
        n126: false, false => reflect_then_rotate_ccw,
        n234: false, true => rotate_ccw,
        n235: false, false => reflect_then_rotate_cw,
        n136: false, true => rotate_cw,
    }
}

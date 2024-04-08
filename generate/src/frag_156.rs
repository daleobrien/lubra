use common::consts::FILL;
use common::types::format::trimf64;
use common::types::path::Path;
use common::types::point::{PATH_WIDTH, Point, S1, S2, S3, TILE_WIDTH};

fn a156_base(invert: bool) -> (String, String, String, String, Path) {
    let w = PATH_WIDTH;
    let r1 = 0.3333f64; // Adjustable

    let r2: f64 = (1. + (2. * r1)) / (4. + (12. * r1));

    let pt1 = Point {
        x: 0.5f64 + ((-1.) * w),
        y: S3,
    };
    let pt2 = Point {
        x: (1. + ((-1.) * w) + (r1 * (5. + (12. * r1) + ((-12.) * (1. + (2. * r1)) * w))))
            / (2. + (12. * r1 * (1. + (2. * r1)))),
        y: (S2 * (1. + w + (r1 * (5. + (8. * r1) + (4. * w)))))
            / (2. + (12. * r1 * (1. + (2. * r1)))),
    };
    let pt3 = Point {
        x: (0.25f64
            * (9.
                + (22. * r1)
                + ((-8.) * (r1 * r1))
                + ((-6.) * S1 * r1.sqrt() * ((1. + (2. * r1)).powf(1.5f64)))
                + ((2.
                    * ((-3.)
                        + ((-10.) * r1)
                        + (12. * (r1 * r1))
                        + (48. * r1.powi(3))
                        + (12. * S1 * ((r1 * (1. + (2. * r1))).sqrt()))
                        + (60. * r1.powf(1.5f64) * ((2. + (4. * r1)).sqrt()))
                        + (72. * r1.powf(2.5f64) * ((2. + (4. * r1)).sqrt())))
                    * (r1 + ((-1.) * w)))
                    / (1. + (6. * r1 * (1. + (2. * r1)))))))
            / (3. + (8. * r1)),
        y: (0.25f64
            * S2
            * (((-1.)
                * (1. + (2. * r1))
                * ((-3.) + ((-4.) * r1) + (2. * S1 * ((r1 * (1. + (2. * r1))).sqrt()))))
                + ((2.
                    * (3.
                        + (4. * S1 * ((r1 * (1. + (2. * r1))).sqrt()))
                        + (24. * S1 * ((r1.powi(5) * (1. + (2. * r1))).sqrt()))
                        + (2.
                            * r1
                            * (5.
                                + (10. * S1 * ((r1 * (1. + (2. * r1))).sqrt()))
                                + ((-6.) * r1 * (1. + (4. * r1))))))
                    * (r1 + ((-1.) * w)))
                    / (1. + (6. * r1 * (1. + (2. * r1)))))))
            / (3. + (8. * r1)),
    };
    let pt4 = Point {
        x: (9.
            + (28. * r1)
            + ((-6.) * S1 * ((r1 * (1. + (2. * r1))).sqrt()))
            + ((-6.) * w)
            + ((-8.) * r1 * w)
            + ((-12.) * S1 * ((r1 * (1. + (2. * r1))).sqrt()) * w))
            / (12. + (32. * r1)),
        y: ((-0.25f64)
            * S2
            * ((-3.) + ((-4.) * r1) + (2. * S1 * ((r1 * (1. + (2. * r1))).sqrt())))
            * (1. + (2. * w)))
            / (3. + (8. * r1)),
    };
    let pt5 = Point {
        x: 0.5 + ((-1.) * w),
        y: 0.,
    };
    let pt6 = Point { x: 0.5 + w, y: 0. };
    let pt7 = Point {
        x: 0.25f64 * (3. + (2. * w)),
        y: 0.25f64 * S2 * (1. + ((-2.) * w)),
    };
    let pt8 = Point {
        x: 0.25f64 * (3. + ((-2.) * w)),
        y: 0.25f64 * S2 * (1. + (2. * w)),
    };
    let pt9 = Point {
        x: (1. + w + (r1 * (5. + (12. * w) + (12. * r1 * (1. + (2. * w))))))
            / (2. + (12. * r1 * (1. + (2. * r1)))),
        y: (S2 * (1. + (r1 * (5. + (8. * r1) + ((-4.) * w))) + ((-1.) * w)))
            / (2. + (12. * r1 * (1. + (2. * r1)))),
    };
    let pt10 = Point {
        x: 0.5f64 + w,
        y: S3,
    };

    let pts = Path::new(vec![pt1, pt2, pt3, pt4, pt5, pt6, pt7, pt8, pt9, pt10]);

    let pts = if invert { pts.invert() } else { pts };
    let r1s = trimf64(TILE_WIDTH * (r1 - w));
    let r2s = trimf64(TILE_WIDTH * (r1 + w));
    let r3s = trimf64(TILE_WIDTH * (r2 - w));
    let r4s = trimf64(TILE_WIDTH * (r2 + w));
    return (r1s, r2s, r3s, r4s, pts);
}

fn a156_str(invert: bool, pts: Path, r1: String, r2: String, r3: String, r4: String) -> String {
    let (d1, d2) = if invert { (0, 1) } else { (1, 0) };
    let pts = pts.str();
    return format!(
        "<path fill=\\\"{fill}\\\"  d=\\\"\
      M {a} \
      A {r1} {r1} 0 0 {d1} {b} \
      A {r4} {r4} 0 0 {d2} {c} \
      A {r3} {r3} 0 0 {d1} {d} \
      A {r4} {r4} 0 0 {d2} {e} \
      A {r1} {r1} 0 0 {d1} {f} \
      L {g} \
      A {r2} {r2} 0 0 {d2} {h} \
      A {r3} {r3} 0 0 {d1} {i} \
      L {j}
      A {r3} {r3} 0 0 {d1} {k} \
      A {r2} {r2} 0 0 {d2} {l} \
      \\\" />\
      <path stroke=\\\"black\\\" d=\\\"\
      M {a} \
      A {r1} {r1} 0 0 {d1} {b} \
      A {r4} {r4} 0 0 {d2} {c} \
      A {r3} {r3} 0 0 {d1} {d} \
      A {r4} {r4} 0 0 {d2} {e} \
      A {r1} {r1} 0 0 {d1} {f} \
      M {g} \
      A {r2} {r2} 0 0 {d2} {h} \
      A {r3} {r3} 0 0 {d1} {i} \
      M {j}
      A {r3} {r3} 0 0 {d1} {k} \
      A {r2} {r2} 0 0 {d2} {l} \
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
        fill = FILL,
    );
}

pub fn n156(invert: bool) -> String {
    let (r1, r2, r3, r4, pts) = a156_base(invert);
    let pts = pts
        .reflect()
        .rotate_cw(invert, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
    return a156_str(invert, pts, r1, r2, r3, r4);
}

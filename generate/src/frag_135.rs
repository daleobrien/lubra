use common::consts::FILL;
use common::types::format::trimf64;
use common::types::path::Path;
use common::types::point::{PATH_WIDTH, Point, S2, S3, TILE_WIDTH};

fn a135_base(invert: bool) -> (String, String, String, String, Path) {
    let w = PATH_WIDTH;
    let r1 = 0.3333; // Adjustable

    let r2 = (1. + 2. * r1) / (4. + 12. * r1);

    let p1 = Point { x: 0.5 - w, y: S3 };
    let p2 = Point {
        x: (1. - w + r1 * (5. + 12. * r1 - 12. * (1. + 2. * r1) * w))
            / (2. + 12. * r1 * (1. + 2. * r1)),
        y: (S2 * (1. + w + r1 * (5. + 8. * r1 + 4. * w))) / (2. + 12. * r1 * (1. + 2. * r1)),
    };
    let p3 = Point {
        x: (2. + 7. * r1 - 2. * w - 6. * r1 * w) / (4. + 12. * r1),
        y: S2 / 4. * (1. - 2. * w),
    };
    let p4 = Point {
        x: (5. + 16. * r1) / (8. + 24. * r1) - w,
        y: (S2 * (1. + 4. * r1)) / (8. + 24. * r1),
    };
    let p5 = Point {
        x: (2. + 2. * r1 * (5. + r1 * (9. - 6. * w)) + w) / (2. + 12. * r1 * (1. + 2. * r1)),
        y: -(S2 * (w + 8. * r1 * w + 2. * r1 * r1 * (-1. + 6. * w)))
            / (2. + 12. * r1 * (1. + 2. * r1)),
    };
    let p6 = Point {
        x: 1. - w / 2.,
        y: -(S2 * w) / 2.,
    };

    let p7 = Point {
        x: (2. + w) / 2.,
        y: (S2 * w) / 2.,
    };
    let p8 = Point {
        x: (2. - w + 2. * r1 * (5. + r1 * (9. + 6. * w))) / (2. + 12. * r1 * (1. + 2. * r1)),
        y: (S2 * (w + 2. * r1 * (r1 + 4. * w + 6. * r1 * w))) / (2. + 12. * r1 * (1. + 2. * r1)),
    };
    let p9 = Point {
        x: 1. / 4. * (3. + 2. * w),
        y: 1. / 4. * S2 * (1. - 2. * w),
    };
    let p10 = Point {
        x: 1. / 4. * (3. - 2. * w),
        y: 1. / 4. * S2 * (1. + 2. * w),
    };
    let p11 = Point {
        x: (1. + w + r1 * (5. + 12. * w + 12. * r1 * (1. + 2. * w)))
            / (2. + 12. * r1 * (1. + 2. * r1)),
        y: S2 * (1. + r1 * (5. + 8. * r1 - 4. * w) - w) / (2. + 12. * r1 * (1. + 2. * r1)),
    };
    let p12 = Point {
        x: 1. / 2. + w,
        y: S3,
    };

    let pts = Path::new(vec![p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12]);

    let pts = if invert { pts.invert() } else { pts };
    let r1s = trimf64(TILE_WIDTH * (r1 - w));
    let r2s = trimf64(TILE_WIDTH * (r1 + w));
    let r3s = trimf64(TILE_WIDTH * (r2 - w));
    let r4s = trimf64(TILE_WIDTH * (r2 + w));
    return (r1s, r2s, r3s, r4s, pts);
}

fn a135_str(invert: bool, pts: Path, r1: String, r2: String, r3: String, r4: String) -> String {
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

pub fn n124(invert: bool) -> String {
    let (r1, r2, r3, r4, pts) = a135_base(invert);
    let pts = pts.reflect();
    return a135_str(invert, pts, r1, r2, r3, r4);
}

pub fn n236(invert: bool) -> String {
    let (r1, r2, r3, r4, pts) = a135_base(invert);
    let pts = pts
        .reflect()
        .rotate_ccw(invert, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
    return a135_str(invert, pts, r1, r2, r3, r4);
}

pub fn n135(invert: bool) -> String {
    let (r1, r2, r3, r4, pts) = a135_base(invert);
    let pts = pts
        .reflect()
        .rotate_cw(invert, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
    return a135_str(invert, pts, r1, r2, r3, r4);
}

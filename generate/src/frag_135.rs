use common::consts::FILL;
use common::define_shape_family;
use common::macros::{reflect, reflect_then_rotate_ccw, reflect_then_rotate_cw};
use common::types::format::trimf64;
use common::types::path::Path;
use common::types::point::{Point, PATH_WIDTH, S2, S3, TILE_WIDTH};

fn base(invert: bool) -> (Vec<String>, Path) {
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
    (vec![r1s, r2s, r3s, r4s], pts)
}

fn svg(invert: bool, radii: Vec<String>, pts: Path) -> String {
    let (d1, d2) = if invert { (0, 1) } else { (1, 0) };
    let pts = pts.str();
    format!(
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
        r1 = radii[0],
        r2 = radii[1],
        r3 = radii[2],
        r4 = radii[3],
        fill = FILL,
    )
}

define_shape_family! {
    base: base,
    svg: svg,
    offset_rotations: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
    variants: {
        n124: false, false => reflect,
        n236: false, false => reflect_then_rotate_ccw,
        n135: false, false => reflect_then_rotate_cw,
    }
}

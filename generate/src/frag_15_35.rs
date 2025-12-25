use common::consts::FILL;
use common::types::format::trimf64;
use common::types::path::Path;
use common::types::point::{Point, PATH_WIDTH, S2, S3, TILE_WIDTH};

fn base(invert: bool) -> (Path, String, String, String, String) {
    let w = PATH_WIDTH;

    let r1: f64 = 0.166667f64 + w;


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
        x: 0.75f64 + (0.5f64 * w),
        y: (0.25f64 * S2) + ((-0.5f64) * S2 * w),
    };
    let pt4 = Point {
        x: 1.5f64
            + (((-1.)
                + ((-5.) * r1)
                + ((-12.) * (r1 * r1))
                + ((-1.) * w)
                + ((-12.) * r1 * w)
                + ((-24.) * (r1 * r1) * w))
                / (2. + (12. * r1) + (24. * (r1 * r1)))),
        y: S3
            + (((1.5f64
                * (1. + (4. * r1))
                * (1. + (5. * r1) + (8. * (r1 * r1)) + ((-1.) * w) + ((-4.) * r1 * w)))
                / (1. + (6. * r1) + (12. * (r1 * r1))))
                / (((-1.) * S2) + ((-4.) * S2 * r1))),
    };
    let pt5 = Point {
        x: 1. + ((-1.) * w),
        y: 0.,
    };
    let pt6 = Point { x: 1. + w, y: 0. };
    let pt7 = Point {
        x: 1.5f64
            + (((-1.)
                + ((-1.) * (r1 * r1) * (12. + ((-24.) * w)))
                + ((-1.) * r1 * (5. + ((-12.) * w)))
                + w)
                / (2. + (12. * r1) + (24. * (r1 * r1)))),
        y: S3
            + (((-1.) * S2 * (1. + (8. * (r1 * r1)) + w + (r1 * (5. + (4. * w)))))
                / (2. + (12. * r1) + (24. * (r1 * r1)))),
    };
    let pt8 = Point {
        x: 0.25f64 * (3. + ((-2.) * w)),
        y: 0.25f64 * S2 * (1. + (2. * w)),
    };
    let pt9 = Point {
        x: (1. + (5. * r1) + (12. * (r1 * r1)) + w + (12. * r1 * w) + (24. * (r1 * r1) * w))
            / (2. + (12. * r1) + (24. * (r1 * r1))),
        y: (((-1.5f64)
            * (1. + (4. * r1))
            * (1. + (5. * r1) + (8. * (r1 * r1)) + ((-1.) * w) + ((-4.) * r1 * w)))
            / (1. + (6. * r1) + (12. * (r1 * r1))))
            / (((-1.) * S2) + ((-4.) * S2 * r1)),
    };
    let pt10 = Point {
        x: 0.5f64 + w,
        y: S3,
    };

    let pts = Path::new(vec![pt1, pt2, pt3, pt4, pt5, pt6, pt7, pt8, pt9, pt10]);

    let pts = if invert { pts.invert() } else { pts };

    let r1l = trimf64(TILE_WIDTH * (r1 + w));
    let r2l = trimf64(TILE_WIDTH * (r2 + w));
    let r1s = trimf64(TILE_WIDTH * (r1 - w));
    let r2s = trimf64(TILE_WIDTH * (r2 - w));

    return (pts, r1s, r1l, r2s, r2l);
}

fn svg(invert: bool, pts: Path, r1s: String, r1l: String, r2s: String, r2l: String) -> String {
    let pts = pts.str();
    let (d1, d2) = if invert { (0, 1) } else { (1, 0) };
    return format!(
        "\
        <path fill='{FILL}'  d='\
           M {a} \
           A {r1s} {r1s} 0 0 {d2} {b} \
           A {r2l} {r2l} 0 0 {d1} {c} \
           A {r2s} {r2s} 0 0 {d2} {d} \
           A {r1l} {r1l} 0 0 {d1} {e} \
           L {f}
           A {r1s} {r1s} 0 0 {d2} {g} \
           A {r2l} {r2l} 0 0 {d1} {h} \
           A {r2s} {r2s} 0 0 {d2} {i} \
           A {r1l} {r1l} 0 0 {d1} {j} \
           ' \
           />\
        <path stroke='black'  d='\
           M {a} \
           A {r1s} {r1s} 0 0 {d2} {b} \
           A {r2l} {r2l} 0 0 {d1} {c} \
           A {r2s} {r2s} 0 0 {d2} {d} \
           A {r1l} {r1l} 0 0 {d1} {e} \
           M {f}
           A {r1s} {r1s} 0 0 {d2} {g} \
           A {r2l} {r2l} 0 0 {d1} {h} \
           A {r2s} {r2s} 0 0 {d2} {i} \
           A {r1l} {r1l} 0 0 {d1} {j} \
           ' \
           />\
           ",
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
    );
}
pub fn n15_35(invert: bool) -> String {
    let (pts, r1, r2, r3, r4) = base(invert);
    return svg(invert, pts, r1, r2, r3, r4);
}

pub fn n24_14(invert: bool) -> String {
    let (pts, r1, r2, r3, r4) = base(invert);
    let pts = pts.rotate_ccw(invert, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    return svg(invert, pts, r1, r2, r3, r4);
}

pub fn n36_26(invert: bool) -> String {
    let (pts, r1, r2, r3, r4) = base(invert);
    let pts = pts.rotate_cw(invert, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    return svg(invert, pts, r1, r2, r3, r4);
}

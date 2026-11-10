use common::consts::FILL;
use common::define_shape_family;
use common::macros::{identity, rotate_ccw, rotate_cw};
use common::types::format::trimf64;
use common::types::path::Path;
use common::types::point::{Point, PATH_WIDTH, S2, S3, S4, TILE_WIDTH};

fn base(invert: bool) -> (Vec<String>, Path) {
    let w = PATH_WIDTH;
    let pt1 = Point {
        x: 0.5f64 + -w,
        y: S3,
    };
    let pt2 = Point {
        x: 0.5f64 + -w,
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
        x: 0.333333f64,
        y: (-0.166667f64) + S4 + -w,
    };
    let pt6 = Point {
        x: 0.5f64 + -w,
        y: (-0.333333f64) + S4,
    };
    let pt7 = Point {
        x: 0.5f64 + -w,
        y: 0.,
    };
    let pt8 = Point {
        x: 0.5f64 + w,
        y: 0.,
    };
    let pt9 = Point {
        x: 0.5f64 + w,
        y: (-0.333333f64) + S4,
    };
    let pt10 = Point {
        x: 0.666667f64,
        y: (-0.166667f64) + S4 + -w,
    };
    let pt11 = Point {
        x: 0.25f64 * (3. + (2. * w)),
        y: 0.25f64 * S2 * (1. + ((-2.) * w)),
    };
    let pt12 = Point {
        x: 0.25f64 * (3. + ((-2.) * w)),
        y: 0.25f64 * S2 * (1. + (2. * w)),
    };
    let pt13 = Point {
        x: 0.5f64 + w,
        y: S4,
    };
    let pt14 = Point {
        x: 0.5f64 + w,
        y: S3,
    };

    let pts = Path::new(vec![
        pt1, pt2, pt3, pt4, pt5, pt6, pt7, pt8, pt9, pt10, pt11, pt12, pt13, pt14,
    ]);

    let pts = if invert { pts.invert() } else { pts };
    let r1s = trimf64(TILE_WIDTH * (1. / 6. - w));
    let r1l = trimf64(TILE_WIDTH * (1. / 6. + w));
    (vec![r1s, r1l], pts)
}

fn svg(invert: bool, radii: Vec<String>, pts: Path) -> String {
    let (d1, d2) = if invert { (0, 1) } else { (1, 0) };
    let r1s = &radii[0];
    let r1l = &radii[1];
    let pts = pts.str();
    format!(
        "<path fill='{FILL}' d='M{a}L{b}A{r1s} {r1s} 0 0 {d1} {c}L{d}A{r1l} {r1l} 0 0 {d2} {e}A{r1s} {r1s} 0 0 {d1} {f}L{g}L{h}L{i}A{r1s} {r1s} 0 0 {d1} {j}A{r1l} {r1l} 0 0 {d2} {k}L{l}A{r1s} {r1s} 0 0 {d1} {m}L{n}'/>
        <path stroke='black' d='M{a}L{b}A{r1s} {r1s} 0 0 {d1} {c}M{d}A{r1l} {r1l} 0 0 {d2} {e}A{r1s} {r1s} 0 0 {d1} {f}L{g}M{h}L{i}A{r1s} {r1s} 0 0 {d1} {j}A{r1l} {r1l} 0 0 {d2} {k}M{l}A{r1s} {r1s} 0 0 {d1} {m}L{n}'/>",
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
    )
}

define_shape_family! {
    base: base,
    svg: svg,
    offset_rotations: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13],
    variants: {
        n1456: false, true => identity,
        n2456: false, true => rotate_ccw,
        n3456: false, true => rotate_cw,
    }
}

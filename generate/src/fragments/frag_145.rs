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
        x: 0.25f64 + ((-0.5f64) * w),
        y: (0.25f64 * S2) + ((-0.5f64) * S2 * w),
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

    let pts = Path::new(vec![pt1, pt2, pt3, pt4, pt7, pt8, pt9, pt10]);

    let pts = if invert { pts.invert() } else { pts };
    let r1s = trimf64(TILE_WIDTH * (1. / 6. - w));
    // The wall facing edge 6 is the outer wall of a plain 4-5 arc: its ends (pt4, pt7)
    // are both (0.5 + w) from vertex 1, so it is one arc centred on that vertex.
    let r_outer = trimf64(TILE_WIDTH * (0.5 + w));
    (vec![r1s, r_outer], pts)
}

fn svg(invert: bool, radii: Vec<String>, pts: Path) -> String {
    let (d1, d2) = if invert { (0, 1) } else { (1, 0) };
    let pts = pts.str();
    format!(
        "<path fill='{fill}' d='M{a}L{b}A{r1s} {r1s} 0 0 {d1} {c}L{d}A{r_outer} {r_outer} 0 0 {d2} {e}L{f}A{r1s} {r1s} 0 0 {d1} {g}L{h}'/><path stroke='black' d='M{a}L{b}A{r1s} {r1s} 0 0 {d1} {c}M{d}A{r_outer} {r_outer} 0 0 {d2} {e}M{f}A{r1s} {r1s} 0 0 {d1} {g}L{h}'/>",
        a = pts[0],
        b = pts[1],
        c = pts[2],
        d = pts[3],
        e = pts[4],
        f = pts[5],
        g = pts[6],
        h = pts[7],
        fill = FILL,
        r1s = radii[0],
        r_outer = radii[1],
    )
}

define_shape_family! {
    base: base,
    svg: svg,
    offset_rotations: &[0, 1, 2, 3, 4, 5, 6, 7],
    variants: {
        n145: false, true => identity,
        n246: false, true => rotate_ccw,
        n356: false, true => rotate_cw,
    }
}

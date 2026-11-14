use common::consts::FILL;
use common::define_shape_family;
use common::macros::{identity, rotate_ccw, rotate_cw};
use common::types::format::trimf64;
use common::types::path::Path;
use common::types::point::{Point, PATH_WIDTH, S2, S3, S4, TILE_WIDTH};

/// Radius of the corner wall between each side opening and the bottom opening
/// (35/120 of a tile). Larger is a gentler bend with shorter straights; at about
/// 49/120 the straights vanish.
pub const SIDE_CORNER_RADIUS: f64 = 35.0 / TILE_WIDTH;

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
    // The wall from pt4 runs at 30 degrees until it meets the vertical wall
    // x = 0.5 - w at `corner`. A single arc rounds that 60 degree turn; its
    // tangent points lie `along` from the corner on each straight.
    let corner = Point {
        x: 0.5f64 + -w,
        y: pt4.y - (0.5f64 + -w - pt4.x) * S4,
    };
    let along = SIDE_CORNER_RADIUS * S4;
    let pt5 = Point {
        x: corner.x - along * S3,
        y: corner.y + along * 0.5,
    };
    let pt6 = Point {
        x: corner.x,
        y: corner.y - along,
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
        x: 1. - pt6.x,
        y: pt6.y,
    };
    let pt10 = Point {
        x: 1. - pt5.x,
        y: pt5.y,
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
    let rc = trimf64(TILE_WIDTH * SIDE_CORNER_RADIUS);
    (vec![r1s, rc], pts)
}

fn svg(invert: bool, radii: Vec<String>, pts: Path) -> String {
    let d1 = if invert { 0 } else { 1 };
    let r1s = &radii[0];
    let rc = &radii[1];
    let pts = pts.str();
    format!(
        "<path fill='{FILL}' d='M{a}L{b}A{r1s} {r1s} 0 0 {d1} {c}L{d}L{e}A{rc} {rc} 0 0 {d1} {f}L{g}L{h}L{i}A{rc} {rc} 0 0 {d1} {j}L{k}L{l}A{r1s} {r1s} 0 0 {d1} {m}L{n}'/>
        <path stroke='black' d='M{a}L{b}A{r1s} {r1s} 0 0 {d1} {c}M{d}L{e}A{rc} {rc} 0 0 {d1} {f}L{g}M{h}L{i}A{rc} {rc} 0 0 {d1} {j}L{k}M{l}A{r1s} {r1s} 0 0 {d1} {m}L{n}'/>",
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

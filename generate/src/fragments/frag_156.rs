use common::consts::FILL;
use common::define_shape_family;
use common::macros::{
    identity, reflect, reflect_then_rotate_ccw, reflect_then_rotate_cw, rotate_ccw, rotate_cw,
};
use common::types::format::trimf64;
use common::types::path::Path;
use common::types::point::{Point, PATH_WIDTH, S2, S3, S4, TILE_WIDTH};

/// Vertex 1, the edge beside it (5) and the edge opposite it (6).
///
/// Vertex 1 and the midpoint of edge 6 lie on a straight line, so the track between
/// them is drawn straight. The branch to edge 5 turns out of its side.
fn base(invert: bool) -> (Vec<String>, Path) {
    let w = PATH_WIDTH;

    // Straight wall from the vertex down to edge 6.
    let pt1 = Point {
        x: 0.5f64 - w,
        y: S3,
    };
    let pt2 = Point {
        x: 0.5f64 - w,
        y: 0.,
    };
    // Inner wall of the 5-6 arc (centred on vertex 3).
    let pt3 = Point {
        x: 0.5f64 + w,
        y: 0.,
    };
    let pt4 = Point {
        x: 0.25f64 * (3. + (2. * w)),
        y: 0.25f64 * S2 * (1. - (2. * w)),
    };
    // Turn-out from edge 5 back up to the vertex.
    let pt5 = Point {
        x: 0.25f64 * (3. - (2. * w)),
        y: 0.25f64 * S2 * (1. + (2. * w)),
    };
    let pt6 = Point {
        x: 0.5f64 + w,
        y: S4,
    };
    let pt7 = Point {
        x: 0.5f64 + w,
        y: S3,
    };

    let pts = Path::new(vec![pt1, pt2, pt3, pt4, pt5, pt6, pt7]);

    let pts = if invert { pts.invert() } else { pts };
    let r_turn = trimf64(TILE_WIDTH * (1. / 6. - w));
    let r_arc = trimf64(TILE_WIDTH * (0.5f64 - w));
    (vec![r_turn, r_arc], pts)
}

fn svg(invert: bool, radii: Vec<String>, pts: Path) -> String {
    let sweep = if invert { 1 } else { 0 };
    let pts = pts.str();
    format!(
        "\
       <path fill='{fill}' d='M{a}L{b}L{c}A{r_arc} {r_arc} 0 0 {sweep} {d}L{e}A{r_turn} {r_turn} 0 0 {sweep} {f}L{g}Z'/>\
       <path stroke='black' d='M{a}L{b}M{c}A{r_arc} {r_arc} 0 0 {sweep} {d}M{e}A{r_turn} {r_turn} 0 0 {sweep} {f}L{g}'/>",
        a = pts[0],
        b = pts[1],
        c = pts[2],
        d = pts[3],
        e = pts[4],
        f = pts[5],
        g = pts[6],
        fill = FILL,
        r_turn = radii[0],
        r_arc = radii[1],
    )
}

define_shape_family! {
    base: base,
    svg: svg,
    offset_rotations: &[0, 1, 2, 3, 4, 5, 6],
    variants: {
        n156: false, false => identity,
        n245: false, false => rotate_ccw,
        n346: false, false => rotate_cw,
        n146: false, true => reflect,
        n256: false, true => reflect_then_rotate_ccw,
        n345: false, true => reflect_then_rotate_cw,
    }
}

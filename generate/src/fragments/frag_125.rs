use common::consts::FILL;
use common::define_shape_family;
use common::macros::{
    identity, reflect, reflect_then_rotate_ccw, reflect_then_rotate_cw, rotate_ccw, rotate_cw,
};
use common::types::format::trimf64;
use common::types::path::Path;
use common::types::point::{Point, PATH_WIDTH, S2, S3, S4, TILE_WIDTH};

/// Vertices 1 and 2, and edge 5 (beside vertex 1, opposite vertex 2).
///
/// The outer wall is the plain 1-2 track's arc. Vertex 2 and the midpoint of edge 5
/// lie on a straight line, so that wall is straight. The branch to edge 5 turns out
/// beside vertex 1.
fn base(invert: bool) -> (Vec<String>, Path) {
    let w = PATH_WIDTH;

    // Outer wall of the 1-2 track: an arc of radius (1 - w) centred on vertex 3
    // mirrored across edge 1-2, from beside vertex 1 to beside vertex 2.
    let pt1 = Point {
        x: 0.5f64 - w,
        y: S3,
    };
    let pt2 = w * Point { x: -0.5, y: S3 };
    // Straight wall from vertex 2 to edge 5.
    let pt3 = w * Point { x: 0.5, y: -S3 };
    let pt4 = Point {
        x: 0.25f64 * (3. + (2. * w)),
        y: 0.25f64 * S2 * (1. - (2. * w)),
    };
    // Turn-out from edge 5 back up to vertex 1.
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
    let r_track = trimf64(TILE_WIDTH * (1. - w));
    let r_turn = trimf64(TILE_WIDTH * (1. / 6. - w));
    (vec![r_track, r_turn], pts)
}

fn svg(invert: bool, radii: Vec<String>, pts: Path) -> String {
    let sweep = if invert { 0 } else { 1 };
    let pts = pts.str();
    format!(
        "\
       <path fill='{fill}' d='M{a}A{r_track} {r_track} 0 0 {sweep} {b}L{c}L{d}L{e}A{r_turn} {r_turn} 0 0 {sweep} {f}L{g}Z'/>\
       <path stroke='black' d='M{a}A{r_track} {r_track} 0 0 {sweep} {b}M{c}L{d}M{e}A{r_turn} {r_turn} 0 0 {sweep} {f}L{g}'/>",
        a = pts[0],
        b = pts[1],
        c = pts[2],
        d = pts[3],
        e = pts[4],
        f = pts[5],
        g = pts[6],
        fill = FILL,
        r_track = radii[0],
        r_turn = radii[1],
    )
}

define_shape_family! {
    base: base,
    svg: svg,
    offset_rotations: &[0, 1, 2, 3, 4, 5, 6],
    variants: {
        n125: false, true => identity,
        n134: false, false => reflect,
        n126: false, false => reflect_then_rotate_ccw,
        n234: false, true => rotate_ccw,
        n235: false, false => reflect_then_rotate_cw,
        n136: false, true => rotate_cw,
    }
}

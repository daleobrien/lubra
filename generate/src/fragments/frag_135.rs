use common::consts::FILL;
use common::define_shape_family;
use common::macros::{reflect, reflect_then_rotate_ccw, reflect_then_rotate_cw};
use common::types::format::trimf64;
use common::types::path::Path;
use common::types::point::{Point, PATH_WIDTH, S2, S3, S4, TILE_WIDTH};

/// Vertices 1 and 3, and edge 5 between them.
///
/// The outer wall is the plain 1-3 track's arc. The branch to edge 5 has a turn-out
/// from each vertex; the two are mirror images in the line from vertex 2 to edge 5.
fn base(invert: bool) -> (Vec<String>, Path) {
    let w = PATH_WIDTH;

    // Mirror in the line through vertex 2 (the origin) and the midpoint of edge 5,
    // which runs at 30 degrees. It swaps vertices 1 and 3.
    let mirror = |p: Point| Point {
        x: (0.5 * p.x) + (S3 * p.y),
        y: (S3 * p.x) - (0.5 * p.y),
    };

    let beside_vertex_outer = Point {
        x: 0.5f64 - w,
        y: S3,
    };
    let beside_vertex_inner = Point {
        x: 0.5f64 + w,
        y: S3,
    };
    let turn_start = Point {
        x: 0.5f64 + w,
        y: S4,
    };
    let edge_upper = Point {
        x: 0.25f64 * (3. - (2. * w)),
        y: 0.25f64 * S2 * (1. + (2. * w)),
    };

    let pts = Path::new(vec![
        // Outer wall: an arc of radius (1 + w) centred on vertex 2 mirrored across edge 1-3.
        beside_vertex_outer,
        mirror(beside_vertex_outer),
        // Turn-out from vertex 3 to edge 5.
        mirror(beside_vertex_inner),
        mirror(turn_start),
        mirror(edge_upper),
        // Turn-out from edge 5 to vertex 1.
        edge_upper,
        turn_start,
        beside_vertex_inner,
    ]);

    let pts = if invert { pts.invert() } else { pts };
    let r_track = trimf64(TILE_WIDTH * (1. + w));
    let r_turn = trimf64(TILE_WIDTH * (1. / 6. - w));
    (vec![r_track, r_turn], pts)
}

fn svg(invert: bool, radii: Vec<String>, pts: Path) -> String {
    // Every variant of this family is a reflection of the base shape.
    let (outer, turn) = if invert { (1, 0) } else { (0, 1) };
    let pts = pts.str();
    format!(
        "\
       <path fill='{fill}' d='M{a}A{r_track} {r_track} 0 0 {outer} {b}L{c}L{d}A{r_turn} {r_turn} 0 0 {turn} {e}L{f}A{r_turn} {r_turn} 0 0 {turn} {g}L{h}Z'/>\
       <path stroke='black' d='M{a}A{r_track} {r_track} 0 0 {outer} {b}M{c}L{d}A{r_turn} {r_turn} 0 0 {turn} {e}M{f}A{r_turn} {r_turn} 0 0 {turn} {g}L{h}'/>",
        a = pts[0],
        b = pts[1],
        c = pts[2],
        d = pts[3],
        e = pts[4],
        f = pts[5],
        g = pts[6],
        h = pts[7],
        fill = FILL,
        r_track = radii[0],
        r_turn = radii[1],
    )
}

define_shape_family! {
    base: base,
    svg: svg,
    offset_rotations: &[0, 1, 2, 3, 4, 5, 6, 7],
    variants: {
        n124: false, false => reflect,
        n236: false, false => reflect_then_rotate_ccw,
        n135: false, false => reflect_then_rotate_cw,
    }
}

//! A dead end reached by a U-turn round a tile corner (`15/15/5` and its rotations
//! and reflections).
//!
//! The track arrives through the edge of the tile diagonally opposite across a vertex,
//! turns once round that vertex, then runs straight towards the dead-end node and
//! stops short of it with a round cap. It replaces two pieces that met at the vertex,
//! a hook into the vertex and a curl out of it, whose turns went opposite ways and left
//! a bump in the walls; this track only ever turns one way.

use common::consts::FILL;
use common::define_shape_family;
use common::macros::{
    identity, reflect, reflect_then_rotate_ccw, reflect_then_rotate_cw, rotate_ccw, rotate_cw,
};
use common::types::format::trimf64;
use common::types::path::Path;
use common::types::point::{Point, PATH_WIDTH, S3, S5, TILE_WIDTH};

/// Radius of the U-turn round the vertex, to the middle of the track, as a fraction of
/// the tile width (about 35 of 120).
const U_TURN_RADIUS: f64 = S5;
/// How far short of the dead-end node the track stops, to the middle of its cap, as a
/// fraction of the tile width. The cap reaches `PATH_WIDTH` further.
const STOP_SHORT: f64 = 22.0 / TILE_WIDTH;

fn left_of(heading: f64) -> Point {
    Point::new(-heading.sin(), heading.cos())
}

fn dot(a: Point, b: Point) -> f64 {
    a.x * b.x + a.y * b.y
}

fn length(p: Point) -> f64 {
    dot(p, p).sqrt()
}

/// The point on the turn after turning `angle` (radians, positive) from `start`.
fn on_turn(centre: Point, start: Point, angle: f64, side: f64) -> Point {
    let from = start - centre;
    let (sin, cos) = (side * angle).sin_cos();
    centre + Point::new(from.x * cos - from.y * sin, from.x * sin + from.y * cos)
}

/// The track in the frame of `n15_15_5` on a normal tile (inverted when `invert`).
///
/// The tile's net joins vertex 1 to edge 5, the dead end is across edge 5, and the
/// track arrives from the tile opposite vertex 1. That tile's opening is the reflection,
/// through vertex 1, of the middle of edge 4.
///
/// Returns `[outer radius, inner radius, cap radius, turn sweep flag]` and the six wall
/// points
/// `[start outer, turn end outer, end outer, end inner, turn end inner, start inner]`.
fn base(invert: bool) -> (Vec<String>, Path) {
    let w = PATH_WIDTH;
    let r = U_TURN_RADIUS;
    let vertex = Point::new(0.5, S3);
    let dead_end = Point::new(0.75, S3 / 2.0);
    let start = vertex * 2.0 - Point::new(0.25, S3 / 2.0);

    // Arrive heading into the opposite tile, at right angles to its edge: away from
    // the line through the two vertices of that edge (vertex 1 and its mirror of vertex 2).
    let opposite_centre = vertex * 2.0 - Point::new(0.5, S3 / 3.0);
    let into = opposite_centre - start;
    let start_heading = into.y.atan2(into.x);
    // Turn towards the vertex: left (+1) if the vertex is on the left.
    let to_vertex = vertex - start;
    let side = if dot(left_of(start_heading), to_vertex) > 0.0 {
        1.0
    } else {
        -1.0
    };
    let centre = start + left_of(start_heading) * (side * r);

    // Find how far to turn so the track then points straight at the dead-end node.
    let aim = |angle: f64| {
        let at = on_turn(centre, start, angle, side);
        let heading = start_heading + side * angle;
        let to = dead_end - at;
        // Positive while the node is still on the turning side of the heading.
        side * dot(left_of(heading), to)
    };
    let (mut lo, mut hi) = (90f64.to_radians(), 200f64.to_radians());
    assert!(
        aim(lo) > 0.0 && aim(hi) < 0.0,
        "frag_corner_hook: no turn aims at the node"
    );
    for _ in 0..60 {
        let mid = (lo + hi) / 2.0;
        if aim(mid) > 0.0 {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    let turn = lo;
    let turn_end = on_turn(centre, start, turn, side);
    let heading = start_heading + side * turn;
    let run = length(dead_end - turn_end) - STOP_SHORT;
    assert!(
        run >= 0.0,
        "frag_corner_hook: STOP_SHORT is longer than the run"
    );
    let end = turn_end + Point::new(heading.cos(), heading.sin()) * run;

    // The outer wall is on the side away from the turn's centre.
    let out = |p: Point, h: f64| p - left_of(h) * (side * w);
    let inn = |p: Point, h: f64| p + left_of(h) * (side * w);
    let points = vec![
        out(start, start_heading),
        out(turn_end, heading),
        out(end, heading),
        inn(end, heading),
        inn(turn_end, heading),
        inn(start, start_heading),
    ];
    let pts = Path::new(points);
    let pts = if invert { pts.invert() } else { pts };
    // SVG sweep flag 1 is a left (counter-clockwise) turn.
    let turn_flag = if side > 0.0 { "1" } else { "0" };
    let radii = vec![
        trimf64(TILE_WIDTH * (r + w)),
        trimf64(TILE_WIDTH * (r - w)),
        trimf64(TILE_WIDTH * w),
        turn_flag.to_string(),
    ];
    (radii, pts)
}

fn svg(invert: bool, radii: Vec<String>, pts: Path) -> String {
    // The outer wall and the cap turn with the track; the inner wall is drawn back
    // the other way. A mirrored or inverted variant turns the other way.
    let base_flag: u8 = if radii[3] == "1" { 1 } else { 0 };
    let turn = if invert { 1 - base_flag } else { base_flag };
    let back = 1 - turn;
    let p = pts.str();
    let (outer, inner, cap) = (&radii[0], &radii[1], &radii[2]);
    let walls = format!(
        "M{}A{outer} {outer} 0 0 {turn} {}L{}A{cap} {cap} 0 0 {turn} {}L{}A{inner} {inner} 0 0 {back} {}",
        p[0], p[1], p[2], p[3], p[4], p[5]
    );
    format!("<path fill='{FILL}' d='{walls}z'/><path stroke='black' d='{walls}'/>")
}

define_shape_family! {
    base: base,
    svg: svg,
    offset_rotations: &[0, 1, 2, 3, 4, 5],
    variants: {
        n15_15_5: false, false => identity,
        n24_24_4: false, false => rotate_ccw,
        n36_36_6: false, false => rotate_cw,
        n14_14_4: false, true => reflect,
        n26_26_6: false, true => reflect_then_rotate_ccw,
        n35_35_5: false, true => reflect_then_rotate_cw,
    }
}

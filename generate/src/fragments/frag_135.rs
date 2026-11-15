//! The junction that joins two vertices of a tile and drops to the middle of the edge
//! between them (`135` and its rotations `124` and `236`).
//!
//! The track between the vertices is one arch: it lifts out of each vertex on a side
//! curve, crosses over on a top curve, and the drop meets the top curve's lower wall
//! with a rounded corner on each side. The arch has to be high enough in the middle
//! for those corners to fit above the edge, which is why the sides lift first. The
//! track keeps its full width everywhere.

use common::consts::FILL;
use common::define_shape_family;
use common::macros::{reflect, reflect_then_rotate_ccw, reflect_then_rotate_cw};
use common::types::format::trimf64;
use common::types::path::Path;
use common::types::point::{Point, PATH_WIDTH, TILE_WIDTH};

/// Radius of the side curves that lift the track out of each vertex, to the middle of
/// the track, as a fraction of the tile width.
const SIDE_RADIUS: f64 = 60.0 / TILE_WIDTH;
/// How far the side curves turn, in degrees. More turn lifts the arch higher.
const SIDE_TURN: f64 = 20.0;
/// Radius of the rounded corners where the drop meets the arch (the wall itself), as a
/// fraction of the tile width. A wider track leaves less room above the edge, so the
/// corners shrink to the largest radius that fits, up to this (see `corner_radius`).
const CORNER_RADIUS: f64 = 13.0 / TILE_WIDTH;

/// The top curve's centre and radius (to the middle of the track), in the local frame
/// of `local_arch`.
fn top_curve() -> (Point, f64) {
    let a = SIDE_RADIUS;
    let start_heading = 30f64.to_radians();
    let top_heading = start_heading + SIDE_TURN.to_radians();
    let side_centre = left_of(start_heading) * a;
    let side_end = side_centre - left_of(top_heading) * a;
    let right = left_of(top_heading) * -1.0;
    let top_radius = (0.5 - side_end.x) / right.x;
    (side_end + right * top_radius, top_radius)
}

/// Where a corner of `radius` sits, against the drop's left wall and inside the top
/// curve's lower wall, for a track of half-width `w`: its centre, if it fits above the
/// edge.
fn corner_centre(radius: f64, w: f64) -> Option<Point> {
    let (top_centre, top_radius) = top_curve();
    let lower = top_radius - w;
    let corner_x = 0.5 - w - radius;
    let reach = (lower - radius).powi(2) - (corner_x - top_centre.x).powi(2);
    let corner_y = top_centre.y + reach.max(0.0).sqrt();
    (reach >= 0.0 && corner_y >= 0.0).then(|| Point::new(corner_x, corner_y))
}

/// The corners' radius for a track of half-width `w`: `CORNER_RADIUS`, or the largest
/// radius below it that fits above the edge (to a hundredth of a unit).
fn corner_radius(w: f64) -> f64 {
    if corner_centre(CORNER_RADIUS, w).is_some() {
        return CORNER_RADIUS;
    }
    let (mut fits, mut too_big) = (0.0, CORNER_RADIUS);
    while (too_big - fits) * TILE_WIDTH > 0.01 {
        let middle = (fits + too_big) / 2.0;
        if corner_centre(middle, w).is_some() {
            fits = middle;
        } else {
            too_big = middle;
        }
    }
    assert!(
        fits > 0.0,
        "frag_135: no rounded corner fits a track this wide"
    );
    fits
}

fn left_of(heading: f64) -> Point {
    Point::new(-heading.sin(), heading.cos())
}

fn length(p: Point) -> f64 {
    (p.x * p.x + p.y * p.y).sqrt()
}

/// The arch in a local frame: the vertices at (0, 0) and (1, 0), the drop at (0.5, 0),
/// the tile above. Returns the wall radii `[side inner, side outer, top outer,
/// top inner, corner]` and the fourteen wall points
/// `[top start, top side end, top side end', top start', bottom start',
/// bottom side end', corner start', corner end', drop right, drop left, corner end,
/// corner start, bottom side end, bottom start]` (primes on the right-hand side).
fn local_arch() -> ([f64; 5], Vec<Point>) {
    let w = PATH_WIDTH;
    let a = SIDE_RADIUS;
    let start_heading = 30f64.to_radians();
    let top_heading = start_heading + SIDE_TURN.to_radians();

    // Side curve (a left turn) out of the vertex at the origin.
    let side_centre = left_of(start_heading) * a;
    let side_end = side_centre - left_of(top_heading) * a;
    // Top curve (a right turn) centred on the middle of the chord.
    let (top_centre, top_radius) = top_curve();

    // Rounded corner: inside the top curve's lower wall, against the drop's left wall.
    let lower = top_radius - w;
    let corner = corner_radius(w);
    let corner_centre = corner_centre(corner, w).expect("corner_radius fits");
    let corner_y = corner_centre.y;
    let towards = (corner_centre - top_centre) * (1.0 / length(corner_centre - top_centre));
    let corner_start = top_centre + towards * lower;
    let corner_end = Point::new(0.5 - w, corner_y);

    let mirror = |p: Point| Point::new(1.0 - p.x, p.y);
    let top_start = left_of(start_heading) * w;
    let top_side_end = side_end + left_of(top_heading) * w;
    let bottom_start = left_of(start_heading) * -w;
    let bottom_side_end = side_end - left_of(top_heading) * w;

    let points = vec![
        top_start,
        top_side_end,
        mirror(top_side_end),
        mirror(top_start),
        mirror(bottom_start),
        mirror(bottom_side_end),
        mirror(corner_start),
        mirror(corner_end),
        Point::new(0.5 + w, 0.0),
        Point::new(0.5 - w, 0.0),
        corner_end,
        corner_start,
        bottom_side_end,
        bottom_start,
    ];
    let radii = [a - w, a + w, top_radius + w, lower, corner];
    (radii, points)
}

/// The junction in the family's base frame (nodes 1, 3 and 5 before the variants'
/// reflection), inverted when `invert`.
fn base(invert: bool) -> (Vec<String>, Path) {
    let (radii, local) = local_arch();
    // Lay the local chord along the edge from vertex 3 (1, 0) to vertex 1 (0.5, S3),
    // with the local "up" pointing into the tile: a rotation by 120 degrees.
    let (sin, cos) = 120f64.to_radians().sin_cos();
    let origin = Point::new(1.0, 0.0);
    let place = |p: Point| origin + Point::new(p.x * cos - p.y * sin, p.x * sin + p.y * cos);

    let pts = Path::new(local.into_iter().map(place).collect());
    let pts = if invert { pts.invert() } else { pts };
    let radii = radii.iter().map(|r| trimf64(TILE_WIDTH * r)).collect();
    (radii, pts)
}

fn svg(invert: bool, radii: Vec<String>, pts: Path) -> String {
    // Sweep flags for left (counter-clockwise) and right turns. Every variant is
    // reflected, which swaps them, and an inverted tile swaps them back.
    let (l, r) = if invert { (1, 0) } else { (0, 1) };
    let p = pts.str();
    let [side_in, side_out, top_out, top_in, corner] = radii.as_slice() else {
        panic!("frag_135: expected 5 radii");
    };
    let top = format!(
        "M{}A{side_in} {side_in} 0 0 {l} {}A{top_out} {top_out} 0 0 {r} {}A{side_in} {side_in} 0 0 {l} {}",
        p[0], p[1], p[2], p[3]
    );
    let bottom_right = format!(
        "M{}L{}A{corner} {corner} 0 0 {r} {}A{top_in} {top_in} 0 0 {r} {}A{side_out} {side_out} 0 0 {l} {}",
        p[8], p[7], p[6], p[5], p[4]
    );
    let bottom_left = format!(
        "M{}A{side_out} {side_out} 0 0 {l} {}A{top_in} {top_in} 0 0 {r} {}A{corner} {corner} 0 0 {r} {}L{}",
        p[13], p[12], p[11], p[10], p[9]
    );
    let outline = format!(
        "{top}L{}A{side_out} {side_out} 0 0 {r} {}A{top_in} {top_in} 0 0 {l} {}A{corner} {corner} 0 0 {l} {}\
         L{}L{}L{}A{corner} {corner} 0 0 {l} {}A{top_in} {top_in} 0 0 {l} {}A{side_out} {side_out} 0 0 {r} {}z",
        p[4], p[5], p[6], p[7], p[8], p[9], p[10], p[11], p[12], p[13]
    );
    format!(
        "<path fill='{FILL}' d='{outline}'/><path stroke='black' d='{top}{bottom_left}{bottom_right}'/>"
    )
}

define_shape_family! {
    base: base,
    svg: svg,
    offset_rotations: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13],
    variants: {
        n124: false, false => reflect,
        n236: false, false => reflect_then_rotate_ccw,
        n135: false, false => reflect_then_rotate_cw,
    }
}

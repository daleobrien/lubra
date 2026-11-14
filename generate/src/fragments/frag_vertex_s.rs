//! A track that passes through a tile vertex from one edge opening to another, drawn
//! as one S of two half-circles (`15/14` and its rotations).
//!
//! The net joins vertex 1 to edge 5 here and vertex 1 to edge 4 in the tile opposite
//! that vertex. Both openings point the same way and sit side by side (a tile width
//! apart, with no distance between them along the track), so the track has to turn a
//! half-circle one way and a half-circle back. It replaces two hooks that met at the
//! vertex with opposite turns and left a kink.
//!
//! Only this ordering draws: the tile opposite gets a blank fragment for the same track
//! (the generator adds it), so the S is drawn once.

use common::consts::FILL;
use common::define_shape_family;
use common::macros::{identity, rotate_ccw, rotate_cw};
use common::types::format::trimf64;
use common::types::path::Path;
use common::types::point::{Point, PATH_WIDTH, S3, TILE_WIDTH};

/// Radius of the first half-circle, from the opening in the opposite tile, to the
/// middle of the track, as a fraction of the tile width. The two radii add up to half a
/// tile width, so the second is `0.5 - FIRST_HALF_RADIUS`; equal halves meet at the vertex.
const FIRST_HALF_RADIUS: f64 = 30.0 / TILE_WIDTH;

fn left_of(heading: f64) -> Point {
    Point::new(-heading.sin(), heading.cos())
}

/// The point reached after turning `angle` (signed, positive is left) on a circle of
/// `radius` from `at`, heading `heading`; with the new heading.
fn turn(at: Point, heading: f64, radius: f64, angle: f64) -> (Point, f64) {
    let side = angle.signum();
    let centre = at + left_of(heading) * (side * radius);
    let new_heading = heading + angle;
    (centre - left_of(new_heading) * (side * radius), new_heading)
}

/// The S in the frame of `n15_14` on a normal tile (inverted when `invert`).
///
/// Returns the radii `[r1 + w, r1 - w, r2 - w, r2 + w]` and the wall points, five per
/// wall (start, quarter, middle, three quarters, end) for the wall left of the track
/// and then the wall right of it. Each half-circle is drawn as two quarter arcs.
fn base(invert: bool) -> (Vec<String>, Path) {
    let w = PATH_WIDTH;
    let r1 = FIRST_HALF_RADIUS;
    let r2 = 0.5 - r1;
    assert!(
        r2 > w && r1 > w,
        "frag_vertex_s: FIRST_HALF_RADIUS leaves a half too tight"
    );

    // The opening in the opposite tile is the reflection, through vertex 1, of the
    // middle of edge 5; the track arrives at right angles to it, heading 30 degrees.
    let vertex = Point::new(0.5, S3);
    let start = vertex * 2.0 - Point::new(0.75, S3 / 2.0);
    let heading = 30f64.to_radians();
    let half = std::f64::consts::FRAC_PI_2;

    // Right half-circle, then left half-circle, each split into quarters.
    let mut centre_line = vec![(start, heading)];
    for (radius, angle) in [(r1, -half), (r1, -half), (r2, half), (r2, half)] {
        let &(at, h) = centre_line.last().expect("starts with a point");
        centre_line.push(turn(at, h, radius, angle));
    }
    let left: Vec<Point> = centre_line
        .iter()
        .map(|&(p, h)| p + left_of(h) * w)
        .collect();
    let right: Vec<Point> = centre_line
        .iter()
        .map(|&(p, h)| p - left_of(h) * w)
        .collect();

    let pts = Path::new(left.into_iter().chain(right).collect());
    let pts = if invert { pts.invert() } else { pts };
    let radii = [r1 + w, r1 - w, r2 - w, r2 + w]
        .iter()
        .map(|r| trimf64(TILE_WIDTH * r))
        .collect();
    (radii, pts)
}

fn svg(invert: bool, radii: Vec<String>, pts: Path) -> String {
    // In the base frame the first half turns right (sweep 0), the second left (1).
    let (right_turn, left_turn) = if invert { (1, 0) } else { (0, 1) };
    let p = pts.str();
    let (r1_out, r1_in, r2_in, r2_out) = (&radii[0], &radii[1], &radii[2], &radii[3]);
    // The wall left of the track is outside the first turn and inside the second.
    let left = format!(
        "M{}A{r1_out} {r1_out} 0 0 {right_turn} {}A{r1_out} {r1_out} 0 0 {right_turn} {}\
         A{r2_in} {r2_in} 0 0 {left_turn} {}A{r2_in} {r2_in} 0 0 {left_turn} {}",
        p[0], p[1], p[2], p[3], p[4]
    );
    let right = format!(
        "M{}A{r1_in} {r1_in} 0 0 {right_turn} {}A{r1_in} {r1_in} 0 0 {right_turn} {}\
         A{r2_out} {r2_out} 0 0 {left_turn} {}A{r2_out} {r2_out} 0 0 {left_turn} {}",
        p[5], p[6], p[7], p[8], p[9]
    );
    // The fill goes out along the left wall and back along the right one.
    let back = format!(
        "L{}A{r2_out} {r2_out} 0 0 {right_turn} {}A{r2_out} {r2_out} 0 0 {right_turn} {}\
         A{r1_in} {r1_in} 0 0 {left_turn} {}A{r1_in} {r1_in} 0 0 {left_turn} {}z",
        p[9], p[8], p[7], p[6], p[5]
    );
    format!("<path fill='{FILL}' d='{left}{back}'/><path stroke='black' d='{left}{right}'/>")
}

define_shape_family! {
    base: base,
    svg: svg,
    offset_rotations: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
    variants: {
        n15_14: false, false => identity,
        n24_26: false, false => rotate_ccw,
        n36_35: false, false => rotate_cw,
    }
}

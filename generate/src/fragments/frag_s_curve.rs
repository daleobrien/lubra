//! The S-curve primitive: a track from one tile vertex to another that turns one way
//! on a tight curve, then back the other way on a wide curve.
//!
//! It draws the cross-tile combinations that used to be a tight loop followed
//! by a long arc (`frag_arc_long`). Those met at a short straight in the middle of the
//! tile, and the loop came close enough to a vertex-to-edge track on the same tile
//! for their walls to touch. Two tangent arcs give a smoother track with room beside it.
//!
//! Both ends keep the position and direction of the pieces it replaces, so the
//! neighbouring tiles that continue the track are unchanged.

use common::consts::FILL;
use common::define_shape_family;
use common::macros::{
    identity, reflect, reflect_then_rotate_ccw, reflect_then_rotate_cw, rotate_ccw, rotate_cw,
};
use common::types::format::trimf64;
use common::types::path::Path;
use common::types::point::{Point, PATH_WIDTH, S3, TILE_WIDTH};

/// Radius of the tight curve, to the middle of the track, as a fraction of the tile
/// width. The wide curve's radius follows from it: with both ends fixed, a larger
/// tight curve makes the wide curve smaller and brings the track closer to a track
/// beside it. At 26/120 the wide curve is about 56/120 and the white gap to the
/// vertex-to-edge track on the same tile is about 9 units; above about 30/120 they touch.
pub const S_CURVE_TIGHT_RADIUS: f64 = 26.0 / TILE_WIDTH;

/// The left-hand normal of a track heading at angle `a`.
fn left_of(a: f64) -> Point {
    Point::new(-a.sin(), a.cos())
}

fn dot(a: Point, b: Point) -> f64 {
    a.x * b.x + a.y * b.y
}

fn length(a: Point) -> f64 {
    dot(a, a).sqrt()
}

/// The angle swept from `from` to `to` around `centre`, turning left (`turn` > 0) or
/// right, in 0..2π.
fn swept(centre: Point, from: Point, to: Point, turn: f64) -> f64 {
    let (a, b) = (from - centre, to - centre);
    let angle = b.y.atan2(b.x) - a.y.atan2(a.x);
    let angle = if turn > 0.0 { angle } else { -angle };
    angle.rem_euclid(std::f64::consts::TAU)
}

/// The radii of the walls (`[tight left, wide left, tight right, wide right]`), the
/// large-arc flags of the two curves and the six wall points
/// `[start left, bend left, end left, end right, bend right, start right]`, all in the
/// frame of `n45_14_35` on a normal tile (inverted when `invert`).
fn base(invert: bool) -> (Vec<String>, Path) {
    let w = PATH_WIDTH;
    let r1 = S_CURVE_TIGHT_RADIUS;

    // The track leaves the top vertex heading down and to the left (as the loop of
    // old loop did) and arrives at the bottom-right vertex heading straight down
    // (as the long arc of `frag_arc_long` did). It turns left first, then right.
    let start = Point::new(0.5, S3);
    let start_heading = 210f64.to_radians();
    let end = Point::new(1.0, 0.0);
    let end_heading = -90f64.to_radians();
    let first_turn = 1.0;
    let second_turn = -first_turn;

    // Centre of the tight curve, then the radius of the wide curve that makes the two
    // tangent where the track bends the other way: |c1 - c2| = r1 + r2.
    let c1 = start + left_of(start_heading) * (first_turn * r1);
    let from_end = c1 - end;
    let n_end = left_of(end_heading);
    let r2 =
        (dot(from_end, from_end) - r1 * r1) / (2.0 * (second_turn * dot(from_end, n_end) + r1));
    let c2 = end + n_end * (second_turn * r2);
    let towards = (c2 - c1) * (1.0 / length(c2 - c1));
    let bend = c1 + towards * r1;

    // Each wall is the middle line moved `w` to the left or right.
    let points = vec![
        start + left_of(start_heading) * w,
        bend - towards * (first_turn * w),
        end + n_end * w,
        end - n_end * w,
        bend + towards * (first_turn * w),
        start - left_of(start_heading) * w,
    ];
    let large = |angle: f64| {
        if angle > std::f64::consts::PI {
            "1"
        } else {
            "0"
        }
    };
    let radii = vec![
        trimf64(TILE_WIDTH * (r1 - first_turn * w)),
        trimf64(TILE_WIDTH * (r2 - second_turn * w)),
        trimf64(TILE_WIDTH * (r1 + first_turn * w)),
        trimf64(TILE_WIDTH * (r2 + second_turn * w)),
        large(swept(c1, start, bend, first_turn)).to_string(),
        large(swept(c2, bend, end, second_turn)).to_string(),
    ];

    let pts = Path::new(points);
    let pts = if invert { pts.invert() } else { pts };
    (radii, pts)
}

/// Draws the track. With `invert`, the curves turn the other way.
fn svg(invert: bool, radii: Vec<String>, pts: Path) -> String {
    // In the base frame the tight curve turns left (sweep flag 1) and the wide one right.
    let (s1, s2) = if invert { (0, 1) } else { (1, 0) };
    let p = pts.str();
    let (t_left, w_left, t_right, w_right, large1, large2) = (
        &radii[0], &radii[1], &radii[2], &radii[3], &radii[4], &radii[5],
    );
    format!(
        "<path fill='{FILL}' d='M{a}A{t_left} {t_left} 0 {large1} {s1} {b}A{w_left} {w_left} 0 {large2} {s2} {c}\
         L{d}A{w_right} {w_right} 0 {large2} {s1} {e}A{t_right} {t_right} 0 {large1} {s2} {f}z'/>\
         <path stroke='black' d='M{a}A{t_left} {t_left} 0 {large1} {s1} {b}A{w_left} {w_left} 0 {large2} {s2} {c}\
         M{f}A{t_right} {t_right} 0 {large1} {s1} {e}A{w_right} {w_right} 0 {large2} {s2} {d}'/>",
        a = p[0],
        b = p[1],
        c = p[2],
        d = p[3],
        e = p[4],
        f = p[5],
    )
}

define_shape_family! {
    base: base,
    svg: svg,
    offset_rotations: &[0, 1, 2, 3, 4, 5],
    variants: {
        n45_14_35: false, false => identity,
        n46_14_26: false, false => rotate_ccw,
        n56_35_26: false, false => rotate_cw,
        n45_24_15: false, true => reflect,
        n46_24_36: false, true => reflect_then_rotate_ccw,
        n56_15_36: false, true => reflect_then_rotate_cw,
    }
}

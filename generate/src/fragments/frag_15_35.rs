//! A track from one vertex of a tile to the next one round, crossing into the
//! neighbouring tile through the middle of the edge between them (`15/35` and its
//! rotations).
//!
//! Both ends point the same way, so the track is an S of two equal curves of half a
//! tile width, meeting at the middle of the edge. That radius is fixed by the geometry:
//! any other split would cross the edge away from its midpoint.

use crate::fragments::frag_s_curve::{s_curve_base, s_curve_svg};
use common::define_shape_family;
use common::macros::{identity, rotate_ccw, rotate_cw};
use common::types::path::Path;
use common::types::point::{Point, S3};

/// The track in the frame of `n15_35` on a normal tile (inverted when `invert`): from
/// the top vertex to the bottom-right vertex, heading straight down at both ends.
fn base(invert: bool) -> (Vec<String>, Path) {
    let down = -90f64.to_radians();
    s_curve_base(
        invert,
        Point::new(0.5, S3),
        down,
        Point::new(1.0, 0.0),
        down,
        0.5,
    )
}

define_shape_family! {
    base: base,
    svg: s_curve_svg,
    offset_rotations: &[0, 1, 2, 3, 4, 5],
    variants: {
        n15_35: false, false => identity,
        n24_14: false, false => rotate_ccw,
        n36_26: false, false => rotate_cw,
    }
}

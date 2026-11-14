//! A track that runs from one vertex to another round the far side of the tile,
//! through both of the other edge openings (keys such as `56` with `15` and
//! `26` across the vertices). It is drawn as one curve centred on the edge
//! midpoint between the two vertices.

use common::consts::FILL;
use common::types::format::trimf64;
use common::types::point::{Point, PATH_WIDTH, S3, TILE_WIDTH};

/// Radius of the crescent's two end curves (half a tile width makes it one
/// semicircle). Smaller radii join the curves with a straight; the ends keep
/// their positions and directions either way. Keep it in `(PATH_WIDTH, 0.5]`.
pub const CRESCENT_RADIUS: f64 = 0.5;

fn fmt(p: Point, invert: bool) -> String {
    let p = if invert { p.invert() } else { p };
    (p * TILE_WIDTH).trim()
}

/// `mid` is the centre edge midpoint and `u` the unit vector from it to the end vertex.
/// The track starts at the opposite vertex and turns anticlockwise round `mid`.
fn crescent(invert: bool, mid: Point, u: Point) -> String {
    let r = CRESCENT_RADIUS;
    let b = Point { x: u.y, y: -u.x }; // towards the far vertex
    let c1 = mid + u * (r - 0.5);
    let c2 = mid + u * (0.5 - r);
    let wall = |rad: f64| [c1 - u * rad, c1 + b * rad, c2 + b * rad, c2 + u * rad];
    let w = PATH_WIDTH;
    let (outer, inner) = (wall(r + w), wall(r - w));
    let (ro, ri) = (trimf64(TILE_WIDTH * (r + w)), trimf64(TILE_WIDTH * (r - w)));
    let (fwd, back) = if invert { (0, 1) } else { (1, 0) };
    let straight = r < 0.5 - 1e-9;
    let side = |p: [Point; 4], rad: &str, sweep: u8, reverse: bool| {
        let p = if reverse { [p[3], p[2], p[1], p[0]] } else { p };
        let mid = if straight {
            format!("L{}", fmt(p[2], invert))
        } else {
            String::new()
        };
        format!(
            "A{rad} {rad} 0 0 {sweep} {}{mid}A{rad} {rad} 0 0 {sweep} {}",
            fmt(p[1], invert),
            fmt(p[3], invert)
        )
    };
    format!(
        "<path fill='{FILL}' d='M{}{}L{}{}Z'/>\
         <path stroke='black' d='M{}{}M{}{}'/>",
        fmt(outer[0], invert),
        side(outer, &ro, fwd, false),
        fmt(inner[3], invert),
        side(inner, &ri, back, true),
        fmt(outer[0], invert),
        side(outer, &ro, fwd, false),
        fmt(inner[0], invert),
        side(inner, &ri, fwd, false),
    )
}

/// Vertices 1 and 2 round edges 5 and 6 (centred on edge 4).
pub fn n46_15_26(invert: bool) -> String {
    crescent(
        invert,
        Point {
            x: 0.25,
            y: S3 / 2.0,
        },
        Point { x: 0.5, y: S3 },
    )
}

/// Vertices 2 and 3 round edges 4 and 5 (centred on edge 6).
pub fn n45_24_35(invert: bool) -> String {
    crescent(invert, Point { x: 0.5, y: 0.0 }, Point { x: -1.0, y: 0.0 })
}

/// Vertices 1 and 3 round edges 4 and 6 (centred on edge 5).
pub fn n46_14_36(invert: bool) -> String {
    crescent(
        invert,
        Point {
            x: 0.75,
            y: S3 / 2.0,
        },
        Point { x: 0.5, y: -S3 },
    )
}

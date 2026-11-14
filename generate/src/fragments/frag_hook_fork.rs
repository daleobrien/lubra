//! A junction whose track comes in through a vertex and hooks round to the edge
//! opening beside it, where the tile across that edge carries on round the same
//! vertex. Drawn tile by tile, the hook is a 120° turn on a 20-unit centreline,
//! the tightest curve there is.
//!
//! Here the hook and the neighbour's arc are one half-circle from the vertex to
//! the arc's far opening, and what is left of the junction is the plain track from
//! the vertex to its third node. The two fork at the vertex; the point where they
//! separate is rounded with `JOIN_RADIUS`.
//!
//! The tile with the arc draws the fragment (key `arc / junction`), in its own
//! coordinates; the junction's tile lies across the shared edge.

use common::consts::FILL;
use common::types::format::trimf64;
use common::types::point::{Point, PATH_WIDTH, S3, TILE_WIDTH};

/// Radius of the curve that rounds the point where the two tracks separate
/// (9.2/120 of a tile, the same as a junction's corner). 0 leaves a sharp point.
pub const JOIN_RADIUS: f64 = 9.2 / TILE_WIDTH;

/// The half-circle's radius: its ends are a vertex and the middle of an edge from
/// that vertex, so it is a quarter of a tile.
const HALF_CIRCLE_RADIUS: f64 = 0.25;

/// The plain vertex-to-vertex track is an arc of one tile width.
const VERTEX_ARC_RADIUS: f64 = 1.0;

pub(crate) fn dot(a: Point, b: Point) -> f64 {
    a.x * b.x + a.y * b.y
}

pub(crate) fn cross(a: Point, b: Point) -> f64 {
    a.x * b.y - a.y * b.x
}

pub(crate) fn length(a: Point) -> f64 {
    dot(a, a).sqrt()
}

pub(crate) fn unit(a: Point) -> Point {
    a * (1.0 / length(a))
}

/// `a` turned a quarter turn anticlockwise.
pub(crate) fn turned(a: Point) -> Point {
    Point { x: -a.y, y: a.x }
}

/// Where node `node` (1-3) of a normal tile is.
pub(crate) fn vertex(node: usize) -> Point {
    match node {
        1 => Point { x: 0.5, y: S3 },
        2 => Point { x: 0.0, y: 0.0 },
        _ => Point { x: 1.0, y: 0.0 },
    }
}

/// The two vertices at the ends of edge `node` (4-6).
pub(crate) fn edge_ends(node: usize) -> (usize, usize) {
    match node {
        4 => (1, 2),
        5 => (1, 3),
        _ => (2, 3),
    }
}

/// The plain track from the vertex to the junction's third node.
#[derive(Clone, Copy)]
enum Plain {
    /// Straight to `to`.
    Line { to: Point },
    /// An arc of `VERTEX_ARC_RADIUS` about `centre` to `to`.
    Arc { centre: Point, to: Point },
}

/// The point `p` with both `first(p)` and `second(p)` equal to `value`, found by
/// Newton's method from `guess`. The generator stops if it does not settle.
fn solve(
    first: impl Fn(Point) -> f64,
    second: impl Fn(Point) -> f64,
    value: f64,
    guess: Point,
) -> Point {
    let step = 1e-7;
    let mut p = guess;
    for _ in 0..50 {
        let (f, g) = (first(p) - value, second(p) - value);
        let dx = Point { x: step, y: 0.0 };
        let dy = Point { x: 0.0, y: step };
        let (fx, fy) = (
            (first(p + dx) - first(p)) / step,
            (first(p + dy) - first(p)) / step,
        );
        let (gx, gy) = (
            (second(p + dx) - second(p)) / step,
            (second(p + dy) - second(p)) / step,
        );
        let det = fx * gy - fy * gx;
        p = p - Point {
            x: (f * gy - g * fy) / det,
            y: (g * fx - f * gx) / det,
        };
    }
    let miss = (first(p) - value).abs() + (second(p) - value).abs();
    assert!(miss < 1e-9, "the join of a hook fork did not settle");
    p
}

/// The fragment for the arc tile's net (the two edges beside vertex `v`), with the
/// junction `{v, e, x}` in the tile across edge `e`.
pub fn hook_fork(invert: bool, v: usize, e: usize, x: usize) -> String {
    let w = PATH_WIDTH;
    let (first, second) = edge_ends(e);
    let other = if first == v { second } else { first };
    let third = 6 - v - other;

    // The arc tile's own corners, and the junction tile's third corner across edge `e`.
    let at_v = vertex(v);
    let at_other = vertex(other);
    let far = (at_v + vertex(third)) * 0.5;
    let across = at_v + at_other - vertex(third);

    // Both tracks leave the vertex along the junction tile's bisector.
    let heading = unit((at_other + across) * 0.5 - at_v);
    let left = turned(heading);
    let centre = (at_v + far) * 0.5;
    // +1 when the half-circle turns anticlockwise (its centre is on the left).
    let side = dot(centre - at_v, left).signum();
    let towards = left * side;
    let radius = HALF_CIRCLE_RADIUS;

    let plain = if x == other {
        Plain::Arc {
            centre: vertex(third),
            to: at_other,
        }
    } else if x == third {
        Plain::Arc {
            centre: at_v + across - at_other,
            to: across,
        }
    } else {
        Plain::Line {
            to: (at_other + across) * 0.5,
        }
    };

    // The plain track's wall on the half-circle's side (`near`) and the other (`far_wall`):
    // each as (point at the vertex, point at the far end, radius or 0 for a line).
    let wall = |towards_half_circle: bool| -> (Point, Point, f64) {
        let sign = if towards_half_circle { 1.0 } else { -1.0 };
        match plain {
            Plain::Line { to } => (at_v + towards * (sign * w), to + towards * (sign * w), 0.0),
            Plain::Arc { centre: c, to } => {
                let centre_side = dot(c - at_v, towards).signum();
                let r = VERTEX_ARC_RADIUS - sign * centre_side * w;
                (c + unit(at_v - c) * r, c + unit(to - c) * r, r)
            }
        }
    };
    let (_, near_end, near_radius) = wall(true);
    let (far_start, far_end, far_radius) = wall(false);

    // Distance into the blank wedge between the two tracks, from each of its walls.
    let from_near = |p: Point| match plain {
        Plain::Line { .. } => dot(p - at_v, towards) - w,
        Plain::Arc { centre: c, .. } => {
            let centre_side = dot(c - at_v, towards).signum();
            centre_side * (VERTEX_ARC_RADIUS - length(p - c)) - w
        }
    };
    let from_half_circle = |p: Point| length(p - centre) - (radius + w);
    let guess = at_v + heading * 0.4 + towards * 0.15;
    let join_centre = solve(from_near, from_half_circle, JOIN_RADIUS, guess);
    let on_near = match plain {
        Plain::Line { .. } => join_centre - towards * from_near(join_centre),
        Plain::Arc { centre: c, .. } => c + unit(join_centre - c) * near_radius,
    };
    let on_half_circle = centre + unit(join_centre - centre) * (radius + w);

    // Formatting: the inverted tile is the mirror image, so its arcs turn the other way.
    let point = |p: Point| {
        let p = if invert { p.invert() } else { p };
        (p * TILE_WIDTH).trim()
    };
    let arc = |r: f64, anticlockwise: bool, to: Point| {
        let sweep = u8::from(anticlockwise != invert);
        let r = trimf64(TILE_WIDTH * r);
        format!("A{r} {r} 0 0 {sweep} {}", point(to))
    };
    let along = |r: f64, anticlockwise: bool, to: Point| {
        if r == 0.0 {
            format!("L{}", point(to))
        } else {
            arc(r, anticlockwise, to)
        }
    };

    let half_turn = side > 0.0;
    // The plain arc turns towards its centre as it leaves the vertex.
    let plain_turn = match plain {
        Plain::Line { .. } => false,
        Plain::Arc { centre: c, .. } => dot(c - at_v, left) > 0.0,
    };
    let join = if JOIN_RADIUS > 0.0 {
        let turn = cross(on_near - join_centre, on_half_circle - join_centre) > 0.0;
        arc(JOIN_RADIUS, turn, on_half_circle)
    } else {
        String::new()
    };

    let inner = radius - w;
    let inner_start = centre + unit(at_v - centre) * inner;
    let inner_middle = centre + turned(unit(at_v - centre)) * (inner * side);
    let inner_end = centre + unit(far - centre) * inner;
    let outer_end = centre + unit(far - centre) * (radius + w);

    // Three walls: the plain track's far wall; its near wall, the join and the
    // half-circle's outer wall; and the half-circle's inner wall.
    let far_wall = along(far_radius, plain_turn, far_end);
    let middle_wall = format!(
        "{}{join}{}",
        along(near_radius, !plain_turn, on_near),
        arc(radius + w, half_turn, outer_end)
    );
    let inner_wall = format!(
        "{}{}",
        arc(inner, half_turn, inner_middle),
        arc(inner, half_turn, inner_end)
    );
    let inner_back = format!(
        "{}{}",
        arc(inner, !half_turn, inner_middle),
        arc(inner, !half_turn, inner_start)
    );
    format!(
        "<path fill='{FILL}' d='M{fs}{far_wall}L{ne}{middle_wall}L{ie}{inner_back}Z'/>\
         <path stroke='black' d='M{fs}{far_wall}M{ne}{middle_wall}M{is}{inner_wall}'/>",
        fs = point(far_start),
        ne = point(near_end),
        ie = point(inner_end),
        is = point(inner_start),
    )
}

/// Every hook fork: (arc net, junction net, vertex, shared edge, third node), with
/// nets as node bits. The junction is the vertex, the edge and one more node that
/// is not the vertex's other edge (that junction has a hook on both sides, and two
/// arc tiles could claim it).
pub fn hook_forks() -> Vec<(usize, usize, usize, usize, usize)> {
    let bit = |node: usize| 1 << (node - 1);
    let mut out = Vec::new();
    for v in 1..=3 {
        let edges: Vec<usize> = (4..=6)
            .filter(|&e| edge_ends(e).0 == v || edge_ends(e).1 == v)
            .collect();
        let opposite = 15 - edges[0] - edges[1];
        for &e in &edges {
            let (first, second) = edge_ends(e);
            let other = if first == v { second } else { first };
            let third = 6 - v - other;
            for x in [other, third, opposite] {
                let arc = bit(edges[0]) | bit(edges[1]);
                out.push((arc, bit(v) | bit(e) | bit(x), v, e, x));
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_hook_forks_are_eighteen_distinct_keys() {
        let forks = hook_forks();
        let keys: HashSet<(usize, usize)> = forks.iter().map(|f| (f.0, f.1)).collect();
        assert_eq!(forks.len(), 18);
        assert_eq!(keys.len(), 18);
        for (arc, junction, v, e, _) in forks {
            // The arc is two edges, the junction three nodes sharing only edge `e`.
            assert_eq!(arc.count_ones(), 2);
            assert_eq!(arc & 0b111, 0, "the arc joins edges only");
            assert_eq!(junction.count_ones(), 3);
            assert_eq!(arc & junction, 1 << (e - 1));
            assert!(junction & (1 << (v - 1)) != 0);
        }
    }

    #[test]
    fn test_every_hook_fork_has_three_walls() {
        for invert in [false, true] {
            for (_, _, v, e, x) in hook_forks() {
                let fragment = hook_fork(invert, v, e, x);
                let walls = fragment.split("stroke='black' d='").nth(1).expect("walls");
                assert_eq!(walls.matches('M').count(), 3, "{fragment}");
            }
        }
    }

    #[test]
    fn test_half_circle_starts_at_the_vertex_and_ends_at_the_far_opening() {
        // Arc 46 with junction 245 across edge 4: the half-circle runs from vertex 2
        // at the origin to the middle of edge 6, so its inner wall is 19.2 from (30, 0).
        let fragment = hook_fork(false, 2, 4, 5);
        assert!(
            fragment.contains("M10.8 0A19.2 19.2 0 0 0 30 19.2"),
            "{fragment}"
        );
        assert!(
            fragment.ends_with("A19.2 19.2 0 0 0 49.2 0'/>"),
            "{fragment}"
        );
    }
}

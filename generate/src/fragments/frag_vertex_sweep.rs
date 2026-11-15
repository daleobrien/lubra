//! A wide arc from one vertex to the next that carries on through the second vertex
//! into a hook round to the edge beside it, where both turn the same way. Drawn tile
//! by tile, the hook swings out before it turns, which leaves a wiggle after the arc.
//!
//! Here the two are one sweep that only ever turns one way: an arc of 150 that ends a
//! quarter of a tile past the vertex, then an arc of 30 into the edge opening. The
//! ends are where they were, so nothing else changes; the radii follow from that and
//! there is nothing to set.
//!
//! The hook's tile draws the fragment (key `hook / arc`, e.g. `14/12`), in its own
//! coordinates; the arc's tile is the one opposite the vertex. The sweep passes beside
//! the vertex, not through it, so the renderer only keeps it where the measured
//! clearance table says it is clear (see `renders::vertex_s`).

use super::frag_hook_fork::{cross, length, turned, unit, vertex};
use common::consts::FILL;
use common::types::format::trimf64;
use common::types::point::{Point, PATH_WIDTH, S3, TILE_WIDTH};

/// The wide arc turns 60° over a chord of one tile and a quarter, so that is its radius.
const WIDE_RADIUS: f64 = 1.25;

/// The second arc turns 120° into the opening, a quarter of a tile from the vertex line.
const TIGHT_RADIUS: f64 = 0.25;

/// The fragment for the hook from vertex `v` to the edge between `v` and vertex `p`,
/// with the arc from `v` to `p` in the tile opposite `v`.
pub fn vertex_sweep(invert: bool, v: usize, p: usize) -> String {
    let w = PATH_WIDTH;
    let q = 6 - v - p;
    let (at_v, at_p, at_q) = (vertex(v), vertex(p), vertex(q));

    // The opposite tile is this one turned half round about the vertex, which swaps
    // the other two vertices: its `p` is opposite this tile's `q`.
    let start = at_v * 2.0 - at_q;
    let end = (at_v + at_p) * 0.5;
    let join = at_v + (at_q - at_v) * 0.25;

    let along = unit(join - start);
    // +1 when the sweep turns anticlockwise.
    let side = cross(along, at_p - at_v).signum();
    let wide_centre = (start + join) * 0.5 + turned(along) * (side * WIDE_RADIUS * S3);
    // The heading where the arcs meet: the chord's direction turned 30° on.
    let heading = along * S3 + turned(along) * (side * 0.5);
    let tight_centre = join + turned(heading) * (side * TIGHT_RADIUS);
    assert!(
        (length(end - tight_centre) - TIGHT_RADIUS).abs() < 1e-9,
        "the sweep does not end at the edge opening"
    );

    let point = |p: Point| {
        let p = if invert { p.invert() } else { p };
        (p * TILE_WIDTH).trim()
    };
    let arc = |r: f64, anticlockwise: bool, to: Point| {
        let sweep = u8::from(anticlockwise != invert);
        let r = trimf64(TILE_WIDTH * r);
        format!("A{r} {r} 0 0 {sweep} {}", point(to))
    };
    // One wall, `offset` outside the centreline: its three points.
    let wall = |offset: f64| {
        [
            wide_centre + unit(start - wide_centre) * (WIDE_RADIUS + offset),
            wide_centre + unit(join - wide_centre) * (WIDE_RADIUS + offset),
            tight_centre + unit(end - tight_centre) * (TIGHT_RADIUS + offset),
        ]
    };
    let turn = side > 0.0;
    let (outer, inner) = (wall(w), wall(-w));
    let forwards = |points: [Point; 3], offset: f64| {
        format!(
            "{}{}",
            arc(WIDE_RADIUS + offset, turn, points[1]),
            arc(TIGHT_RADIUS + offset, turn, points[2])
        )
    };
    let inner_back = format!(
        "{}{}",
        arc(TIGHT_RADIUS - w, !turn, inner[1]),
        arc(WIDE_RADIUS - w, !turn, inner[0])
    );
    format!(
        "<path fill='{FILL}' d='M{os}{outer_wall}L{ie}{inner_back}Z'/>\
         <path stroke='black' d='M{os}{outer_wall}M{is}{inner_wall}'/>",
        os = point(outer[0]),
        outer_wall = forwards(outer, w),
        ie = point(inner[2]),
        is = point(inner[0]),
        inner_wall = forwards(inner, -w),
    )
}

/// Every vertex sweep: (hook net, arc net, vertex, far vertex), nets as node bits.
pub fn vertex_sweeps() -> Vec<(usize, usize, usize, usize)> {
    let bit = |node: usize| 1 << (node - 1);
    let mut out = Vec::new();
    for v in 1..=3 {
        for p in (1..=3).filter(|&p| p != v) {
            // The edge between vertices v and p: 4 for (1, 2), 5 for (1, 3), 6 for (2, 3).
            let edge = v + p + 1;
            out.push((bit(v) | bit(edge), bit(v) | bit(p), v, p));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertex_sweeps_pair_each_hook_with_the_arc_that_turns_its_way() {
        let sweeps = vertex_sweeps();
        assert_eq!(sweeps.len(), 6);
        // Vertex 1 with vertex 2: the hook is 14 and the arc is 12.
        assert!(sweeps.contains(&(0b001001, 0b000011, 1, 2)));
        for (hook, arc, v, _) in sweeps {
            assert_eq!(hook & arc, 1 << (v - 1), "they share only the vertex");
        }
    }

    #[test]
    fn test_sweep_ends_at_the_far_vertex_and_the_edge_opening() {
        // Hook 26 with arc 23, on a normal tile: vertex 2 is the origin and the edge
        // opening is the middle of the bottom edge. The wide arc starts at the far
        // vertex of the tile opposite, a tile width up and to the left.
        let fragment = vertex_sweep(false, 2, 3);
        let walls = fragment.split("stroke='black' d='").nth(1).expect("walls");
        assert_eq!(walls.matches('M').count(), 2, "{fragment}");
        assert_eq!(walls.matches('A').count(), 4, "{fragment}");
        // the edge opening's walls, half a track either side of its middle
        let half = PATH_WIDTH * TILE_WIDTH;
        let near = format!("{} 0", trimf64(60.0 - half));
        let far = format!("{} 0", trimf64(60.0 + half));
        assert!(walls.contains(&near) && walls.contains(&far), "{fragment}");
    }
}

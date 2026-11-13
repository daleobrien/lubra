//! The entrance and exit markers: a dead end that opens into a ring with a dot in it.

use common::consts::{FILL, IN, N4, N5, N6};
use common::types::point::{PATH_WIDTH, TILE_HEIGHT, TILE_WIDTH};

/// Outer radius of the ring, in SVG units. It sits on the tile's centre, which is
/// about 34.6 units from every edge, so the ring stays inside its tile.
const RING_RADIUS: f64 = 24.0;
/// Radius of the curve that blends each track wall into the ring.
const BLEND_RADIUS: f64 = 8.0;
/// Radius of the dot in the middle of the ring.
const DOT_RADIUS: f64 = 7.0;
/// Colour of the dot.
const DOT_COLOUR: &str = "#E02020";

/// The marker fragment for a tile whose only track is a dead end at an edge.
///
/// `net` is the transformed tile net, as used to look up `MAP_PATHS`: one of the
/// edge nodes `N4`, `N5` or `N6`, with `IN` set for an inverted tile. Returns `None`
/// for anything else, which keeps its ordinary fragment.
///
/// The track runs from the middle of the edge towards the tile's centre. Each wall
/// leaves the track on a small blend curve and joins a ring around the centre, so the
/// outline never has a corner.
pub fn terminal_fragment(net: usize) -> Option<String> {
    let inverted = net & IN != 0;
    // Tile-local positions: the middle of the node's edge and the tile's centre.
    let edge_y = if inverted { TILE_HEIGHT } else { 0.0 };
    let start = match net & !IN {
        N4 => (TILE_WIDTH / 4.0, TILE_HEIGHT / 2.0),
        N5 => (TILE_WIDTH * 3.0 / 4.0, TILE_HEIGHT / 2.0),
        N6 => (TILE_WIDTH / 2.0, edge_y),
        _ => return None,
    };
    let centre_y = if inverted {
        TILE_HEIGHT * 2.0 / 3.0
    } else {
        TILE_HEIGHT / 3.0
    };
    let centre = (TILE_WIDTH / 2.0, centre_y);

    // Work in a frame with `start` at the origin and the track along +x.
    let length = (centre.0 - start.0).hypot(centre.1 - start.1);
    let along = ((centre.0 - start.0) / length, (centre.1 - start.1) / length);
    let point = |x: f64, y: f64| {
        format!(
            "{:.1} {:.1}",
            start.0 + along.0 * x - along.1 * y,
            start.1 + along.1 * x + along.0 * y
        )
    };

    let half_width = PATH_WIDTH * TILE_WIDTH;
    // The blend curve's centre is `BLEND_RADIUS` outside the wall and outside the ring.
    let reach = RING_RADIUS + BLEND_RADIUS;
    let rise = half_width + BLEND_RADIUS;
    let run = (reach * reach - rise * rise).sqrt();
    let blend_x = length - run;
    // Where the blend curve meets the ring.
    let ring_x = length - run * RING_RADIUS / reach;
    let ring_y = rise * RING_RADIUS / reach;

    Some(format!(
        "<path stroke='black' fill='{FILL}' d='M{a}L{b}A{f} {f} 0 0 1 {c}A{r} {r} 0 1 0 {d}A{f} {f} 0 0 1 {e}L{g}'/>\
         <circle cx='{cx:.1}' cy='{cy:.1}' r='{DOT_RADIUS}' fill='{DOT_COLOUR}' fill-opacity='1'/>",
        a = point(0.0, half_width),
        b = point(blend_x, half_width),
        c = point(ring_x, ring_y),
        d = point(ring_x, -ring_y),
        e = point(blend_x, -half_width),
        g = point(0.0, -half_width),
        f = BLEND_RADIUS,
        r = RING_RADIUS,
        cx = centre.0,
        cy = centre.1,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::consts::{N1, N2};

    #[test]
    fn test_terminal_fragment_only_for_edge_dead_ends() {
        for net in [N4, N5, N6, N4 | IN, N5 | IN, N6 | IN] {
            assert!(terminal_fragment(net).is_some(), "net {net}");
        }
        for net in [0, N1, N2, N4 | N5, N5 | N6 | IN] {
            assert!(terminal_fragment(net).is_none(), "net {net}");
        }
    }

    #[test]
    fn test_terminal_fragment_starts_at_the_track_walls() {
        // N6 of a normal tile is the middle of the bottom edge; the walls are
        // PATH_WIDTH either side of it, matching the ordinary dead end.
        let fragment = terminal_fragment(N6).unwrap();
        assert!(fragment.contains("d='M49.2 0.0L"), "{fragment}");
        assert!(fragment.contains("L70.8 0.0'"), "{fragment}");
    }
}

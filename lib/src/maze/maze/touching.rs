//! Touching tracks: finds tracks drawn against each other and separates them.

use std::collections::{HashMap, HashSet};

use super::dead_ends::{Link, TileNet};
use super::fill_clearance::{DRAWN_CLEAR, DRAWN_CROSSING};
use super::net_reduction::VERTEX_FLANKS;
use crate::maze::maze::Maze;
use crate::maze::net::Node;
use crate::renders::vertex_s::{vertex_s_index, VERTEX_S_CLEAR};
use common::consts::{NL, NODE_TO_COORD};

/// The position index of a tile itself in the clearance tables; 0-11 are its neighbours.
const SAME_TILE: usize = 12;

/// How many rounds to try. Each round removes every touch it can; later rounds
/// deal with touches made by reshaping a track or joining a piece back on.
const MAX_ROUNDS: usize = 16;

/// How two drawn tile nets meet.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum Meeting {
    /// White space between them.
    Clear,
    /// One passes cleanly over the other.
    Crossing,
    /// Their walls graze, or one runs along the other.
    Touching,
}

/// How tile net `net` meets tile net `other` at `position` (0-11 for the neighbours
/// in `NODE_TO_COORD` order, 12 for the same tile), from the measured tables.
pub(super) fn meeting(position: usize, net: usize, other: usize) -> Meeting {
    let bit = 1 << (other & 63);
    if DRAWN_CLEAR[position][net & 63] & bit != 0 {
        Meeting::Clear
    } else if DRAWN_CROSSING[position][net & 63] & bit != 0 {
        Meeting::Crossing
    } else {
        Meeting::Touching
    }
}

/// Whether tile nets `net` and `other` of one tile are drawn with white space
/// between them (not touching and not crossing).
pub(crate) fn clear_in_same_tile(net: usize, other: usize) -> bool {
    meeting(SAME_TILE, net, other) == Meeting::Clear
}

impl Maze {
    /// Whether the vertex S (`frag_vertex_s`) with logical key `key` on tile `(i, j)`
    /// would be drawn against a net it does not join, in its tile or the twelve around
    /// it (from the measured `VERTEX_S_CLEAR`). Returns false for other keys.
    ///
    /// The nets it joins are its own, the other half across the vertex, and the two
    /// nets at its openings: across its own edge, and across the far half's edge.
    pub(crate) fn vertex_s_blocked(&self, i: usize, j: usize, key: usize) -> bool {
        let Some(index) = vertex_s_index(key) else {
            return false;
        };
        let own = key & NL;
        let opposite_net = (key >> 6) & NL;
        let shared = own & opposite_net;
        let vertex = shared.trailing_zeros() as usize;
        let edge = (own & !shared).trailing_zeros() as usize;
        let far_edge = (opposite_net & !shared).trailing_zeros() as usize;
        let opposite = self.neighbour(i, j, vertex);
        let far = opposite.and_then(|(oi, oj)| self.tile_across(oi, oj, far_edge));

        (0..=SAME_TILE).any(|position| {
            let at = if position == SAME_TILE {
                Some((i, j))
            } else {
                self.neighbour(i, j, position)
            };
            let Some(tile) = at else {
                return false;
            };
            self.tiles[(tile.1, tile.0)].nodes.iter().any(|&net| {
                let joined = (position == SAME_TILE && net == own)
                    || (Some(tile) == opposite && net == opposite_net)
                    || (position == edge && net & (1 << edge) != 0)
                    || (Some(tile) == far && net & (1 << far_edge) != 0);
                !joined && VERTEX_S_CLEAR[index][position] & (1 << (net & 63)) == 0
            })
        })
    }

    /// The tiles round the vertex that the vertex S with logical key `key` on tile
    /// `(i, j)` turns round: its own tile, then the tile opposite the vertex, then the
    /// (up to) four that flank it. Its half-circles swing into them. Empty for other keys.
    pub(crate) fn vertex_s_footprint(&self, i: usize, j: usize, key: usize) -> Vec<(usize, usize)> {
        if vertex_s_index(key).is_none() {
            return Vec::new();
        }
        let vertex = ((key & NL) & ((key >> 6) & NL)).trailing_zeros() as usize;
        let mut tiles = vec![(i, j)];
        tiles.extend(self.neighbour(i, j, vertex));
        tiles.extend(
            VERTEX_FLANKS[vertex]
                .iter()
                .filter_map(|&p| self.neighbour(i, j, p)),
        );
        tiles
    }

    /// The tile at `position` (0-11) around tile `(i, j)`, if it is inside the grid.
    fn neighbour(&self, i: usize, j: usize, position: usize) -> Option<(usize, usize)> {
        let (di, dj) = NODE_TO_COORD[(i + 3 * (j % 2)) % 6][position];
        let ni = usize::try_from(i as i32 + di).ok()?;
        let nj = usize::try_from(j as i32 + dj).ok()?;
        (ni < self.number_of_columns && nj < self.number_of_rows).then_some((ni, nj))
    }

    /// How tile net `net`, drawn on tile `(i, j)`, meets each tile net around it that
    /// it is not joined to: those on the same tile and on the twelve tiles around.
    ///
    /// A tile net of the same tile that shares a node with `net` is `net` itself, or
    /// the one it would replace, so is left out.
    pub(super) fn meetings(
        &self,
        i: usize,
        j: usize,
        net: usize,
    ) -> Vec<(TileNet, usize, Meeting)> {
        let mut found = Vec::new();
        for &other in &self.tiles[(j, i)].nodes {
            if other & net == 0 {
                let at = TileNet { i, j, bits: other };
                found.push((at, SAME_TILE, meeting(SAME_TILE, net, other)));
            }
        }
        for position in 0..SAME_TILE {
            let Some((ni, nj)) = self.neighbour(i, j, position) else {
                continue;
            };
            for &other in &self.tiles[(nj, ni)].nodes {
                // Positions 0-5 are the tiles across nodes 0-5: sharing that node
                // means the two are one track.
                let joined = position < 6 && net & other & (1 << position) != 0;
                if !joined {
                    let at = TileNet {
                        i: ni,
                        j: nj,
                        bits: other,
                    };
                    found.push((at, position, meeting(position, net, other)));
                }
            }
        }
        found
    }

    /// Every pair of tile nets drawn touching each other, each pair once, in grid order.
    pub(super) fn touching_pairs(&self) -> Vec<(TileNet, TileNet)> {
        let mut pairs = Vec::new();
        let mut seen = HashSet::new();
        for j in 0..self.number_of_rows {
            for i in 0..self.number_of_columns {
                for &bits in &self.tiles[(j, i)].nodes {
                    let here = TileNet { i, j, bits };
                    for (other, _, how) in self.meetings(i, j, bits) {
                        if how == Meeting::Touching
                            && !seen.contains(&(other, here))
                            && seen.insert((here, other))
                        {
                            pairs.push((here, other));
                        }
                    }
                }
            }
        }
        pairs
    }

    /// The number of pairs of tile nets drawn touching each other.
    pub fn touching_count(&self) -> usize {
        self.touching_pairs().len()
    }

    /// Reworks the maze until no two tracks are drawn touching, where it can.
    ///
    /// Each round takes one tile net out of every touching pair, then joins the
    /// pieces that were beyond it back on somewhere else, using only joins that do
    /// not touch anything (`rejoin_nets_tidily`). Tracks that cross are left alone.
    ///
    /// The route from the entrance to the exit is never broken: a pair with both
    /// tile nets on the route is left touching. Pieces that could not be joined
    /// back on are removed at the end, so the maze is one loop-free tree again.
    ///
    /// Returns the number of touching pairs left. `self.nets` is rebuilt afterwards.
    /// Run `fill_blank_areas` after this to grow new branches into freed space.
    pub fn separate_touching_tracks(&mut self) -> usize {
        let mut rng = self.seeded_rng();
        for _ in 0..MAX_ROUNDS {
            if !self.remove_touching_round() {
                break;
            }
            self.rejoin_nets_tidily(&mut rng);
        }
        self.remove_detached_nets();
        self.touching_count()
    }

    /// Takes one tile net out of each touching pair. Returns whether any were removed.
    fn remove_touching_round(&mut self) -> bool {
        let pairs = self.touching_pairs();
        if pairs.is_empty() {
            return false;
        }
        let links = self.tile_net_links();
        let route = self.route_tile_nets(&links);
        let link_count = |net: &TileNet| links.get(net).map_or(0, Vec::len);

        // Tile nets removed or reshaped this round; a pair involving one waits for
        // the next round, when it may no longer touch.
        let mut changed: HashSet<TileNet> = HashSet::new();
        let mut removed_any = false;
        for (a, b) in pairs {
            if changed.contains(&a) || changed.contains(&b) {
                continue;
            }
            // Prefer the one with the fewest tracks hanging off it.
            let choice = [a, b]
                .into_iter()
                .filter(|net| !route.contains(net))
                .min_by_key(link_count);
            let Some(net) = choice else {
                continue;
            };
            let joined = links.get(&net).cloned().unwrap_or_default();
            if joined.iter().any(|link| changed.contains(&link.to)) {
                continue;
            }

            self.edit_tile_nets(net.i, net.j, |tile_net| {
                (tile_net != net.bits).then_some(tile_net)
            });
            changed.insert(net);
            // Each tile net it was joined to gives up the shared node.
            for link in joined {
                let to = link.to;
                self.edit_tile_nets(to.i, to.j, |tile_net| {
                    let kept = if tile_net == to.bits {
                        tile_net & !(1 << link.node)
                    } else {
                        tile_net
                    };
                    (kept != 0).then_some(kept)
                });
                changed.insert(to);
            }
            removed_any = true;
        }
        removed_any
    }

    /// The tile nets on the route from the entrance to the exit.
    fn route_tile_nets(&self, links: &HashMap<TileNet, Vec<Link>>) -> HashSet<TileNet> {
        let Some(&bits) = self.tiles[(0, 0)].nodes.first() else {
            return HashSet::new();
        };
        let root = TileNet { i: 0, j: 0, bits };
        let mut parent: HashMap<TileNet, TileNet> = HashMap::new();
        let mut order = vec![root];
        let mut next = 0;
        while next < order.len() {
            let here = order[next];
            next += 1;
            for link in links.get(&here).into_iter().flatten() {
                if link.to != root && !parent.contains_key(&link.to) {
                    parent.insert(link.to, here);
                    order.push(link.to);
                }
            }
        }

        let exit = (self.number_of_columns - 1, self.number_of_rows - 1);
        let mut route = HashSet::from([root]);
        let mut at = order.iter().find(|net| (net.i, net.j) == exit).copied();
        while let Some(net) = at {
            route.insert(net);
            at = parent.get(&net).copied();
        }
        route
    }

    /// Removes every net that is not joined to the entrance, then rebuilds `self.nets`.
    fn remove_detached_nets(&mut self) {
        self.rebuild_nets();
        let entrance = |node: &Node| (node.i, node.j) == (0, 0);
        let detached: Vec<Node> = self
            .nets
            .iter()
            .filter(|net| !net.iter().any(entrance))
            .flat_map(|net| net.iter().copied())
            .collect();
        for node in detached {
            let mask = 1 << node.node;
            self.edit_tile_nets(node.i, node.j, |tile_net| {
                (tile_net & mask == 0).then_some(tile_net)
            });
        }
        self.rebuild_nets();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::consts::{N4, N5, N6};

    #[test]
    fn test_meeting_same_tile_edge_tracks() {
        // Two dead ends on different edges of one tile are drawn apart.
        assert_eq!(meeting(SAME_TILE, N4, N6), Meeting::Clear);
        // A tile net is never compared with one that shares its nodes, so such a
        // pair is in neither table.
        assert_eq!(meeting(SAME_TILE, N4 | N5, N5 | N6), Meeting::Touching);
    }

    #[test]
    fn test_separate_touching_tracks_random_mazes() {
        for seed in [1, 2, 3, 7, 42] {
            let mut maze = Maze::new(30, 20, true, false, false, Some(seed));
            maze.reduce_nets(1);
            maze.remove_cycles(false);
            maze.trim_dead_ends(2);
            let before = maze.touching_count();
            let entrance = maze.tiles[(0, 0)].nodes.clone();
            let exit = maze.tiles[(19, 29)].nodes.clone();

            let left = maze.separate_touching_tracks();

            assert_eq!(left, maze.touching_count(), "seed {seed}");
            // Only touches along the entrance-to-exit route can be left.
            assert!(left * 4 < before, "seed {seed}: {before} -> {left}");
            assert_eq!(maze.nets.len(), 1, "seed {seed}");
            assert_eq!(maze.loop_count(), 0, "seed {seed}");
            assert_eq!(maze.tiles[(0, 0)].nodes, entrance, "seed {seed}");
            assert_eq!(maze.tiles[(19, 29)].nodes, exit, "seed {seed}");

            // Filling afterwards adds track without adding a touch.
            maze.fill_blank_areas(3);
            assert_eq!(maze.touching_count(), left, "seed {seed}");
            assert_eq!(maze.nets.len(), 1, "seed {seed}");
            assert_eq!(maze.loop_count(), 0, "seed {seed}");
        }
    }

    #[test]
    fn test_separate_touching_tracks_is_repeatable() {
        let run = |seed| {
            let mut maze = Maze::new(20, 12, true, false, false, Some(seed));
            maze.reduce_nets(1);
            maze.remove_cycles(false);
            maze.separate_touching_tracks();
            maze.tiles
                .elements_row_major_iter()
                .map(|tile| tile.nodes.clone())
                .collect::<Vec<_>>()
        };
        assert_eq!(run(5), run(5));
    }
}

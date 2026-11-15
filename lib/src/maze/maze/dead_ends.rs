//! Dead-end trimming: removes dead ends too short to be worth drawing.

use std::collections::HashMap;

use super::touching::crossing_in_same_tile;
use crate::maze::maze::Maze;
use crate::maze::net::Node;
use crate::maze::tile::Tile;
use common::nodes::get_connected_coords_at_coords;
use common::types::coordinate::Coordinate;

/// A tile net, identified by its tile and its node bits.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub(super) struct TileNet {
    pub(super) i: usize,
    pub(super) j: usize,
    pub(super) bits: usize,
}

/// A link from one tile net to the tile net across one of its nodes.
#[derive(Clone, Copy)]
pub(super) struct Link {
    pub(super) node: usize,
    pub(super) to: TileNet,
}

impl Maze {
    /// Removes every dead end that is at most `max_length` tile nets long.
    ///
    /// A dead end is a chain of tile nets that starts at a leaf (a tile net joined to
    /// only one other) and runs through plain two-way tile nets until it reaches a
    /// junction (a tile net joined to three or more). Cutting a loop next to a
    /// junction leaves a dead end only a tile or two long, which is drawn as a stub;
    /// this removes those and takes the branch off the junction.
    ///
    /// All dead ends are measured before any are removed, so a junction that loses
    /// its short branches is not then trimmed away itself. Dead ends that include the
    /// entrance tile `(0, 0)` or the exit tile (the last tile) are kept, as are chains
    /// that never reach a junction. `self.nets` is rebuilt afterwards.
    pub fn trim_dead_ends(&mut self, max_length: usize) {
        if max_length == 0 {
            return;
        }
        let links = self.tile_net_links();
        let exit = (
            self.number_of_columns.saturating_sub(1),
            self.number_of_rows.saturating_sub(1),
        );
        let is_protected = |net: &TileNet| (net.i, net.j) == (0, 0) || (net.i, net.j) == exit;

        // Tile nets to delete, and the node each junction gives up.
        let mut doomed: Vec<TileNet> = Vec::new();
        let mut closed_branches: Vec<(TileNet, usize)> = Vec::new();

        let mut leaves: Vec<&TileNet> = links.keys().filter(|net| links[net].len() == 1).collect();
        leaves.sort_by_key(|net| (net.j, net.i, net.bits));

        for &leaf in leaves {
            let mut chain = vec![leaf];
            let mut previous = leaf;
            let mut link = links[&leaf][0];
            // Walk through two-way tile nets until the chain ends or meets a junction.
            let junction = loop {
                let here = link.to;
                match links[&here].len() {
                    2 => {
                        let next = links[&here]
                            .iter()
                            .find(|l| l.to != previous)
                            .copied()
                            .unwrap_or(link);
                        chain.push(here);
                        previous = here;
                        link = next;
                        if chain.len() > max_length {
                            break None;
                        }
                    }
                    n if n >= 3 => break Some(here),
                    _ => break None, // another leaf: a path with no junction
                }
            };
            let Some(junction) = junction else {
                continue;
            };
            if chain.len() > max_length || chain.iter().any(is_protected) {
                continue;
            }
            // In a maze without crossings, keep a branch whose junction would become
            // a plain track crossing another in its tile.
            if !self.crossings {
                let reduced = junction.bits & !(1 << link.node);
                let crosses = self.tiles[(junction.j, junction.i)]
                    .nodes
                    .iter()
                    .any(|&other| {
                        other & junction.bits == 0 && crossing_in_same_tile(reduced, other)
                    });
                if crosses {
                    continue;
                }
            }
            doomed.extend(chain);
            closed_branches.push((junction, link.node));
        }

        for net in &doomed {
            self.edit_tile_nets(net.i, net.j, |tile_net| {
                (tile_net != net.bits).then_some(tile_net)
            });
        }
        for &(junction, node) in &closed_branches {
            let mask = 1 << node;
            self.edit_tile_nets(junction.i, junction.j, |tile_net| {
                let kept = if tile_net & mask != 0 {
                    tile_net & !mask
                } else {
                    tile_net
                };
                (kept != 0).then_some(kept)
            });
        }
        self.rebuild_nets();
    }

    /// Rewrites the tile nets of tile `(i, j)`: `edit` maps each to its replacement, or
    /// `None` to remove it.
    pub(super) fn edit_tile_nets(
        &mut self,
        i: usize,
        j: usize,
        edit: impl Fn(usize) -> Option<usize>,
    ) {
        let tile_nets: Vec<usize> = self.tiles[(j, i)]
            .nodes
            .iter()
            .filter_map(|&tile_net| edit(tile_net))
            .collect();
        self.tiles[(j, i)] = Tile::new(tile_nets, true);
    }

    /// For every tile net, the tile nets it joins across its nodes.
    pub(super) fn tile_net_links(&self) -> HashMap<TileNet, Vec<Link>> {
        let mut owner: HashMap<Node, TileNet> = HashMap::new();
        for j in 0..self.number_of_rows {
            for i in 0..self.number_of_columns {
                for &bits in &self.tiles[(j, i)].nodes {
                    for k in (0..6).filter(|k| bits & (1 << k) != 0) {
                        owner.insert(Node::new(i, j, k), TileNet { i, j, bits });
                    }
                }
            }
        }

        let mut links: HashMap<TileNet, Vec<Link>> = HashMap::new();
        for (&node, &net) in &owner {
            let entry = links.entry(net).or_default();
            let here = Coordinate {
                i: node.i,
                j: node.j,
            };
            let across = get_connected_coords_at_coords(here, node.node)
                .and_then(|c| owner.get(&Node::new(c.i, c.j, node.node)));
            if let Some(&to) = across {
                entry.push(Link {
                    node: node.node,
                    to,
                });
            }
        }
        // Keep each list in node order so the walk is the same on every run.
        for list in links.values_mut() {
            list.sort_by_key(|link| link.node);
        }
        links
    }
}

#[cfg(test)]
mod tests {
    use crate::maze::maze::Maze;
    use crate::maze::tile::Tile;
    use common::consts::{N4, N5, N6};

    /// A 6 x 2 maze: a corridor along row 0 with a junction at (1,0) and a one-tile
    /// stub above it at (1,1).
    ///
    /// Row 0 neighbours share nodes 5, 6, 4, 5, 6 in turn; (1,0) and (1,1) share node 4.
    fn corridor_with_stub() -> Maze {
        let mut maze = Maze::new(6, 2, false, false, false, None);
        let row = [
            vec![N5],
            vec![N5 | N6 | N4],
            vec![N6 | N4],
            vec![N4 | N5],
            vec![N5 | N6],
            vec![N6],
        ];
        for (i, tile_nets) in row.into_iter().enumerate() {
            maze.tiles[(0, i)] = Tile::new(tile_nets, true);
        }
        maze.tiles[(1, 1)] = Tile::new(vec![N4], true);
        maze.rebuild_nets();
        maze
    }

    #[test]
    fn test_trim_dead_ends_removes_a_short_stub() {
        let mut maze = corridor_with_stub();
        assert_eq!(maze.nets.len(), 1);

        maze.trim_dead_ends(2);

        // The stub is gone and the junction is a plain track again.
        assert!(maze.tiles[(1, 1)].nodes.is_empty());
        assert_eq!(maze.tiles[(0, 1)].nodes, vec![N5 | N6]);
        // The four-tile dead end on the right is longer than the limit, so it stays.
        assert_eq!(maze.tiles[(0, 5)].nodes, vec![N6]);
        assert_eq!(maze.tiles[(0, 2)].nodes, vec![N6 | N4]);
        assert_eq!(maze.nets.len(), 1);
    }

    #[test]
    fn test_trim_dead_ends_keeps_the_entrance() {
        let mut maze = corridor_with_stub();
        maze.trim_dead_ends(2);
        // (0,0) is a one-tile dead end off the junction, but it is the entrance.
        assert_eq!(maze.tiles[(0, 0)].nodes, vec![N5]);
    }

    #[test]
    fn test_trim_dead_ends_zero_does_nothing() {
        let mut maze = corridor_with_stub();
        maze.trim_dead_ends(0);
        assert_eq!(maze.tiles[(1, 1)].nodes, vec![N4]);
        assert_eq!(maze.tiles[(0, 1)].nodes, vec![N5 | N6 | N4]);
    }

    #[test]
    fn test_trim_dead_ends_longer_limit_removes_the_long_dead_end() {
        let mut maze = corridor_with_stub();
        maze.trim_dead_ends(4);
        for i in 2..6 {
            assert!(maze.tiles[(0, i)].nodes.is_empty(), "tile ({i},0)");
        }
        // Both branches were measured before either was removed.
        assert_eq!(maze.tiles[(0, 1)].nodes, vec![N5]);
    }

    #[test]
    fn test_trim_dead_ends_random_mazes_stay_one_tree() {
        for seed in [1, 2, 3, 7, 42] {
            let mut maze = Maze::new(30, 20, true, false, false, Some(seed));
            maze.reduce_nets(1);
            maze.remove_cycles(false);
            let entrance = maze.tiles[(0, 0)].nodes.clone();
            let exit = maze.tiles[(19, 29)].nodes.clone();

            maze.trim_dead_ends(2);

            assert_eq!(maze.nets.len(), 1, "seed {seed}");
            assert_eq!(maze.loop_count(), 0, "seed {seed}");
            assert_eq!(maze.tiles[(0, 0)].nodes, entrance, "seed {seed}");
            assert_eq!(maze.tiles[(19, 29)].nodes, exit, "seed {seed}");
        }
    }
}

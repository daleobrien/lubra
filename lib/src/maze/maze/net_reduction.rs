//! Net reduction: joins the maze's separate nets into one.

use std::collections::HashMap;

use rand::seq::SliceRandom;
use rand_chacha::ChaCha8Rng;

use super::net_operations::DisjointSets;
use super::touching::Meeting;
use crate::maze::maze::Maze;
use crate::maze::net::Node;
use crate::maze::tile::Tile;
use common::nodes::get_connected_coords_at_coords;
use common::types::coordinate::Coordinate;

/// For each vertex (node index 0-2), the surrounding tiles that also touch it, as
/// indices into `get_connected_titles_for_tile`: the two edge neighbours that share
/// the vertex and the two tiles beside the opposite tile. A path through a vertex
/// only continues into the opposite tile, so none of these may use the vertex.
const VERTEX_FLANKS: [[usize; 4]; 3] = [[3, 4, 6, 7], [3, 5, 8, 9], [4, 5, 10, 11]];

/// A tile net, identified by its tile and one of its nodes.
#[derive(Clone, Copy)]
struct TileNetRef {
    i: usize,
    j: usize,
    node: usize,
}

impl TileNetRef {
    fn position(&self) -> Node {
        Node::new(self.i, self.j, self.node)
    }
}

/// A candidate route through empty tile `(i, j)`: in at node `k1` from tile net `a`,
/// out at node `k2` to tile net `b`.
#[derive(Clone, Copy)]
struct EmptyTileRoute {
    i: usize,
    j: usize,
    k1: usize,
    k2: usize,
    a: TileNetRef,
    b: TileNetRef,
}

/// The lowest node index of a tile net, used to refer to it.
fn first_node(tile_net: usize) -> usize {
    tile_net.trailing_zeros() as usize
}

impl Maze {
    /// Joins the maze's nets until only one is left, or no more joins are possible.
    ///
    /// 1. Rebuild the net list from the tiles, then drop every net with at most
    ///    `lower_limit` node positions (`remove_short_nets`; 0 keeps them all).
    /// 2. Repeat passes until one net is left or a pass makes no join. Each pass uses
    ///    the first of these that joins anything:
    ///    - merge two tile nets of different nets that share a tile;
    ///    - open a free node between neighbouring tiles of different nets;
    ///    - route a new two-node net through an empty tile between two nets.
    ///
    /// Within a pass each pair of nets is joined at most once, so joins never create a
    /// loop. Candidates are shuffled with the maze's seed, so a seeded maze always
    /// reduces the same way. `self.nets` is rebuilt after every pass, so it always
    /// matches the tiles.
    pub fn reduce_nets(&mut self, lower_limit: usize) {
        self.rebuild_nets();
        if self.nets.len() <= 1 {
            return;
        }
        self.remove_short_nets(lower_limit);
        self.rebuild_nets();

        let mut rng = self.seeded_rng();
        while self.nets.len() > 1 {
            let joined = self.join_at_shared_tiles(&mut rng, false)
                || self.join_across_free_nodes(&mut rng, false)
                || self.join_through_empty_tiles(&mut rng, false);
            if !joined {
                break;
            }
            self.rebuild_nets();
        }
    }

    /// Joins separate nets like `reduce_nets`, but only with joins whose reshaped
    /// tile nets are not drawn touching another track. Nets that cannot be joined
    /// that way stay separate.
    pub(super) fn rejoin_nets_tidily(&mut self, rng: &mut ChaCha8Rng) {
        self.rebuild_nets();
        while self.nets.len() > 1 {
            let joined = self.join_at_shared_tiles(rng, true)
                || self.join_across_free_nodes(rng, true)
                || self.join_through_empty_tiles(rng, true);
            if !joined {
                break;
            }
            self.rebuild_nets();
        }
    }

    /// The node bits of the tile net that `tile_net` refers to.
    fn bits_of(&self, tile_net: TileNetRef) -> usize {
        self.tiles[(tile_net.j, tile_net.i)]
            .nodes
            .iter()
            .copied()
            .find(|bits| bits & (1 << tile_net.node) != 0)
            .unwrap_or(0)
    }

    /// Whether tile net `bits`, drawn on tile `(i, j)`, would touch a track other
    /// than `partners`: `(position, tile net)` pairs it is about to be joined to.
    fn would_touch(&self, i: usize, j: usize, bits: usize, partners: &[(usize, usize)]) -> bool {
        self.meetings(i, j, bits)
            .into_iter()
            .any(|(other, position, how)| {
                how == Meeting::Touching && !partners.contains(&(position, other.bits))
            })
    }

    /// Maps every node position to the index of its net in `self.nets`.
    fn net_owners(&self) -> HashMap<Node, usize> {
        self.nets
            .iter()
            .enumerate()
            .flat_map(|(index, net)| net.iter().map(move |&node| (node, index)))
            .collect()
    }

    /// The tile nets of tile `(i, j)`, with the net each belongs to.
    fn tile_nets_with_owner(
        &self,
        owners: &HashMap<Node, usize>,
        i: usize,
        j: usize,
    ) -> Vec<(TileNetRef, usize)> {
        self.tiles[(j, i)]
            .nodes
            .iter()
            .filter(|&&tile_net| tile_net != 0)
            .map(|&tile_net| {
                let tile_net_ref = TileNetRef {
                    i,
                    j,
                    node: first_node(tile_net),
                };
                (tile_net_ref, owners[&tile_net_ref.position()])
            })
            .collect()
    }

    /// Whether tile `(i, j)` is the entrance (the first tile) or the exit (the last).
    /// Both hold a single dead end that no pass may extend or route past.
    pub(crate) fn is_terminal(&self, i: usize, j: usize) -> bool {
        (i, j) == (0, 0) || (i + 1, j + 1) == (self.number_of_columns, self.number_of_rows)
    }

    /// The tile across node `k` of tile `(i, j)`, if it is inside the grid.
    pub(super) fn tile_across(&self, i: usize, j: usize, k: usize) -> Option<(usize, usize)> {
        let across = get_connected_coords_at_coords(Coordinate { i, j }, k)?;
        self.tiles.get(across.j, across.i)?;
        Some((across.i, across.j))
    }

    /// Whether node `k` of tile `(i, j)` can carry a new path: it must be unused in
    /// this tile and in the tile across it, and for a vertex, in the tiles flanking it.
    pub(super) fn node_is_free(&self, i: usize, j: usize, k: usize) -> bool {
        let mask = 1 << k;
        let surrounding = self.get_connected_titles_for_tile(i, j);
        let in_use = |tile: Option<&Tile>| tile.is_some_and(|t| t.contains(mask));
        if in_use(Some(&self.tiles[(j, i)])) || in_use(surrounding[k]) {
            return false;
        }
        k >= 3
            || VERTEX_FLANKS[k]
                .iter()
                .all(|&flank| !in_use(surrounding[flank]))
    }

    /// Replaces the tile nets containing `node_a` and `node_b` with their union.
    fn merge_tile_nets(&mut self, i: usize, j: usize, node_a: usize, node_b: usize) {
        let mask = (1 << node_a) | (1 << node_b);
        let (merged, mut kept): (Vec<usize>, Vec<usize>) = self.tiles[(j, i)]
            .nodes
            .iter()
            .partition(|&&tile_net| tile_net & mask != 0);
        kept.push(merged.iter().fold(0, |all, tile_net| all | tile_net));
        self.tiles[(j, i)] = Tile::new(kept, true);
    }

    /// Adds node `k` to the tile net that contains `tile_net.node`.
    fn extend_tile_net(&mut self, tile_net: TileNetRef, k: usize) {
        let member = 1 << tile_net.node;
        let nodes = self.tiles[(tile_net.j, tile_net.i)]
            .nodes
            .iter()
            .map(|&t| if t & member != 0 { t | (1 << k) } else { t })
            .collect();
        self.tiles[(tile_net.j, tile_net.i)] = Tile::new(nodes, true);
    }

    /// Merges tile nets of different nets that share a tile. Returns whether any were merged.
    fn join_at_shared_tiles(&mut self, rng: &mut ChaCha8Rng, tidy: bool) -> bool {
        let owners = self.net_owners();
        let mut candidates: Vec<(TileNetRef, TileNetRef)> = Vec::new();
        for j in 0..self.number_of_rows {
            for i in 0..self.number_of_columns {
                let tile_nets = self.tile_nets_with_owner(&owners, i, j);
                for (a, &(ref_a, net_a)) in tile_nets.iter().enumerate() {
                    for &(ref_b, net_b) in &tile_nets[a + 1..] {
                        if net_a != net_b {
                            candidates.push((ref_a, ref_b));
                        }
                    }
                }
            }
        }
        candidates.shuffle(rng);

        let mut joined_nets = DisjointSets::new(self.nets.len());
        let mut joined = false;
        for (a, b) in candidates {
            let merged = self.bits_of(a) | self.bits_of(b);
            if tidy && self.would_touch(a.i, a.j, merged, &[]) {
                continue;
            }
            if joined_nets.union(owners[&a.position()], owners[&b.position()]) {
                self.merge_tile_nets(a.i, a.j, a.node, b.node);
                joined = true;
            }
        }
        joined
    }

    /// Opens a free node between neighbouring tiles of different nets, extending a tile
    /// net on each side. Returns whether any nets were joined.
    fn join_across_free_nodes(&mut self, rng: &mut ChaCha8Rng, tidy: bool) -> bool {
        let owners = self.net_owners();
        let mut candidates: Vec<(TileNetRef, TileNetRef, usize)> = Vec::new();
        for j in 0..self.number_of_rows {
            for i in 0..self.number_of_columns {
                for k in 0..6 {
                    let Some((bi, bj)) = self.tile_across(i, j, k) else {
                        continue;
                    };
                    // Visit each pair of tiles once, and leave the entrance and exit alone.
                    if (bj, bi) < (j, i) || self.is_terminal(i, j) || self.is_terminal(bi, bj) {
                        continue;
                    }
                    for (ref_a, net_a) in self.tile_nets_with_owner(&owners, i, j) {
                        for (ref_b, net_b) in self.tile_nets_with_owner(&owners, bi, bj) {
                            if net_a != net_b {
                                candidates.push((ref_a, ref_b, k));
                            }
                        }
                    }
                }
            }
        }
        candidates.shuffle(rng);

        let mut joined_nets = DisjointSets::new(self.nets.len());
        let mut joined = false;
        for (a, b, k) in candidates {
            if !self.node_is_free(a.i, a.j, k) {
                continue;
            }
            let (bits_a, bits_b) = (self.bits_of(a), self.bits_of(b));
            if tidy
                && (self.would_touch(a.i, a.j, bits_a | (1 << k), &[(k, bits_b)])
                    || self.would_touch(b.i, b.j, bits_b | (1 << k), &[(k, bits_a)]))
            {
                continue;
            }
            if joined_nets.union(owners[&a.position()], owners[&b.position()]) {
                self.extend_tile_net(a, k);
                self.extend_tile_net(b, k);
                joined = true;
            }
        }
        joined
    }

    /// Fills an empty tile with a two-node net that connects tile nets of different
    /// nets on two of its sides. Returns whether any nets were joined.
    fn join_through_empty_tiles(&mut self, rng: &mut ChaCha8Rng, tidy: bool) -> bool {
        let owners = self.net_owners();
        let mut candidates: Vec<EmptyTileRoute> = Vec::new();
        for j in 0..self.number_of_rows {
            for i in 0..self.number_of_columns {
                if !self.tiles[(j, i)].nodes.is_empty() || self.is_terminal(i, j) {
                    continue;
                }
                for k1 in 0..6 {
                    for k2 in k1 + 1..6 {
                        let (Some((ai, aj)), Some((bi, bj))) =
                            (self.tile_across(i, j, k1), self.tile_across(i, j, k2))
                        else {
                            continue;
                        };
                        if self.is_terminal(ai, aj) || self.is_terminal(bi, bj) {
                            continue;
                        }
                        for (ref_a, net_a) in self.tile_nets_with_owner(&owners, ai, aj) {
                            for (ref_b, net_b) in self.tile_nets_with_owner(&owners, bi, bj) {
                                if net_a != net_b {
                                    candidates.push(EmptyTileRoute {
                                        i,
                                        j,
                                        k1,
                                        k2,
                                        a: ref_a,
                                        b: ref_b,
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
        candidates.shuffle(rng);

        let mut joined_nets = DisjointSets::new(self.nets.len());
        let mut joined = false;
        for EmptyTileRoute { i, j, k1, k2, a, b } in candidates {
            let still_empty = self.tiles[(j, i)].nodes.is_empty();
            if !still_empty || !self.node_is_free(i, j, k1) || !self.node_is_free(i, j, k2) {
                continue;
            }
            let (bits_a, bits_b) = (self.bits_of(a), self.bits_of(b));
            let route = (1 << k1) | (1 << k2);
            if tidy
                && (self.would_touch(i, j, route, &[(k1, bits_a), (k2, bits_b)])
                    || self.would_touch(a.i, a.j, bits_a | (1 << k1), &[])
                    || self.would_touch(b.i, b.j, bits_b | (1 << k2), &[]))
            {
                continue;
            }
            if joined_nets.union(owners[&a.position()], owners[&b.position()]) {
                self.tiles[(j, i)] = Tile::new(vec![(1 << k1) | (1 << k2)], true);
                self.extend_tile_net(a, k1);
                self.extend_tile_net(b, k2);
                joined = true;
            }
        }
        joined
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::maze::net::Net;
    use array2d::Array2D;
    use common::consts::{N1, N5, N6};

    /// Empty tiles before the test row. The entrance and exit tiles are never joined
    /// onto, so the row sits between padding; three keeps the row's node pattern
    /// (neighbours share nodes 5, 6, 4 in turn) the same as starting at column 0.
    const PAD: usize = 3;

    /// A maze with the given tile nets (row 0 only, after `PAD` empty tiles and before
    /// one more), with nets built from the tiles.
    fn maze_from_row(row: &[Option<Vec<usize>>]) -> Maze {
        let mut tiles = Array2D::filled_with(Tile::new(vec![], false), 1, row.len() + PAD + 1);
        for (i, tile_nets) in row.iter().enumerate() {
            if let Some(tile_nets) = tile_nets {
                tiles[(0, i + PAD)] = Tile::new(tile_nets.clone(), true);
            }
        }
        let mut maze = Maze {
            style: None,
            nets: Net::new(),
            width: 0.0,
            height: 0.0,
            seed: Some(1),
            number_of_columns: row.len() + PAD + 1,
            number_of_rows: 1,
            tiles,
            boundaries: false,
            show_node_numbers: false,
            show_tile_nets: false,
        };
        maze.rebuild_nets();
        maze
    }

    fn tile_nets(maze: &Maze, i: usize) -> Vec<usize> {
        let mut nets = maze.tiles[(0, i + PAD)].nodes.clone();
        nets.sort();
        nets
    }

    /// No tile uses a node twice, and no vertex path touches a flanking tile.
    fn assert_valid(maze: &Maze) {
        for j in 0..maze.number_of_rows {
            for i in 0..maze.number_of_columns {
                let tile = &maze.tiles[(j, i)];
                let used = tile.nodes.iter().fold(0, |all, t| all | t);
                let total: u32 = tile.nodes.iter().map(|t| t.count_ones()).sum();
                assert_eq!(used.count_ones(), total, "tile ({i},{j}) reuses a node");

                let surrounding = maze.get_connected_titles_for_tile(i, j);
                for (k, flanks) in VERTEX_FLANKS.iter().enumerate() {
                    if used & (1 << k) == 0 {
                        continue;
                    }
                    for &flank in flanks {
                        let clash = surrounding[flank].is_some_and(|t| t.contains(1 << k));
                        assert!(!clash, "vertex {} of ({i},{j}) touches a flank", k + 1);
                    }
                }
            }
        }
    }

    #[test]
    fn test_reduce_nets_empty_maze() {
        let mut maze = maze_from_row(&[None]);
        maze.reduce_nets(0);
        assert!(maze.nets.is_empty());
    }

    #[test]
    fn test_reduce_nets_single_net_unchanged() {
        // (0,0) and (1,0) share node 5, so this is one net.
        let mut maze = maze_from_row(&[Some(vec![N1 | N5]), Some(vec![N5 | N6])]);
        assert_eq!(maze.nets.len(), 1);
        maze.reduce_nets(0);
        assert_eq!(maze.nets.len(), 1);
        assert_eq!(tile_nets(&maze, 0), vec![N1 | N5]);
        assert_eq!(tile_nets(&maze, 1), vec![N5 | N6]);
    }

    #[test]
    fn test_reduce_nets_merges_nets_sharing_a_tile() {
        // Two nets cross tile (1,0): 5 -> (0,0) and 6 -> (2,0).
        let mut maze = maze_from_row(&[Some(vec![N5]), Some(vec![N5, N6]), Some(vec![N6])]);
        assert_eq!(maze.nets.len(), 2);
        maze.reduce_nets(0);
        assert_eq!(maze.nets.len(), 1);
        assert_eq!(tile_nets(&maze, 1), vec![N5 | N6]);
        assert_valid(&maze);
    }

    #[test]
    fn test_reduce_nets_opens_a_free_node_between_neighbours() {
        // (0,0) only uses vertex 1; (1,0)-(2,0) are joined on node 6. Node 5 between
        // (0,0) and (1,0) is free, so it should be opened.
        let mut maze = maze_from_row(&[Some(vec![N1]), Some(vec![N6]), Some(vec![N6])]);
        assert_eq!(maze.nets.len(), 2);
        maze.reduce_nets(0);
        assert_eq!(maze.nets.len(), 1);
        assert_eq!(tile_nets(&maze, 0), vec![N1 | N5]);
        assert_eq!(tile_nets(&maze, 1), vec![N5 | N6]);
        assert_valid(&maze);
    }

    #[test]
    fn test_reduce_nets_routes_through_an_empty_tile() {
        // (1,0) is empty; its node 5 faces (0,0) and node 6 faces (2,0).
        let mut maze = maze_from_row(&[Some(vec![N1]), Some(vec![]), Some(vec![N1])]);
        assert_eq!(maze.nets.len(), 2);
        maze.reduce_nets(0);
        assert_eq!(maze.nets.len(), 1);
        assert_eq!(tile_nets(&maze, 0), vec![N1 | N5]);
        assert_eq!(tile_nets(&maze, 1), vec![N5 | N6]);
        assert_eq!(tile_nets(&maze, 2), vec![N1 | N6]);
        assert_valid(&maze);
    }

    #[test]
    fn test_reduce_nets_removes_short_nets() {
        // Two single-position nets, both at or below the limit of 1, are removed.
        let mut maze = maze_from_row(&[Some(vec![N1]), Some(vec![]), Some(vec![N1])]);
        maze.reduce_nets(1);
        assert!(maze.nets.is_empty());
        assert!(tile_nets(&maze, 0).is_empty());
        assert!(tile_nets(&maze, 2).is_empty());
    }

    #[test]
    fn test_reduce_nets_random_mazes_end_with_one_valid_net() {
        for seed in [1, 2, 3, 7, 42, 99] {
            let mut maze = Maze::new(16, 10, true, false, false, Some(seed));
            maze.reduce_nets(1);
            assert_eq!(maze.nets.len(), 1, "seed {seed}");
            assert_valid(&maze);
        }
    }

    #[test]
    fn test_reduce_nets_is_reproducible_for_a_seed() {
        let reduce = || {
            let mut maze = Maze::new(16, 10, true, false, false, Some(11));
            maze.reduce_nets(1);
            maze.tiles.as_row_major()
        };
        assert!(reduce() == reduce());
    }
}

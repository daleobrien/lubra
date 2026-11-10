use crate::maze::maze::Maze;
use crate::maze::net::{Node, Nodes};
use crate::maze::tile::Tile;
use common::nodes::get_connected_coords_at_coords;
use common::types::coordinate::Coordinate;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::collections::HashMap;

impl Maze {
    pub fn remove_short_nets(&mut self, lower_limit: usize) {
        if lower_limit == 0 {
            return;
        }

        // Strip out small nets
        self.nets.retain(|nets| {
            // Remove small nets
            if nets.len() > lower_limit {
                return true;
            }
            // Of the form [(i,j,4), (...), ... ]
            for tile_node in nets.iter() {
                //
                if let Some(tile) = self.tiles.get(tile_node.j, tile_node.i) {
                    let reduced_tile = tile
                        .nodes
                        .iter()
                        .filter(|&b| {
                            let mask = 1usize << tile_node.node;
                            mask & *b != mask
                        })
                        .copied()
                        .collect::<Vec<usize>>();

                    self.tiles[(tile_node.j, tile_node.i)] = Tile::new(reduced_tile, true);
                }
            }
            false
        });
    }
}

/// Union-find over `0..len`, used to group node positions into nets and to
/// track which nets have been joined.
pub(super) struct DisjointSets {
    parent: Vec<usize>,
}

impl DisjointSets {
    pub(super) fn new(len: usize) -> Self {
        Self {
            parent: (0..len).collect(),
        }
    }

    /// The representative of the set containing `x` (halving paths as it goes).
    pub(super) fn find(&mut self, mut x: usize) -> usize {
        while self.parent[x] != x {
            self.parent[x] = self.parent[self.parent[x]];
            x = self.parent[x];
        }
        x
    }

    /// Joins the sets containing `a` and `b`. Returns `false` if they were already joined.
    pub(super) fn union(&mut self, a: usize, b: usize) -> bool {
        let (root_a, root_b) = (self.find(a), self.find(b));
        if root_a == root_b {
            return false;
        }
        self.parent[root_b] = root_a;
        true
    }
}

impl Maze {
    /// A random number generator seeded from the maze's current seed, or from the
    /// thread RNG for unseeded mazes.
    pub(super) fn seeded_rng(&self) -> ChaCha8Rng {
        match self.seed {
            Some(s) => ChaCha8Rng::seed_from_u64(s),
            None => ChaCha8Rng::from_rng(thread_rng()).unwrap(),
        }
    }

    /// Recomputes `self.nets` from the tiles.
    ///
    /// Two node positions are in the same net when they belong to the same tile net,
    /// or when they are the same physical node seen from the two tiles either side of
    /// it: logical node `k` of a tile and logical node `k` of the tile across node `k`.
    /// (A path through a vertex continues into the opposite tile only.)
    ///
    /// The result is deterministic: nets are ordered by the first tile (row by row)
    /// that uses them.
    pub(crate) fn rebuild_nets(&mut self) {
        // Number every node position used by a tile net, in grid order.
        let mut positions: Vec<Node> = Vec::new();
        let mut position_index: HashMap<Node, usize> = HashMap::new();
        for j in 0..self.number_of_rows {
            for i in 0..self.number_of_columns {
                for &tile_net in &self.tiles[(j, i)].nodes {
                    for k in (0..6).filter(|k| tile_net & (1 << k) != 0) {
                        let node = Node::new(i, j, k);
                        position_index.entry(node).or_insert_with(|| {
                            positions.push(node);
                            positions.len() - 1
                        });
                    }
                }
            }
        }

        let mut sets = DisjointSets::new(positions.len());

        // Nodes of one tile net are connected inside the tile.
        for j in 0..self.number_of_rows {
            for i in 0..self.number_of_columns {
                for &tile_net in &self.tiles[(j, i)].nodes {
                    let members: Vec<usize> = (0..6)
                        .filter(|k| tile_net & (1 << k) != 0)
                        .map(|k| position_index[&Node::new(i, j, k)])
                        .collect();
                    for pair in members.windows(2) {
                        sets.union(pair[0], pair[1]);
                    }
                }
            }
        }

        // A node is connected to the same logical node of the tile across it.
        for (index, node) in positions.iter().enumerate() {
            let here = Coordinate {
                i: node.i,
                j: node.j,
            };
            if let Some(across) = get_connected_coords_at_coords(here, node.node) {
                if let Some(&other) = position_index.get(&Node::new(across.i, across.j, node.node))
                {
                    sets.union(index, other);
                }
            }
        }

        // Group by root, keeping the order in which each net is first seen.
        let mut net_of_root: HashMap<usize, usize> = HashMap::new();
        let mut nets: Vec<Nodes> = Vec::new();
        for (index, &node) in positions.iter().enumerate() {
            let root = sets.find(index);
            let net = *net_of_root.entry(root).or_insert_with(|| {
                nets.push(Nodes::new());
                nets.len() - 1
            });
            nets[net].insert(node);
        }
        self.nets = nets;
    }
}

#[cfg(test)]
mod tests {
    use crate::maze::maze::Maze;
    use crate::maze::net::Nodes;
    use crate::maze::tile::Tile;
    use common::consts::{N1, N5};
    use std::collections::BTreeSet;

    /// The nets as a set of sorted node lists, so order doesn't matter.
    fn canonical(nets: &[Nodes]) -> BTreeSet<Vec<(usize, usize, usize)>> {
        nets.iter()
            .map(|net| {
                let mut nodes: Vec<_> = net.iter().map(|n| (n.i, n.j, n.node)).collect();
                nodes.sort();
                nodes
            })
            .collect()
    }

    #[test]
    fn test_rebuild_nets_covers_every_node_once() {
        for seed in [1, 2, 3, 7, 42] {
            let maze = Maze::new(12, 8, true, false, false, Some(seed));
            let mut used = 0;
            for j in 0..maze.number_of_rows {
                for i in 0..maze.number_of_columns {
                    used += maze.tiles[(j, i)]
                        .nodes
                        .iter()
                        .map(|t| t.count_ones() as usize)
                        .sum::<usize>();
                }
            }
            let listed: usize = maze.nets.iter().map(|net| net.len()).sum();
            assert_eq!(listed, used, "seed {seed}");
            assert_eq!(canonical(&maze.nets).len(), maze.nets.len(), "seed {seed}");
        }
    }

    #[test]
    fn test_rebuild_nets_joins_across_a_shared_node() {
        // (0,0) and (1,0) share node 5; (2,0) is separate.
        let mut maze = Maze::new(3, 1, false, false, false, None);
        maze.tiles[(0, 0)] = Tile::new(vec![N1 | N5], true);
        maze.tiles[(0, 1)] = Tile::new(vec![N5], true);
        maze.tiles[(0, 2)] = Tile::new(vec![N1], true);
        maze.rebuild_nets();
        assert_eq!(maze.nets.len(), 2);
        assert_eq!(maze.nets[0].len(), 3);
    }

    #[test]
    fn test_rebuild_nets_is_deterministic() {
        let mut a = Maze::new(12, 8, true, false, false, Some(5));
        let mut b = Maze::new(12, 8, true, false, false, Some(5));
        a.rebuild_nets();
        b.rebuild_nets();
        let order = |m: &Maze| {
            m.nets
                .iter()
                .map(|net| net.iter().map(|n| (n.i, n.j, n.node)).min())
                .collect::<Vec<_>>()
        };
        assert_eq!(order(&a), order(&b));
    }

    #[test]
    fn test_rebuild_nets_empty_maze() {
        let mut maze = Maze::new(3, 3, false, false, false, None);
        maze.rebuild_nets();
        assert!(maze.nets.is_empty());
    }
}

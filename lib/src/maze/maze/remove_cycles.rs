//! Cycle removal: turns every net into a tree, so there is one route between any
//! two points.

use std::collections::HashMap;

use super::net_operations::DisjointSets;
use crate::maze::maze::Maze;
use crate::maze::net::Node;
use crate::maze::tile::Tile;
use common::nodes::get_connected_coords_at_coords;
use common::types::coordinate::Coordinate;
use rand::seq::SliceRandom;

/// The maze as a graph: node positions and tile nets are the vertices.
///
/// Each tile net is joined to each of its node positions (a hub, so a junction is
/// not mistaken for a loop), and each node position is joined to the same physical
/// node in the tile across it.
struct MazeGraph {
    positions: Vec<Node>,
    /// `(tile net vertex, position vertex)` joins; tile net vertices start after the positions.
    tile_joins: Vec<(usize, usize)>,
    /// `(position, position)` joins across tile boundaries.
    crossing_joins: Vec<(usize, usize)>,
    vertex_count: usize,
}

impl MazeGraph {
    fn new(maze: &Maze) -> Self {
        let mut positions = Vec::new();
        let mut position_index: HashMap<Node, usize> = HashMap::new();
        let mut tile_nets: Vec<Vec<usize>> = Vec::new();
        for j in 0..maze.number_of_rows {
            for i in 0..maze.number_of_columns {
                for &tile_net in &maze.tiles[(j, i)].nodes {
                    let members = (0..6)
                        .filter(|k| tile_net & (1 << k) != 0)
                        .map(|k| {
                            let node = Node::new(i, j, k);
                            *position_index.entry(node).or_insert_with(|| {
                                positions.push(node);
                                positions.len() - 1
                            })
                        })
                        .collect();
                    tile_nets.push(members);
                }
            }
        }

        let tile_joins = tile_nets
            .iter()
            .enumerate()
            .flat_map(|(t, members)| members.iter().map(move |&p| (t, p)))
            .map(|(t, p)| (positions.len() + t, p))
            .collect();

        let crossing_joins = positions
            .iter()
            .enumerate()
            .filter_map(|(p, node)| {
                let here = Coordinate {
                    i: node.i,
                    j: node.j,
                };
                let across = get_connected_coords_at_coords(here, node.node)?;
                let q = *position_index.get(&Node::new(across.i, across.j, node.node))?;
                (p < q).then_some((p, q))
            })
            .collect();

        Self {
            vertex_count: positions.len() + tile_nets.len(),
            positions,
            tile_joins,
            crossing_joins,
        }
    }
}

impl Maze {
    /// Removes every loop from the maze without splitting any net.
    ///
    /// Joins are added to a union-find in this order (Kruskal's algorithm):
    /// 1. all crossing joins (each position has at most one partner across a tile
    ///    boundary, so these can never form a loop on their own);
    /// 2. the tile-net joins, shuffled with the maze's seed.
    ///
    /// A tile-net join whose ends are already connected would close a loop, so it is
    /// cut: that node is detached from its tile net and kept as a single-node net (a
    /// dead end into the tile). Cutting a join that closes a loop never disconnects
    /// anything, so the number of nets is unchanged. `self.nets` is rebuilt afterwards.
    pub fn remove_cycles(&mut self, debug: bool) {
        let graph = MazeGraph::new(self);
        let mut connected = DisjointSets::new(graph.vertex_count);
        for &(p, q) in &graph.crossing_joins {
            connected.union(p, q);
        }

        let mut tile_joins = graph.tile_joins.clone();
        tile_joins.shuffle(&mut self.seeded_rng());

        let cuts: Vec<Node> = tile_joins
            .into_iter()
            .filter(|&(tile_net, position)| !connected.union(tile_net, position))
            .map(|(_, position)| graph.positions[position])
            .collect();

        for node in &cuts {
            self.detach_node(*node);
        }
        self.rebuild_nets();

        if debug {
            println!(
                "Cycle removal: cut {} loops, {} nets",
                cuts.len(),
                self.nets.len()
            );
        }
    }

    /// Moves `node` out of its tile net into a single-node tile net of its own.
    fn detach_node(&mut self, node: Node) {
        let mask = 1 << node.node;
        let mut tile_nets: Vec<usize> = self.tiles[(node.j, node.i)]
            .nodes
            .iter()
            .map(|&t| t & !mask)
            .filter(|&t| t != 0)
            .collect();
        tile_nets.push(mask);
        self.tiles[(node.j, node.i)] = Tile::new(tile_nets, true);
    }

    /// The number of independent loops in the maze (joins − vertices + components).
    #[cfg(test)]
    pub(crate) fn loop_count(&self) -> usize {
        let graph = MazeGraph::new(self);
        let mut connected = DisjointSets::new(graph.vertex_count);
        let joins = graph.crossing_joins.iter().chain(&graph.tile_joins);
        joins.filter(|&&(a, b)| !connected.union(a, b)).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::maze::maze::with_large_stack;
    use common::consts::{N2, N4, N5, N6};

    macro_rules! place_tile {
        ($maze:expr, $nodes:expr, $i:expr, $j:expr) => {{
            let tile = Tile::new($nodes, true);
            $maze.update_network(&tile, $i, $j);
            $maze.tiles[($j, $i)] = tile;
        }};
    }

    #[test]
    fn test_remove_cycles_with_simple_loop() {
        let mut maze = Maze::new(3, 3, false, true, true, None);

        place_tile!(maze, vec![N5], 0, 0);
        place_tile!(maze, vec![N5 | N4], 1, 0);
        place_tile!(maze, vec![N5 | N4 | N6], 1, 1);
        place_tile!(maze, vec![N5 | N6], 0, 1);
        place_tile!(maze, vec![N5 | N6], 2, 1);
        place_tile!(maze, vec![N5 | N6], 0, 2);
        place_tile!(maze, vec![N5 | N6], 1, 2);
        place_tile!(maze, vec![N5 | N6], 2, 2);
        maze.reduce_nets(0);
        maze.remove_cycles(false);
        assert_eq!(maze.nets.len(), 1);
        assert_eq!(maze.loop_count(), 0);
    }

    #[test]
    fn test_remove_cycles_no_loops() {
        let mut maze = Maze::new(4, 3, false, true, true, None);

        place_tile!(maze, vec![N5], 0, 0);
        place_tile!(maze, vec![N5 | N4], 1, 0);
        place_tile!(maze, vec![N4 | N6, N5 | N2], 1, 1);
        place_tile!(maze, vec![N2], 3, 0);
        place_tile!(maze, vec![N5 | N6], 0, 1);
        place_tile!(maze, vec![N5 | N6], 2, 1);
        place_tile!(maze, vec![N5 | N6], 0, 2);
        place_tile!(maze, vec![N5 | N6], 1, 2);
        place_tile!(maze, vec![N5 | N6], 2, 2);

        maze.reduce_nets(0);
        maze.remove_cycles(false);

        assert_eq!(maze.nets.len(), 1);
        assert_eq!(maze.loop_count(), 0);
    }

    #[test]
    fn test_remove_cycles_cross_loop() {
        let mut maze = Maze::new(4, 3, false, true, true, None);

        place_tile!(maze, vec![N5], 0, 0);
        place_tile!(maze, vec![N5 | N4], 1, 0);
        place_tile!(maze, vec![N4 | N6 | N5 | N2], 1, 1);
        place_tile!(maze, vec![N2], 3, 0);
        place_tile!(maze, vec![N5 | N6], 0, 1);
        place_tile!(maze, vec![N5 | N6], 2, 1);
        place_tile!(maze, vec![N5 | N6], 0, 2);
        place_tile!(maze, vec![N5 | N6], 1, 2);
        place_tile!(maze, vec![N5 | N6], 2, 2);

        maze.reduce_nets(0);
        maze.remove_cycles(false);

        assert_eq!(maze.nets.len(), 1);
        assert_eq!(maze.loop_count(), 0);
    }

    #[test]
    fn test_remove_cycles_two_cycles_same_net() {
        let mut maze = Maze::new(5, 5, false, true, true, None);

        // Create two cycles within the same network
        // First cycle
        place_tile!(maze, vec![N5], 0, 0);
        place_tile!(maze, vec![N5 | N4], 1, 0);
        place_tile!(maze, vec![N4 | N6 | N5], 1, 1);
        place_tile!(maze, vec![N5 | N6], 0, 1);
        place_tile!(maze, vec![N5 | N6], 2, 1);
        place_tile!(maze, vec![N5 | N6], 0, 2);
        place_tile!(maze, vec![N5 | N6], 1, 2);
        place_tile!(maze, vec![N5 | N6 | N4], 2, 2);

        // Second cycle
        place_tile!(maze, vec![N6 | N4], 3, 2);
        place_tile!(maze, vec![N6 | N5 | N4], 3, 3);
        place_tile!(maze, vec![N5 | N4], 2, 3);
        place_tile!(maze, vec![N5 | N4], 4, 3);
        place_tile!(maze, vec![N5 | N4], 2, 4);
        place_tile!(maze, vec![N5 | N4], 3, 4);
        place_tile!(maze, vec![N5 | N4], 4, 4);

        maze.reduce_nets(0);
        maze.remove_cycles(false);

        // After cycle removal, we should have at least one network
        assert_eq!(maze.nets.len(), 1);
        assert_eq!(maze.loop_count(), 0);
    }

    #[test]
    fn test_remove_cycles_two_cycles_same_net_overlapping() {
        let mut maze = Maze::new(5, 5, false, true, true, None);

        // First cycle (2x2)
        place_tile!(maze, vec![N5], 0, 0);
        place_tile!(maze, vec![N5 | N4], 1, 0);
        place_tile!(maze, vec![N4 | N6 | N5], 1, 1);
        place_tile!(maze, vec![N5 | N6], 0, 1);

        // Second cycle overlapping at tile (1,1)
        place_tile!(maze, vec![N5 | N6], 2, 1);
        place_tile!(maze, vec![N6 | N5], 2, 2);
        place_tile!(maze, vec![N5 | N4 | N6], 1, 2);

        // Additional tiles to ensure connectivity
        place_tile!(maze, vec![N5 | N6], 0, 2);
        place_tile!(maze, vec![N4 | N5 | N6], 1, 3);
        place_tile!(maze, vec![N5 | N6], 0, 3);
        place_tile!(maze, vec![N5 | N6], 2, 3);
        place_tile!(maze, vec![N5 | N6], 0, 4);
        place_tile!(maze, vec![N5 | N6], 1, 4);
        place_tile!(maze, vec![N5 | N6], 2, 4);

        maze.reduce_nets(0);
        maze.remove_cycles(false);

        // After cycle removal, we should have at least one network
        assert_eq!(maze.nets.len(), 1);
        assert_eq!(maze.loop_count(), 0);
    }
    #[test]
    fn test_remove_cycles_five_cycles() {
        let mut maze = Maze::new(7, 5, false, true, true, None);

        // First cycle (2x2)
        place_tile!(maze, vec![N5], 0, 0);
        place_tile!(maze, vec![N5 | N4], 1, 0);
        place_tile!(maze, vec![N4 | N6 | N5], 1, 1);
        place_tile!(maze, vec![N5 | N6], 0, 1);

        // Bottom right
        place_tile!(maze, vec![N4 | N5 | N6], 4, 1);
        place_tile!(maze, vec![N4 | N6], 5, 1);
        place_tile!(maze, vec![N4 | N6], 6, 1);
        place_tile!(maze, vec![N4 | N6], 4, 2);
        place_tile!(maze, vec![N4 | N5 | N6], 5, 2);
        place_tile!(maze, vec![N4 | N6], 6, 2);

        // Top right
        place_tile!(maze, vec![N4 | N6], 4, 3);
        place_tile!(maze, vec![N4 | N5 | N6], 5, 3);
        place_tile!(maze, vec![N4 | N6], 6, 3);
        place_tile!(maze, vec![N4 | N5 | N6], 4, 4);
        place_tile!(maze, vec![N4 | N6], 5, 4);
        place_tile!(maze, vec![N4 | N6], 6, 4);

        // Join left and right loops
        place_tile!(maze, vec![N4 | N5], 3, 4);
        place_tile!(maze, vec![N4 | N5], 3, 1);

        // Second cycle overlapping at tile (1,1)
        place_tile!(maze, vec![N5 | N4 | N6], 2, 1);
        place_tile!(maze, vec![N6 | N5], 2, 2);
        place_tile!(maze, vec![N5 | N4 | N6], 1, 2);

        // Additional tiles to ensure connectivity
        place_tile!(maze, vec![N5 | N6], 0, 2);
        place_tile!(maze, vec![N4 | N5 | N6], 1, 3);
        place_tile!(maze, vec![N5 | N6], 0, 3);
        place_tile!(maze, vec![N5 | N6], 2, 3);
        place_tile!(maze, vec![N5 | N6], 0, 4);
        place_tile!(maze, vec![N5 | N6], 1, 4);
        place_tile!(maze, vec![N5 | N4 | N6], 2, 4);

        maze.reduce_nets(0);
        maze.remove_cycles(false);

        // After cycle removal, we should have at least one network
        assert_eq!(maze.nets.len(), 1);
        assert_eq!(maze.loop_count(), 0);
    }

    #[test]
    fn test_remove_cycles_random_mazes() {
        with_large_stack(|| {
            for seed in [1, 2, 3, 7, 42] {
                let mut maze = Maze::new(20, 14, true, false, false, Some(seed));
                maze.rebuild_nets();
                let nets_before = maze.nets.len();
                assert!(maze.loop_count() > 0, "seed {seed} should start with loops");
                maze.remove_cycles(false);
                assert_eq!(maze.loop_count(), 0, "seed {seed}");
                assert_eq!(maze.nets.len(), nets_before, "seed {seed} split a net");
            }
        });
    }

    #[test]
    fn test_remove_cycles_after_reduce_leaves_one_tree() {
        with_large_stack(|| {
            for seed in 1..=10 {
                let mut maze = Maze::new(30, 20, true, false, false, Some(seed));
                maze.reduce_nets(1);
                maze.remove_cycles(false);
                assert_eq!(maze.nets.len(), 1, "seed {seed}");
                assert_eq!(maze.loop_count(), 0, "seed {seed}");
            }
        });
    }
}

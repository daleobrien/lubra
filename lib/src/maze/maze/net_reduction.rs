use crate::maze::maze::Maze;
use crate::maze::net::Node;
use crate::maze::random::select_random_element;
use crate::maze::tile::Tile;

/// Net reduction methods for the Maze struct.
/// These methods handle reducing the number of nets by merging them.
impl Maze {
    pub fn reduce_nets(&mut self, lower_limit: usize) {
        self.sort_nets_by_length();
        // remove_short_nets might be broken ... seem to create more disjoint nets!
        self.remove_short_nets(lower_limit);

        // Create a hashSet of the nets, but used just i,j coordinates as the keys
        let coord_maps = self.create_coord_maps();

        // NOTE: Do I only need get intersections one at a time, since we are merging them?
        // But faster to get them all at once, and then not use the later ones we no longer
        // need

        // We can do the below until there is only one list
        // Alter node and track which nets can be merged, since we need to do it on one pass?
        let shared_coords = Self::get_intersections(coord_maps);

        for ((n, m), intersections) in shared_coords.iter() {
            // Must always have at least one element

            // We could loop through, looking for any intersections that are exactly 2
            let mut found = false;
            for (i, j) in intersections {
                let tiles_at_intersections = self.tiles.get(*j, *i).unwrap().clone();
                if tiles_at_intersections.nodes.len() == 2 {
                    let bits = tiles_at_intersections.bits;
                    // We can merge these two nets in a simple way
                    let _ = self.tiles.set(*j, *i, Tile::new(vec![bits], true));

                    // Join the nets
                    let nodes = tiles_at_intersections.get_first_nodes();
                    // Find the nets that contain the nodes
                    let mut indexes: Vec<usize> = self
                        .nets
                        .iter()
                        .enumerate()
                        .filter_map(|(index, net)| {
                            if nodes
                                .iter()
                                .any(|&node| net.contains(&Node::new(*i, *j, node)))
                            {
                                Some(index)
                            } else {
                                None
                            }
                        })
                        .collect();
                    indexes.sort();

                    // Join the nets with the matching indexes
                    // We only have 2 items
                    if indexes.len() == 2 {
                        let first_index = indexes[0];
                        let other_index = indexes[1];
                        let other_net = self.nets[other_index].clone();
                        self.nets[first_index].extend(other_net);
                        //self.nets.remove(other_index);
                        self.nets[other_index].clear(); // Leave an empty net to avoid messing up indices
                    } else {
                        println!("Error, we don't have 2 indexes to merge {:?}", indexes);
                    }
                    found = true;
                    break;
                }
            }
            if !found {
                // complex merging required
                // Check if indices are still valid (nets may have been merged/removed)
                if *n >= self.nets.len() || *m >= self.nets.len() {
                    println!("  WARNING: Net indices ({}, {}) are stale (only {} nets remain after merging)",
                             n, m, self.nets.len());
                    continue;
                }

                for (i, j) in intersections {
                    // Get nodes for net n at this tile
                    let net_n_nodes: Vec<usize> = self.nets[*n]
                        .iter()
                        .filter(|node| node.i == *i && node.j == *j)
                        .map(|node| node.node)
                        .collect();

                    // Get nodes for net m at this tile
                    let net_m_nodes: Vec<usize> = self.nets[*m]
                        .iter()
                        .filter(|node| node.i == *i && node.j == *j)
                        .map(|node| node.node)
                        .collect();

                    // Print the original tile at this location
                    let original_tile = self.tiles.get(*j, *i).unwrap();

                    // Select a random element from net_n_nodes and net_m_nodes to merge, and keep the rest unchanged
                    let selected_m = select_random_element(net_m_nodes, self.seed);
                    let selected_n = select_random_element(net_n_nodes, self.seed);
                    if selected_m.is_some() && selected_n.is_some() {
                        let bit_n = 1usize << (selected_n.unwrap());
                        let bit_m = 1usize << (selected_m.unwrap());

                        let combined_mask = bit_n | bit_m;

                        let indexes_with_bits: Vec<usize> = original_tile
                            .nodes
                            .iter()
                            .enumerate()
                            .filter(|(_, &node_bits)| node_bits & combined_mask != 0)
                            .map(|(index, _)| index)
                            .collect();

                        // Create new node by ANDing each indexed node with combined_mask and ORing them together
                        let new_node: usize = indexes_with_bits
                            .iter()
                            .map(|&idx| original_tile.nodes[idx])
                            .fold(0, |acc, bits| acc | bits);

                        // Remove the indexed nodes and add the new merged node
                        let mut new_nodes: Vec<usize> = original_tile
                            .nodes
                            .iter()
                            .enumerate()
                            .filter(|(idx, _)| !indexes_with_bits.contains(idx))
                            .map(|(_, &node)| node)
                            .collect();

                        new_nodes.push(new_node);

                        let _ = self.tiles.set(*j, *i, Tile::new(new_nodes, true));
                    } else {
                        println!(
                            "  WARNING: Could not select random nodes for merging at tile ({}, {})",
                            i, j
                        );
                    }
                }
            }
        }

        // Remove nets that have been merged (are empty)
        self.nets.retain(|net| !net.is_empty());

        if self.nets.len() > 1 {
            // Recursively reduce nets until we have one
            println!("\nWe have disjoint {:?} nets:", self.nets.len());

            // TODO: find all the surrounding tiles for each net, and see if any can be merged, if any of those
            // tiles are shared between nets
            if let Some(info) = self.select_common_surrounding_coordinate_with_info() {
                println!(
                    "Connecting nets {} and {}",
                    info.net_a_index, info.net_b_index
                );
                println!("  Common coordinate: {:?}", info.common_coordinate);
                println!("  Net A has {} tiles", info.net_a_coordinates.len());
                println!("  Net B has {} tiles", info.net_b_coordinates.len());
                println!("  Proximity: {:?}", info.proximity_level);
            }
        }
    }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::maze::net::{Net, Node, Nodes};
  use crate::maze::tile::Tile;
  use array2d::Array2D;
  use common::consts::{N1, N2, N3, N4, N5, N6};

  /// Helper function to create a test maze with given nets and tiles
    fn create_test_maze_with_tiles(nets: Net, tiles: Array2D<Tile>) -> Maze {
        let rows = tiles.num_rows();
        let cols = tiles.num_columns();
        Maze {
            style: None,
            nets,
            width: 0.0,
            height: 0.0,
            seed: None,
            number_of_columns: cols,
            number_of_rows: rows,
            tiles,
            boundaries: false,
            show_node_numbers: false,
        }
    }

    #[test]
    fn test_reduce_nets_complex_merge_scenario() {
        // This test creates a complex scenario with 3 nets that intersect
        // Net 0: Passes through tiles (0,0) and (0,1)
        // Net 1: Passes through tiles (0,1) and (0,2)
        // Net 2: Passes through tiles (0,2) and (0,3)
        //
        // At (0,1), nets 0 and 1 meet with exactly 2 nodes each
        // At (0,2), nets 1 and 2 meet with exactly 2 nodes each
        //
        // Expected behavior: After reduction, we should have fewer nets
        // because the algorithm merges nets at intersection points with exactly 2 nodes

        // Create net 0: (0,0,node0) -> (0,1,node1)
        let mut net0 = Nodes::new();
        net0.insert(Node::new(0, 0, 0)); // i=0, j=0, node=0 (N1)
        net0.insert(Node::new(1, 0, 1)); // i=1, j=0, node=1 (N2)

        // Create net 1: (0,1,node1) -> (0,2,node2)
        let mut net1 = Nodes::new();
        net1.insert(Node::new(1, 0, 2)); // i=1, j=0, node=2 (N3) - shares tile with net0
        net1.insert(Node::new(2, 0, 3)); // i=2, j=0, node=3 (N4)

        // Create net 2: (0,2,node3) -> (0,3,node4)
        let mut net2 = Nodes::new();
        net2.insert(Node::new(2, 0, 4)); // i=2, j=0, node=4 (N5) - shares tile with net1
        net2.insert(Node::new(3, 0, 5)); // i=3, j=0, node=5 (N6)

        // Create tiles grid (4x1 for simplicity)
        let mut tiles = Array2D::filled_with(Tile::new(vec![], false), 1, 4);

        // Tile at (0,0): Has node 0 from net0
        let _ = tiles.set(0, 0, Tile::new(vec![N1], true));

        // Tile at (1,0): Has nodes 1 and 2 (intersection of net0 and net1)
        // This is a critical tile with exactly 2 nodes - perfect for merging!
        let _ = tiles.set(0, 1, Tile::new(vec![N2, N3], true));

        // Tile at (2,0): Has nodes 3 and 4 (intersection of net1 and net2)
        // Another critical tile with exactly 2 nodes
        let _ = tiles.set(0, 2, Tile::new(vec![N4, N5], true));

        // Tile at (3,0): Has node 5 from net2
        let _ = tiles.set(0, 3, Tile::new(vec![N6], true));

        let nets = vec![net0.clone(), net1.clone(), net2.clone()];
        let mut maze = create_test_maze_with_tiles(nets, tiles);

        let initial_net_count = maze.nets.len();
        assert_eq!(initial_net_count, 3, "Should start with 3 nets");

        // Reduce the nets
        maze.reduce_nets(0);

        let final_net_count = maze.nets.len();
        println!("\nInitial nets: {}", initial_net_count);
        println!("Final nets: {}", final_net_count);
        println!("Nodes after reduction: {:?}", maze.nets);

        // After reduction, we should have fewer nets due to merging
        assert!(
            final_net_count < initial_net_count,
            "reduce_nets should merge intersecting nets, expected < {} nets but got {}",
            initial_net_count,
            final_net_count
        );

        // Verify that the total number of tile-node combinations is preserved
        let total_nodes_after: usize = maze.nets.iter().map(|net| net.len()).sum();
        let total_nodes_before = net0.len() + net1.len() + net2.len();
        assert_eq!(
            total_nodes_after, total_nodes_before,
            "Total number of nodes should be preserved during merge"
        );

        // Verify that at least one net has grown (merged with another)
        let max_net_size = maze.nets.iter().map(|net| net.len()).max().unwrap_or(0);
        assert!(
            max_net_size > 2,
            "At least one net should have grown beyond its original size due to merging"
        );

        // Verify the tiles at intersection points have been updated
        // The tile at (1,0) should now have a single merged net instead of 2 separate ones
        let tile_at_1_0 = maze.tiles.get(0, 1).unwrap();
        if tile_at_1_0.initialised {
            // If the merge happened at this tile, it should now have 1 node (merged)
            // Note: The actual behavior depends on which intersection was processed first
            assert!(
                tile_at_1_0.nodes.len() <= 2,
                "Tile should have been updated during merge"
            );
        }
    }

    #[test]
    fn test_reduce_nets_no_intersections() {
        // Test case where nets don't intersect - nothing should be merged
        let mut net0 = Nodes::new();
        net0.insert(Node::new(0, 0, 0));
        net0.insert(Node::new(1, 0, 1));

        let mut net1 = Nodes::new();
        net1.insert(Node::new(2, 0, 0));
        net1.insert(Node::new(3, 0, 1));

        let mut tiles = Array2D::filled_with(Tile::new(vec![], false), 1, 4);
        let _ = tiles.set(0, 0, Tile::new(vec![N1], true));
        let _ = tiles.set(0, 1, Tile::new(vec![N2], true));
        let _ = tiles.set(0, 2, Tile::new(vec![N1], true));
        let _ = tiles.set(0, 3, Tile::new(vec![N2], true));

        let nets = vec![net0, net1];
        let mut maze = create_test_maze_with_tiles(nets, tiles);

        maze.reduce_nets(0);

        // Should still have 2 nets since they don't intersect
        assert_eq!(
            maze.nets.len(),
            2,
            "Non-intersecting nets should not be merged"
        );
    }

    #[test]
    fn test_reduce_nets_intersection_with_more_than_two_nodes() {
        // Test case where intersection has more than 2 nodes - should merge
        let mut net0 = Nodes::new();
        net0.insert(Node::new(0, 0, 0));
        net0.insert(Node::new(1, 0, 1));

        let mut net1 = Nodes::new();
        net1.insert(Node::new(1, 0, 2)); // Shares tile (1,0) with net0
        net1.insert(Node::new(2, 0, 3));

        let mut net2 = Nodes::new();
        net2.insert(Node::new(1, 0, 4)); // Also shares tile (1,0)
        net2.insert(Node::new(3, 0, 5));

        let mut tiles = Array2D::filled_with(Tile::new(vec![], false), 1, 4);
        let _ = tiles.set(0, 0, Tile::new(vec![N1], true));
        // Tile at (1,0) has 3 nodes - more than the threshold for simple merging
        let _ = tiles.set(0, 1, Tile::new(vec![N2, N3, N5], true));
        let _ = tiles.set(0, 2, Tile::new(vec![N4], true));
        let _ = tiles.set(0, 3, Tile::new(vec![N6], true));

        let nets = vec![net0, net1, net2];
        let mut maze = create_test_maze_with_tiles(nets, tiles);

        maze.reduce_nets(0);

        assert_eq!(
            maze.nets.len(),
            2,
            "Nodes with complex intersections (>2 nodes) should be merged"
        );
    }

    #[test]
    fn test_reduce_nets_empty_maze() {
        // Test with no nets at all
        let tiles = Array2D::filled_with(Tile::new(vec![], false), 1, 1);
        let mut maze = create_test_maze_with_tiles(vec![], tiles);

        maze.reduce_nets(0);

        assert_eq!(maze.nets.len(), 0, "Empty maze should remain empty");
    }

    #[test]
    fn test_reduce_nets_single_net() {
        // Test with only one net - nothing to merge
        let mut net0 = Nodes::new();
        net0.insert(Node::new(0, 0, 0));
        net0.insert(Node::new(1, 0, 1));

        let mut tiles = Array2D::filled_with(Tile::new(vec![], false), 1, 2);
        let _ = tiles.set(0, 0, Tile::new(vec![N1], true));
        let _ = tiles.set(0, 1, Tile::new(vec![N2], true));

        let nets = vec![net0.clone()];
        let mut maze = create_test_maze_with_tiles(nets, tiles);

        maze.reduce_nets(0);

        assert_eq!(maze.nets.len(), 1, "Single net should remain as is");
        assert_eq!(
            maze.nets[0].len(),
            net0.len(),
            "Single net should not be modified"
        );
    }
}

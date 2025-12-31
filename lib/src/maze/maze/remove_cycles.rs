use crate::maze::maze::Maze;
use crate::maze::net::Node;
use common::consts::NODE_TO_COORD;
use common::types::coordinate::add;
use std::collections::{HashMap, HashSet};

/// Information about a detected cycle in the network.
#[derive(Debug, Clone)]
pub struct CycleInfo {
    /// The index of the network containing the cycle
    pub net_index: usize,
    /// The nodes that form the cycle (in order)
    pub cycle_nodes: Vec<Node>,
}

/// Cycle removal methods for the Maze struct.
/// Detects and handles cycles in the maze network using DFS.
impl Maze {
    /// Normalize a cycle to a canonical form for deduplication.
    ///
    /// A cycle like [A, B, C, D] is the same as [B, C, D, A] or [D, C, B, A] (reversed).
    /// This function rotates the cycle to start with the smallest node and ensures
    /// the second node is smaller than the last node (to handle both directions).
    ///
    /// Returns the normalized cycle without the closing duplicate node.
    fn normalize_cycle(cycle: &[Node]) -> Vec<Node> {
        if cycle.len() <= 2 {
            return cycle.to_vec();
        }

        // Remove the last node if it's a duplicate of the first (closes the cycle)
        let nodes = if !cycle.is_empty() && cycle[0] == cycle[cycle.len() - 1] {
            &cycle[..cycle.len() - 1]
        } else {
            cycle
        };

        if nodes.is_empty() {
            return vec![];
        }

        // Find the position of the minimum node
        let min_pos = nodes
            .iter()
            .enumerate()
            .min_by_key(|(_, node)| (node.i, node.j, node.node))
            .map(|(i, _)| i)
            .unwrap_or(0);

        // Rotate to start with minimum node
        let mut normalized: Vec<Node> = nodes[min_pos..]
            .iter()
            .chain(nodes[..min_pos].iter())
            .copied()
            .collect();

        // Check if we should reverse (to handle both clockwise and counter-clockwise)
        // Compare the second node with the last node - keep the lexicographically smaller order
        if normalized.len() > 2 {
            let second = normalized[1];
            let last = normalized[normalized.len() - 1];
            if (last.i, last.j, last.node) < (second.i, second.j, second.node) {
                // Reverse (keeping first element in place)
                let first = normalized[0];
                normalized[1..].reverse();
                normalized[0] = first;
            }
        }

        normalized
    }

    /// Detects all cycles in the network using depth-first search.
    ///
    /// Returns a vector of CycleInfo containing:
    /// - The network index
    /// - The nodes that form each cycle (in order)
    ///
    /// # Algorithm
    /// For each network:
    /// 1. Build an adjacency map showing which nodes connect to which
    /// 2. Perform DFS from each unvisited node
    /// 3. Detect back edges (edges to nodes in the current path)
    /// 4. Extract the cycle when a back edge is found
    /// 5. Deduplicate cycles by normalizing to canonical form
    ///
    /// Note: Nodes within the same tile (same i, j coordinates) are treated as
    /// the same node for cycle detection, since intra-tile connections don't
    /// form meaningful cycles.
    pub fn detect_cycles(&self) -> Vec<CycleInfo> {
        let mut all_cycles = Vec::new();
        let mut seen_cycles: HashSet<Vec<Node>> = HashSet::new();

        for (net_idx, nodes) in self.nets.iter().enumerate() {
            // Build adjacency map for this network
            let adjacency = self.build_adjacency_map(nodes);

            // Track visited tiles (i, j) globally and tiles in current DFS path
            // Using tile position instead of full Node since nodes in the same tile
            // are considered the same node for cycle detection
            let mut visited: HashSet<(usize, usize)> = HashSet::new();
            let mut rec_stack: HashSet<(usize, usize)> = HashSet::new();
            let mut path = Vec::new();

            // Try starting DFS from each unvisited tile
            for &start_node in nodes.iter() {
                let start_tile = (start_node.i, start_node.j);
                if !visited.contains(&start_tile) {
                    if let Some(cycle) = self.dfs_detect_cycle(
                        start_node,
                        None,
                        &adjacency,
                        &mut visited,
                        &mut rec_stack,
                        &mut path,
                    ) {
                        // Normalize the cycle to check for duplicates
                        let normalized = Self::normalize_cycle(&cycle);

                        // Only add if we haven't seen this cycle before
                        if !seen_cycles.contains(&normalized) {
                            seen_cycles.insert(normalized);
                            all_cycles.push(CycleInfo {
                                net_index: net_idx,
                                cycle_nodes: cycle,
                            });
                        }
                        // Else: skip this duplicate cycle
                    }
                }
            }
        }

        all_cycles
    }

    /// Build adjacency map for nodes in a network.
    /// Each node maps to its connected neighbors within the same network.
    fn build_adjacency_map(&self, nodes: &HashSet<Node>) -> HashMap<Node, Vec<Node>> {
        let mut adjacency: HashMap<Node, Vec<Node>> = HashMap::new();

        for &node in nodes.iter() {
            let neighbors = self.get_connected_nodes(node, nodes);
            adjacency.insert(node, neighbors);
        }

        adjacency
    }

    /// Get nodes that are connected to the given node within the network.
    ///
    /// For a hexagonal grid, connections can be:
    /// 1. Intra-tile: nodes within the same tile that share the same net
    /// 2. Inter-tile: nodes in adjacent tiles (matching node indices)
    ///
    /// This handles bidirectional connections.
    fn get_connected_nodes(&self, node: Node, network: &HashSet<Node>) -> Vec<Node> {
        // TODO: Is this the same as self.nets ?

        let mut connected = Vec::new();

        // 1. Intra-tile connections: nodes in the same tile and network
        // Check the tile to see which nodes are connected within it
        if let Some(tile) = self.tiles.get(node.j, node.i) {
            if tile.initialised {
                // For each net in the tile, check if our node is part of it
                for net_bits in &tile.nodes {
                    let node_mask = 1usize << node.node;
                    if net_bits & node_mask != 0 {
                        // Our node is part of this net, find other nodes in same net
                        for other_node_idx in 0..6 {
                            if other_node_idx != node.node {
                                let other_mask = 1usize << other_node_idx;
                                if net_bits & other_mask != 0 {
                                    // This node is also part of the same net
                                    let other_node = Node::new(node.i, node.j, other_node_idx);
                                    if network.contains(&other_node) {
                                        connected.push(other_node);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // 2. Inter-tile connections: node -> adjacent tile's corresponding node
        let mapping = NODE_TO_COORD[(node.i + (3 * (node.j % 2))) % 6];
        let coord = mapping[node.node];

        let adj_i = add(node.i, coord.0);
        let adj_j = add(node.j, coord.1);

        match (adj_i, adj_j) {
            (Some(i), Some(j)) => {
                let adjacent_node = Node::new(i, j, node.node);
                if network.contains(&adjacent_node) && !connected.contains(&adjacent_node) {
                    connected.push(adjacent_node);
                }
            }
            _ => {}
        }

        // 3. Reverse inter-tile connections: find nodes that connect to this node
        for &other_node in network.iter() {
            if other_node == node {
                continue;
            }

            // Skip nodes in same tile (already handled above)
            if other_node.i == node.i && other_node.j == node.j {
                continue;
            }

            // Check if other_node connects to our node
            let other_mapping = NODE_TO_COORD[(other_node.i + (3 * (other_node.j % 2))) % 6];
            let other_coord = other_mapping[other_node.node];

            let target_i = add(other_node.i, other_coord.0);
            let target_j = add(other_node.j, other_coord.1);

            match (target_i, target_j) {
                (Some(i), Some(j)) => {
                    if i == node.i && j == node.j && other_node.node == node.node {
                        if !connected.contains(&other_node) {
                            connected.push(other_node);
                        }
                    }
                }
                _ => {}
            }
        }

        connected
    }

    /// DFS-based cycle detection. Returns Some(cycle) if a cycle is found.
    ///
    /// # Arguments
    /// * `current` - Current node being visited
    /// * `parent_tile` - Parent tile position (to avoid detecting trivial back-edges)
    /// * `adjacency` - Adjacency map of the graph
    /// * `visited` - Set of all visited tile positions (i, j)
    /// * `rec_stack` - Set of tile positions in current recursion stack (path)
    /// * `path` - Current path being explored (as nodes for cycle reporting)
    ///
    /// # Returns
    /// Some(Vec<Node>) containing the cycle if found, None otherwise
    ///
    /// Note: Uses tile positions (i, j) for cycle detection since nodes within
    /// the same tile are considered the same node.
    fn dfs_detect_cycle(
        &self,
        current: Node,
        parent_tile: Option<(usize, usize)>,
        adjacency: &HashMap<Node, Vec<Node>>,
        visited: &mut HashSet<(usize, usize)>,
        rec_stack: &mut HashSet<(usize, usize)>,
        path: &mut Vec<Node>,
    ) -> Option<Vec<Node>> {
        let current_tile = (current.i, current.j);
        visited.insert(current_tile);
        rec_stack.insert(current_tile);
        path.push(current);

        if let Some(neighbors) = adjacency.get(&current) {
            for &neighbor in neighbors {
                let neighbor_tile = (neighbor.i, neighbor.j);

                // Skip neighbors on the same tile - intra-tile connections don't form cycles
                if neighbor_tile == current_tile {
                    continue;
                }

                // Skip the parent tile to avoid detecting trivial back-edges
                if Some(neighbor_tile) == parent_tile {
                    continue;
                }

                if !visited.contains(&neighbor_tile) {
                    // Recurse to unvisited neighbor
                    if let Some(cycle) = self.dfs_detect_cycle(
                        neighbor,
                        Some(current_tile),
                        adjacency,
                        visited,
                        rec_stack,
                        path,
                    ) {
                        return Some(cycle);
                    }
                } else if rec_stack.contains(&neighbor_tile) {
                    // Found a back edge to a tile in current path - this is a cycle!
                    // Find the position in path where this tile first appears
                    let cycle_start = path
                        .iter()
                        .position(|n| n.i == neighbor.i && n.j == neighbor.j)
                        .unwrap();
                    let mut cycle = path[cycle_start..].to_vec();
                    cycle.push(neighbor); // Close the cycle
                    return Some(cycle);
                }
            }
        }

        // Backtrack
        path.pop();
        rec_stack.remove(&current_tile);
        None
    }

    /// Split a network into connected components.
    ///
    /// After removing nodes, a network might become disconnected.
    /// This method finds all connected components and returns them as separate networks.
    fn split_disconnected_networks(&self, nodes: &HashSet<Node>) -> Vec<HashSet<Node>> {
        let mut components = Vec::new();
        let mut visited = HashSet::new();
        let adjacency = self.build_adjacency_map(nodes);

        for &start_node in nodes.iter() {
            if visited.contains(&start_node) {
                continue;
            }

            // BFS to find all nodes in this connected component
            let mut component = HashSet::new();
            let mut queue = vec![start_node];
            component.insert(start_node);
            visited.insert(start_node);

            while let Some(current) = queue.pop() {
                if let Some(neighbors) = adjacency.get(&current) {
                    for &neighbor in neighbors {
                        if !visited.contains(&neighbor) {
                            visited.insert(neighbor);
                            component.insert(neighbor);
                            queue.push(neighbor);
                        }
                    }
                }
            }

            components.push(component);
        }

        components
    }

    /// Remove cycles from the network by breaking cycle edges.
    ///
    /// This method iteratively detects cycles and removes nodes to break them.
    /// After each removal, it re-checks for new cycles. Once all cycles are
    /// broken, it splits disconnected networks into separate components.
    pub fn remove_cycles(&mut self) {
        println!("Removing cycles ...");

        let mut iteration = 0;
        let max_iterations = 100; // Safety limit

        // Iteratively break cycles one at a time
        loop {
            iteration += 1;
            if iteration > max_iterations {
                println!("Warning: Reached maximum iterations");
                break;
            }

            let cycles = self.detect_cycles();

            if cycles.is_empty() {
                println!("No more cycles detected");
                break;
            }

            println!("Iteration {}: Found {} cycle(s)", iteration, cycles.len());
            cycles.iter().for_each(|cycle_info| {
                println!(
                    "  Cycle in net {} with {} nodes",
                    cycle_info.net_index,
                    cycle_info.cycle_nodes.len()
                );
                cycle_info.cycle_nodes.iter().for_each(|node| {
                    println!("    ({},{}.{})", node.i, node.j, node.node + 1);
                });
            });

            // Break only the first cycle found
            let cycle_info = &cycles[0];
            println!(
                "  Breaking cycle in net {} with {} nodes",
                cycle_info.net_index,
                cycle_info.cycle_nodes.len()
            );

            // Strategy: Remove the last edge in the cycle
            // (the edge that closes the cycle)
            if cycle_info.cycle_nodes.len() >= 2 {
                let node_to_remove = cycle_info.cycle_nodes[cycle_info.cycle_nodes.len() - 2];

                println!(
                    "    Removing node at tile ({}, {}), node index {}",
                    node_to_remove.i,
                    node_to_remove.j,
                    node_to_remove.node + 1
                );

                // Remove the node from the network
                if cycle_info.net_index < self.nets.len() {
                    self.nets[cycle_info.net_index].remove(&node_to_remove);
                }
                // Update the tile too
                if let Some(tile) = self.tiles.get_mut(node_to_remove.j, node_to_remove.i) {
                    if tile.initialised {
                        let node_mask = 1usize << node_to_remove.node;
                        // Find the node that has the bit set and remove that bit
                        tile.nodes.iter_mut().for_each(|net_bits| {
                            if *net_bits & node_mask != 0 {
                                *net_bits &= !node_mask;
                            }
                        });
                        // Add the bit as a node
                        tile.nodes.push(node_mask);
                    }
                }
            }
        }

        // After removing all cycles, split any disconnected networks
        println!("Splitting disconnected networks...");
        let mut new_nets = Vec::new();
        for (idx, network) in self.nets.iter().enumerate() {
            let components = self.split_disconnected_networks(network);
            println!("  Net {} split into {} components", idx, components.len());
            new_nets.extend(components);
        }
        self.nets = new_nets;

        println!("Cycle removal complete: {} network(s)", self.nets.len());
    }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::maze::maze::FileType::SVG;
  use crate::maze::tile::Tile;
  use common::consts::{N2, N4, N5, N6};

  /// Helper macro to create a tile, update network, and place it in the maze.
    /// Note: coordinates are swapped between update_network(i, j) and tiles[(j, i)].
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
        // Should complete without panicking on an empty maze

        place_tile!(maze, vec![N5], 0, 0);
        place_tile!(maze, vec![N5 | N4], 1, 0);
        place_tile!(maze, vec![N5 | N4 | N6], 1, 1);
        place_tile!(maze, vec![N5 | N6], 0, 1);
        place_tile!(maze, vec![N5 | N6], 2, 1);
        place_tile!(maze, vec![N5 | N6], 0, 2);
        place_tile!(maze, vec![N5 | N6], 1, 2);
        place_tile!(maze, vec![N5 | N6], 2, 2);

        maze.reduce_nets(0);
        maze.remove_cycles();

        let name = format!("test-cycle-1");
        let _ = maze.render(name.to_string(), SVG);

        assert_eq!(maze.nets.len(), 1);
    }

    #[test]
    fn test_remove_cycles_no_loops() {
        let mut maze = Maze::new(4, 3, false, true, true, None);
        // Should complete without panicking on an empty maze

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
        maze.remove_cycles();

        let name = format!("test-cycle-2");
        let _ = maze.render(name.to_string(), SVG);

        assert_eq!(maze.nets.len(), 1);
    }

    #[test]
    fn test_normalize_cycle() {
        // Test that cycle normalization produces consistent results
        let n1 = Node::new(0, 0, 0);
        let n2 = Node::new(1, 0, 0);
        let n3 = Node::new(1, 1, 0);
        let n4 = Node::new(0, 1, 0);

        // Same cycle in different rotations should normalize to the same form
        let cycle1 = vec![n1, n2, n3, n4, n1]; // Starts at n1
        let cycle2 = vec![n2, n3, n4, n1, n2]; // Starts at n2
        let cycle3 = vec![n3, n4, n1, n2, n3]; // Starts at n3
        let cycle4 = vec![n4, n1, n2, n3, n4]; // Starts at n4

        let norm1 = Maze::normalize_cycle(&cycle1);
        let norm2 = Maze::normalize_cycle(&cycle2);
        let norm3 = Maze::normalize_cycle(&cycle3);
        let norm4 = Maze::normalize_cycle(&cycle4);

        println!("Normalized cycles:");
        println!("  cycle1 -> {:?}", norm1);
        println!("  cycle2 -> {:?}", norm2);
        println!("  cycle3 -> {:?}", norm3);
        println!("  cycle4 -> {:?}", norm4);

        // All should normalize to the same cycle
        assert_eq!(
            norm1, norm2,
            "cycle1 and cycle2 should normalize to the same form"
        );
        assert_eq!(
            norm2, norm3,
            "cycle2 and cycle3 should normalize to the same form"
        );
        assert_eq!(
            norm3, norm4,
            "cycle3 and cycle4 should normalize to the same form"
        );

        // Reversed cycle should also normalize to the same form
        let cycle_rev = vec![n4, n3, n2, n1, n4];
        let norm_rev = Maze::normalize_cycle(&cycle_rev);
        println!("  cycle_rev -> {:?}", norm_rev);
        assert_eq!(
            norm1, norm_rev,
            "Reversed cycle should normalize to the same form"
        );
    }

    #[test]
    fn test_detect_cycles_single_tile_no_cycle() {
        // Test that a "cycle" within a single tile is NOT detected as a cycle
        // since all nodes on the same tile are considered the same node
        let mut maze = Maze::new(2, 2, false, true, true, None);

        // Create a single tile with nodes N2-N4-N6 - these form a connected group
        // but NOT a cycle for maze purposes since they're all on the same tile
        place_tile!(maze, vec![N2 | N4 | N6], 0, 0);

        // Don't call reduce_nets - we want to test cycle detection directly
        let cycles = maze.detect_cycles();

        println!("Detected {} cycle(s) in single-tile group", cycles.len());

        // No cycles should be detected - intra-tile connections are not cycles
        assert_eq!(
            cycles.len(),
            0,
            "Expected 0 cycles for single-tile group, but found {}",
            cycles.len()
        );
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

        let name = format!("test-cycle-3.1");
        let _ = maze.render(name.to_string(), SVG);

        maze.remove_cycles();

        let name = format!("test-cycle-3.2");
        let _ = maze.render(name.to_string(), SVG);

        assert_eq!(maze.nets.len(), 1);
    }

    #[test]
    fn test_dfs_detect_cycle_single_tile_no_cycle() {
        // Test DFS cycle detection with nodes all on the same tile
        // Since nodes on the same tile are considered the same node, this should NOT detect a cycle
        let mut maze = Maze::new(2, 2, false, true, true, None);

        // Create a single tile with nodes N2|N4|N6
        // All on tile (0,0), so they're all considered the same node for cycle detection
        place_tile!(maze, vec![N2 | N4 | N6], 0, 0);

        // Build the network and adjacency map
        let network = &maze.nets[0];
        let adjacency = maze.build_adjacency_map(network);

        println!("Network nodes: {:?}", network);
        println!("Adjacency map:");
        for (node, neighbors) in &adjacency {
            println!(
                "  Node({}, {}, {}) -> {:?}",
                node.i, node.j, node.node, neighbors
            );
        }

        // Run DFS cycle detection from the first node
        let start_node = *network.iter().next().unwrap();
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();
        let mut path = Vec::new();

        let result = maze.dfs_detect_cycle(
            start_node,
            None,
            &adjacency,
            &mut visited,
            &mut rec_stack,
            &mut path,
        );

        println!("DFS result: {:?}", result);

        // Should NOT detect a cycle since all nodes are on the same tile
        assert!(
            result.is_none(),
            "Expected no cycle for single-tile connections"
        );
    }

  #[test]
  fn test_dfs_detect_cycle_single_tile_one_cycle() {
    // Test DFS cycle detection with nodes all on the same tile
    // Since nodes on the same tile are considered the same node, this should NOT detect a cycle
    let mut maze = Maze::new(2, 2, false, true, true, None);

    place_tile!(maze, vec![N5], 0, 0);
    place_tile!(maze, vec![N5 | N4], 1, 0);
    place_tile!(maze, vec![N5 | N4 | N6], 1, 1);
    place_tile!(maze, vec![N5 | N6], 0, 1);
    place_tile!(maze, vec![N5 | N6], 2, 1);
    place_tile!(maze, vec![N5 | N6], 0, 2);
    place_tile!(maze, vec![N5 | N6], 1, 2);
    place_tile!(maze, vec![N5 | N6], 2, 2);

    // Build the network and adjacency map
    let network = &maze.nets[0];
    let adjacency = maze.build_adjacency_map(network);

    println!("Network nodes: {:?}", network);
    println!("Adjacency map:");
    for (node, neighbors) in &adjacency {
      println!(
        "  Node({}, {}, {}) -> {:?}",
        node.i, node.j, node.node, neighbors
      );
    }

    // Run DFS cycle detection from the first node
    let start_node = *network.iter().next().unwrap();
    let mut visited = HashSet::new();
    let mut rec_stack = HashSet::new();
    let mut path = Vec::new();

    let result = maze.dfs_detect_cycle(
      start_node,
      None,
      &adjacency,
      &mut visited,
      &mut rec_stack,
      &mut path,
    );

    println!("DFS result: {:?}", result);

    // Should NOT detect a cycle since all nodes are on the same tile
    assert!(
      result.is_some(),
      "Expected no cycle for single-tile connections"
    );
  }
}

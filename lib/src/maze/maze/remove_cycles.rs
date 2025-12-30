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
    pub fn detect_cycles(&self) -> Vec<CycleInfo> {
        let mut all_cycles = Vec::new();

        for (net_idx, nodes) in self.nets.iter().enumerate() {
            // Build adjacency map for this network
            let adjacency = self.build_adjacency_map(nodes);

            // Track visited nodes globally and nodes in current DFS path
            let mut visited = HashSet::new();
            let mut rec_stack = HashSet::new();
            let mut path = Vec::new();

            // Try starting DFS from each unvisited node
            for &start_node in nodes.iter() {
                if !visited.contains(&start_node) {
                    if let Some(cycle) = self.dfs_detect_cycle(
                        start_node,
                        None,
                        &adjacency,
                        &mut visited,
                        &mut rec_stack,
                        &mut path,
                    ) {
                        all_cycles.push(CycleInfo {
                            net_index: net_idx,
                            cycle_nodes: cycle,
                        });
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
    /// * `parent` - Parent node (to avoid detecting trivial back-edges)
    /// * `adjacency` - Adjacency map of the graph
    /// * `visited` - Set of all visited nodes
    /// * `rec_stack` - Set of nodes in current recursion stack (path)
    /// * `path` - Current path being explored
    ///
    /// # Returns
    /// Some(Vec<Node>) containing the cycle if found, None otherwise
    fn dfs_detect_cycle(
        &self,
        current: Node,
        parent: Option<Node>,
        adjacency: &HashMap<Node, Vec<Node>>,
        visited: &mut HashSet<Node>,
        rec_stack: &mut HashSet<Node>,
        path: &mut Vec<Node>,
    ) -> Option<Vec<Node>> {
        visited.insert(current);
        rec_stack.insert(current);
        path.push(current);

        if let Some(neighbors) = adjacency.get(&current) {
            for &neighbor in neighbors {
                // Skip the parent to avoid detecting trivial back-edges
                if Some(neighbor) == parent {
                    continue;
                }

                if !visited.contains(&neighbor) {
                    // Recurse to unvisited neighbor
                    if let Some(cycle) = self.dfs_detect_cycle(
                        neighbor,
                        Some(current),
                        adjacency,
                        visited,
                        rec_stack,
                        path,
                    ) {
                        return Some(cycle);
                    }
                } else if rec_stack.contains(&neighbor) {
                    // Found a back edge to a node in current path - this is a cycle!
                    let cycle_start = path.iter().position(|&n| n == neighbor).unwrap();
                    let mut cycle = path[cycle_start..].to_vec();
                    cycle.push(neighbor); // Close the cycle
                    return Some(cycle);
                }
            }
        }

        // Backtrack
        path.pop();
        rec_stack.remove(&current);
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
  use common::consts::{N4, N5, N6};

  #[test]
    fn test_remove_cycles_empty_maze() {
        let mut maze = Maze::new(3, 3, false, true, true, None);
        // Should complete without panicking on an empty maze


    let tile = Tile::new(vec![N5], true);
    maze.update_network(&tile, 0, 0);
    maze.tiles[(0, 0)] = tile;

    let tile = Tile::new(vec![N5|N4], true);
    maze.update_network(&tile, 1, 0);
    maze.tiles[(0, 1)] = tile;
    let tile = Tile::new(vec![N5|N4|N6], true);
    maze.update_network(&tile, 1, 1);
    maze.tiles[(1, 1)] = tile;

    let tile = Tile::new(vec![N5|N6], true);
    maze.update_network(&tile, 0, 1);
    maze.tiles[(1, 0)] = tile;

    let tile = Tile::new(vec![N5|N6], true);
    maze.update_network(&tile, 2, 1);
    maze.tiles[(1, 2)] = tile;

    let tile = Tile::new(vec![N5|N6], true);
    maze.update_network(&tile, 0, 2);
    maze.tiles[(2, 0)] = tile;
    let tile = Tile::new(vec![N5|N6], true);
    maze.update_network(&tile, 1, 2);
    maze.tiles[(2, 1)] = tile;
    let tile = Tile::new(vec![N5|N6], true);
    maze.update_network(&tile, 2, 2);
    maze.tiles[(2, 2)] = tile;

    maze.reduce_nets(0);
    maze.remove_cycles();

    let name = format!("test-cycle");
    let _ = maze.render(name.to_string(), SVG);

    assert_eq!(maze.nets.len(), 1);
    }
}

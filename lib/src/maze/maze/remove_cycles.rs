use crate::maze::maze::Maze;
use crate::maze::net::Node;
use common::consts::NODE_TO_COORD;
use common::types::coordinate::add;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

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

    /// Build adjacency map for nodes in a network.
    /// Each node maps to its connected neighbors within the same network.
    fn build_adjacency_map(&self, nodes: &HashSet<Node>, debug: bool) -> HashMap<Node, Vec<Node>> {
        let start = Instant::now();
        // Build coordinate lookup map for O(1) node lookup
        let coord_map: HashMap<(usize, usize, usize), Node> = nodes
            .iter()
            .map(|&node| ((node.i, node.j, node.node), node))
            .collect();

        let mut adjacency: HashMap<Node, Vec<Node>> = HashMap::new();

        // First pass: build adjacency with forward connections
        // Sort nodes for deterministic iteration order
        let mut sorted_nodes: Vec<&Node> = nodes.iter().collect();
        sorted_nodes.sort_by_key(|n| (n.i, n.j, n.node));

        for &node in sorted_nodes {
            let mut neighbors = Vec::new();

            // 1. Intra-tile connections
            if let Some(tile) = self.tiles.get(node.j, node.i) {
                if tile.initialised {
                    for net_bits in &tile.nodes {
                        let node_mask = 1usize << node.node;
                        if net_bits & node_mask != 0 {
                            for other_node_idx in 0..6 {
                                if other_node_idx != node.node {
                                    let other_mask = 1usize << other_node_idx;
                                    if net_bits & other_mask != 0 {
                                        let other_node = Node::new(node.i, node.j, other_node_idx);
                                        if nodes.contains(&other_node) {
                                            neighbors.push(other_node);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // 2. Inter-tile connections (forward direction)
            let mapping = NODE_TO_COORD[(node.i + (3 * (node.j % 2))) % 6];
            let coord = mapping[node.node];
            if let (Some(adj_i), Some(adj_j)) = (add(node.i, coord.0), add(node.j, coord.1)) {
                if let Some(&adjacent_node) = coord_map.get(&(adj_i, adj_j, node.node)) {
                    if nodes.contains(&adjacent_node) {
                        neighbors.push(adjacent_node);
                    }
                }
            }

            // Sort neighbors for deterministic order
            neighbors.sort_by_key(|n| (n.i, n.j, n.node));
            adjacency.insert(node, neighbors);
        }

        // Second pass: ensure symmetry by adding reverse edges
        let mut symmetric_adjacency = adjacency.clone();
        for (node, neighbors) in adjacency.iter() {
            for &neighbor in neighbors {
                symmetric_adjacency.entry(neighbor).or_default().push(*node);
            }
        }

        // Sort all neighbor lists for deterministic order
        for neighbors in symmetric_adjacency.values_mut() {
            neighbors.sort_by_key(|n| (n.i, n.j, n.node));
        }

        if debug {
            println!(
                "Building adjacency map for {} nodes took: {:?}",
                nodes.len(),
                start.elapsed()
            );
        }
        symmetric_adjacency
    }

    /// Remove a node from an adjacency map, updating all neighbor references.
    /// This is more efficient than rebuilding the entire adjacency map.
    fn remove_node_from_adjacency(adjacency: &mut HashMap<Node, Vec<Node>>, node: Node) {
        // Remove the node itself
        adjacency.remove(&node);

        // Remove references to this node from all neighbors
        for neighbors in adjacency.values_mut() {
            neighbors.retain(|&n| n != node);
        }
    }

    /// Split a network into connected components.
    ///
    /// After removing nodes, a network might become disconnected.
    /// This method finds all connected components and returns them as separate networks.
    fn split_disconnected_networks(
        &self,
        nodes: &HashSet<Node>,
        debug: bool,
    ) -> Vec<HashSet<Node>> {
        let start = Instant::now();
        let mut components = Vec::new();
        let mut visited = HashSet::new();
        let adjacency = self.build_adjacency_map(nodes, debug);

        // Sort nodes for deterministic iteration order
        let mut sorted_nodes: Vec<&Node> = nodes.iter().collect();
        sorted_nodes.sort_by_key(|n| (n.i, n.j, n.node));

        for &start_node in sorted_nodes {
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

        if debug {
            println!(
                "Splitting disconnected networks ({} nodes) took: {:?}",
                nodes.len(),
                start.elapsed()
            );
        }
        components
    }

    pub fn remove_cycles(&mut self, debug: bool) {
        let mut iteration = 0;
        let max_iterations = 100; // Safety limit

        let initial_total_nodes: usize = self.nets.iter().map(|net| net.len()).sum();
        if debug {
            println!(
                "Starting cycle removal: {} networks, {} total nodes",
                self.nets.len(),
                initial_total_nodes
            );
        }

        // Precompute adjacency maps for all networks using optimized O(N) builder
        let adj_start = Instant::now();
        let mut network_adjacencies: Vec<HashMap<Node, Vec<Node>>> = Vec::new();
        for network in self.nets.iter() {
            if network.is_empty() {
                network_adjacencies.push(HashMap::new());
            } else {
                network_adjacencies.push(self.build_adjacency_map(network, debug));
            }
        }
        if debug {
            println!("Building adjacency maps took: {:?}", adj_start.elapsed());
            let non_empty_adj: usize = network_adjacencies
                .iter()
                .filter(|adj| !adj.is_empty())
                .count();
            println!("  Non-empty adjacency maps: {}", non_empty_adj);
        }

        let detection_start = Instant::now();
        loop {
            iteration += 1;
            if debug {
                println!("Cycle removal iteration {}", iteration);
            }
            if iteration > max_iterations {
                println!("Warning: Reached maximum iterations");
                break;
            }

            let cycle_start = Instant::now();
            let mut cycles = self.detect_cycles_with_adjacency(&network_adjacencies, debug);
            if debug {
                if iteration == 1 {
                    println!("First cycle detection took: {:?}", cycle_start.elapsed());
                }
                println!("  Found {} cycles", cycles.len());
            }
            if cycles.is_empty() {
                if debug {
                    println!(
                        "Total cycle detection time: {:?}",
                        detection_start.elapsed()
                    );
                }
                break;
            }

            // Sort cycles for deterministic processing
            cycles.sort_by(|a, b| {
                // First by network index
                a.net_index
                    .cmp(&b.net_index)
                    // Then by cycle length
                    .then(a.cycle_nodes.len().cmp(&b.cycle_nodes.len()))
                    // Then by the nodes themselves for tie-breaking
                    .then_with(|| {
                        for (node_a, node_b) in a.cycle_nodes.iter().zip(b.cycle_nodes.iter()) {
                            let cmp = (node_a.i, node_a.j, node_a.node).cmp(&(
                                node_b.i,
                                node_b.j,
                                node_b.node,
                            ));
                            if cmp != std::cmp::Ordering::Equal {
                                return cmp;
                            }
                        }
                        a.cycle_nodes.len().cmp(&b.cycle_nodes.len())
                    })
            });

            // Break all cycles found in this iteration
            let removal_start = Instant::now();
            let mut cycles_removed = 0;

            // Collect unique nodes to remove per network to avoid duplicate removals
            let mut nodes_to_remove_per_net: HashMap<usize, HashSet<Node>> = HashMap::new();

            for cycle_info in &cycles {
                if cycle_info.cycle_nodes.len() >= 2 {
                    // Remove the second-to-last node in the cycle (edge that closes the cycle)
                    let node_to_remove = cycle_info.cycle_nodes[cycle_info.cycle_nodes.len() - 2];

                    nodes_to_remove_per_net
                        .entry(cycle_info.net_index)
                        .or_default()
                        .insert(node_to_remove);

                    cycles_removed += 1;

                    // Log which cycle and node we're removing
                    println!(
                        "[Cycle removal] Network {}: removing node {:?} from cycle of length {}",
                        cycle_info.net_index,
                        node_to_remove,
                        cycle_info.cycle_nodes.len()
                    );
                    println!("[Cycle removal] Full cycle: {:?}", cycle_info.cycle_nodes);
                }
            }

            // Actually remove nodes from each network
            // Sort by network index for deterministic processing
            let mut sorted_entries: Vec<(usize, HashSet<Node>)> =
                nodes_to_remove_per_net.into_iter().collect();
            sorted_entries.sort_by_key(|(net_idx, _)| *net_idx);

            for (net_index, nodes_to_remove) in sorted_entries {
                if net_index < self.nets.len() {
                    if debug {
                        println!(
                            "  Removing {} nodes from network {}",
                            nodes_to_remove.len(),
                            net_index
                        );
                    }
                    println!(
                        "X[Cycle removal] Network {}: removing {} nodes: {:?}",
                        net_index,
                        nodes_to_remove.len(),
                        nodes_to_remove
                    );

                    // Remove nodes from network
                    let mut sorted_nodes: Vec<&Node> = nodes_to_remove.iter().collect();
                    sorted_nodes.sort_by_key(|n| (n.i, n.j, n.node));
                    for node in &sorted_nodes {
                        self.nets[net_index].remove(node);
                    }

                    // Update adjacency maps for this network
                    if let Some(adjacency) = network_adjacencies.get_mut(net_index) {
                        for node in &sorted_nodes {
                            Self::remove_node_from_adjacency(adjacency, **node);
                        }
                    }

                    // Update tiles for each removed node
                    for node in &sorted_nodes {
                        if let Some(tile) = self.tiles.get_mut(node.j, node.i) {
                            if tile.initialised {
                                let node_mask = 1usize << node.node;
                                // Find the subnet that contains this node and remove it
                                tile.nodes.iter_mut().for_each(|net_bits| {
                                    if *net_bits & node_mask != 0 {
                                        *net_bits &= !node_mask;
                                    }
                                });
                                // Add the node as its own separate subnet
                                tile.nodes.push(node_mask);
                            }
                        }
                    }

                    if debug {
                        let current_size = self.nets[net_index].len();
                        println!("  Network {} now has {} nodes", net_index, current_size);
                    }
                }
            }

            if debug && cycles_removed > 0 {
                println!("  Removed {} cycles in this iteration", cycles_removed);
                if iteration == 1 {
                    println!(
                        "First cycle removal batch took: {:?}",
                        removal_start.elapsed()
                    );
                }
            }
        }

        // After removing all cycles, split any disconnected networks
        let split_start = Instant::now();
        let mut new_nets = Vec::new();
        for network in self.nets.iter() {
            let components = self.split_disconnected_networks(network, debug);
            new_nets.extend(components);
        }
        self.nets = new_nets;
        if debug {
            println!(
                "Splitting disconnected networks took: {:?}",
                split_start.elapsed()
            );
            let final_nodes: usize = self.nets.iter().map(|net| net.len()).sum();
            println!(
                "Cycle removal completed: {} networks, {} total nodes (removed {} nodes)",
                self.nets.len(),
                final_nodes,
                initial_total_nodes - final_nodes
            );
        }
    }

    /// Detect cycles using precomputed adjacency maps.
    /// This is faster than detect_cycles() which rebuilds adjacency maps each time.
    fn detect_cycles_with_adjacency(
        &self,
        adjacencies: &[HashMap<Node, Vec<Node>>],
        debug: bool,
    ) -> Vec<CycleInfo> {
        let start = Instant::now();
        let mut all_cycles = Vec::new();
        let mut seen_cycles: HashSet<Vec<Node>> = HashSet::new();

        for (net_idx, nodes) in self.nets.iter().enumerate() {
            if nodes.is_empty() {
                continue;
            }

            let adjacency = &adjacencies[net_idx];

            // Track visited nodes globally and nodes in current DFS path
            // Nodes in the same internal net on a tile are marked together as visited
            let mut visited: HashSet<Node> = HashSet::new();
            let mut rec_stack: HashSet<Node> = HashSet::new();
            let mut path = Vec::new();

            // Count cycles found in this network
            let mut network_cycle_count = 0;

            // Try starting DFS from each unvisited node
            // Sort nodes for deterministic iteration order
            let mut sorted_nodes: Vec<&Node> = nodes.iter().collect();
            sorted_nodes.sort_by_key(|n| (n.i, n.j, n.node));

            for &start_node in sorted_nodes {
                if !visited.contains(&start_node) {
                    if let Some(cycle) = self.dfs_detect_cycle_with_adjacency(
                        start_node,
                        None,
                        adjacency,
                        &mut visited,
                        &mut rec_stack,
                        &mut path,
                    ) {
                        // Normalize the cycle to check for duplicates
                        let normalized = Self::normalize_cycle(&cycle);

                        // Only add if we haven't seen this cycle before
                        if !seen_cycles.contains(&normalized) {
                            seen_cycles.insert(normalized.clone());
                            all_cycles.push(CycleInfo {
                                net_index: net_idx,
                                cycle_nodes: cycle.clone(),
                            });
                            network_cycle_count += 1;
                            // Log cycle even when debug=false to understand non-determinism
                            println!(
                                "[Cycle detection] Network {}: found cycle of length {}: {:?}",
                                net_idx,
                                cycle.len(),
                                normalized
                            );
                        } else {
                            println!(
                                "[Cycle detection] Network {}: skipped duplicate cycle: {:?}",
                                net_idx, normalized
                            );
                        }
                        // Else: skip this duplicate cycle
                    }
                }
            }

            if debug && network_cycle_count > 0 {
                println!(
                    "  Network {}: found {} cycles ({} nodes)",
                    net_idx,
                    network_cycle_count,
                    nodes.len()
                );
            }
        }

        if debug {
            println!("Cycle detection function took: {:?}", start.elapsed());
            println!("Total cycles found: {}", all_cycles.len());
        } else {
            println!("[Cycle detection] Total cycles found: {}", all_cycles.len());
        }
        all_cycles
    }

    /// DFS cycle detection using precomputed adjacency map.
    fn dfs_detect_cycle_with_adjacency(
        &self,
        current: Node,
        parent_tile: Option<(usize, usize)>,
        adjacency: &HashMap<Node, Vec<Node>>,
        visited: &mut HashSet<Node>,
        rec_stack: &mut HashSet<Node>,
        path: &mut Vec<Node>,
    ) -> Option<Vec<Node>> {
        let current_tile = (current.i, current.j);

        // Find all nodes on this tile that are reachable from the entry node
        // via intra-tile connections (i.e., in the same net within this tile)
        let mut reachable_on_tile: HashSet<Node> = HashSet::new();
        let mut to_visit: Vec<Node> = vec![current];
        while let Some(node) = to_visit.pop() {
            if reachable_on_tile.contains(&node) {
                continue;
            }
            reachable_on_tile.insert(node);

            // Find intra-tile neighbors (same tile, connected via adjacency)
            if let Some(neighbors) = adjacency.get(&node) {
                for &neighbor in neighbors {
                    if neighbor.i == current.i && neighbor.j == current.j {
                        // Same tile - this is an intra-tile connection
                        if !reachable_on_tile.contains(&neighbor) {
                            to_visit.push(neighbor);
                        }
                    }
                }
            }
        }

        // Mark all reachable nodes on this tile as visited and in rec_stack
        // Sort reachable nodes for deterministic iteration
        let mut reachable_sorted: Vec<&Node> = reachable_on_tile.iter().collect();
        reachable_sorted.sort_by_key(|n| (n.i, n.j, n.node));
        for &node in reachable_sorted {
            visited.insert(node);
            rec_stack.insert(node);
        }
        path.push(current);

        // Collect inter-tile neighbors from all reachable nodes on this tile
        let mut inter_tile_neighbors: Vec<Node> = Vec::new();
        // Sort reachable nodes for deterministic iteration
        let mut reachable_sorted: Vec<&Node> = reachable_on_tile.iter().collect();
        reachable_sorted.sort_by_key(|n| (n.i, n.j, n.node));
        for &node in reachable_sorted {
            if let Some(neighbors) = adjacency.get(&node) {
                for &neighbor in neighbors {
                    let neighbor_tile = (neighbor.i, neighbor.j);
                    // Only collect inter-tile neighbors (different tile)
                    if neighbor_tile != current_tile {
                        inter_tile_neighbors.push(neighbor);
                    }
                }
            }
        }

        for neighbor in inter_tile_neighbors {
            let neighbor_tile = (neighbor.i, neighbor.j);

            // Skip the parent tile to avoid detecting trivial back-edges
            if Some(neighbor_tile) == parent_tile {
                continue;
            }

            if !visited.contains(&neighbor) {
                // Recurse to unvisited neighbor
                if let Some(cycle) = self.dfs_detect_cycle_with_adjacency(
                    neighbor,
                    Some(current_tile),
                    adjacency,
                    visited,
                    rec_stack,
                    path,
                ) {
                    // if debug {
                    //     println!("DFS recursion found cycle of length {}", cycle.len());
                    // }
                    return Some(cycle);
                }
            } else if rec_stack.contains(&neighbor) {
                // Found a back edge to a node in current path - this is a cycle!
                // Find the position in path where this node (or a connected node on same tile) first appears
                let cycle_start = path
                    .iter()
                    .position(|n| n.i == neighbor.i && n.j == neighbor.j)
                    .unwrap();
                let mut cycle = path[cycle_start..].to_vec();
                cycle.push(neighbor); // Close the cycle
                                      // if debug {
                                      //     println!("DFS found back-edge cycle of length {} ({cycle_start})", cycle.len());
                                      // }
                return Some(cycle);
            }
        }

        // Backtrack - remove all reachable nodes from rec_stack
        path.pop();
        // Sort reachable nodes for deterministic iteration
        let mut reachable_sorted: Vec<&Node> = reachable_on_tile.iter().collect();
        reachable_sorted.sort_by_key(|n| (n.i, n.j, n.node));
        for &node in reachable_sorted {
            rec_stack.remove(&node);
        }
        None
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

        println!("Before reduce_nets: {} networks", maze.nets.len());
        maze.reduce_nets(0);
        println!("After reduce_nets: {} networks", maze.nets.len());
        maze.remove_cycles(false);

        let name = "test-cycle-1".to_string();
        let _ = maze.render(name.to_string(), SVG);

        println!("Final nets: {} networks", maze.nets.len());
        for (i, net) in maze.nets.iter().enumerate() {
            println!("  Network {}: {} nodes", i, net.len());
        }
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
        maze.remove_cycles(false);

        let name = "test-cycle-2".to_string();
        let _ = maze.render(name.to_string(), SVG);

        // Note the nets can be correct, bit the SVG isn't?
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
        maze.remove_cycles(false);

        // No cycles should be detected - intra-tile connections are not cycles
        assert_eq!(maze.nets.len(), 1);
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

        let name = "test-two-cycles-1".to_string();
        let _ = maze.render(name.to_string(), SVG);

        maze.reduce_nets(0);
        maze.remove_cycles(false);

        let name = "test-two-cycles-2".to_string();
        let _ = maze.render(name.to_string(), SVG);

        // After cycle removal, we should have at least one network
        assert!(!maze.nets.is_empty());
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

        let name = "test-two-cycles-overlapping-1".to_string();
        let _ = maze.render(name.to_string(), SVG);

        maze.reduce_nets(0);
        maze.remove_cycles(false);

        let name = "test-two-cycles-overlapping-2".to_string();
        let _ = maze.render(name.to_string(), SVG);

        // After cycle removal, we should have at least one network
        assert!(maze.nets.len() == 1);
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

        let name = "test-five-cycles-1".to_string();
        let _ = maze.render(name.to_string(), SVG);

        maze.reduce_nets(0);
        maze.remove_cycles(false);

        let name = "test-five-cycles-2".to_string();
        let _ = maze.render(name.to_string(), SVG);

        // After cycle removal, we should have at least one network
        assert!(maze.nets.len() == 1);
    }
}

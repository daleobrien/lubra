use crate::maze::maze::Maze;
use common::consts::NODE_TO_COORD;
use common::types::coordinate::add;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::collections::{HashMap, HashSet};

/// Selects a random element from a vector using the provided seed.
fn select_random<T: Clone>(vec: Vec<T>, seed: Option<u64>) -> Option<T> {
    if vec.is_empty() {
        return None;
    }

    let mut rng = match seed {
        Some(s) => ChaCha8Rng::seed_from_u64(s),
        None => ChaCha8Rng::from_rng(thread_rng()).unwrap(),
    };

    let index = rng.gen_range(0..vec.len());
    Some(vec[index].clone())
}

/// Methods for finding surrounding coordinates between nets.
impl Maze {
    /// Gets all surrounding tile coordinates for a single net.
    ///
    /// A "surrounding coordinate" is a tile that is adjacent to at least one tile
    /// in the net, but is not itself part of the net.
    ///
    /// # Arguments
    /// * `net_index` - The index of the net in `self.nets`
    ///
    /// # Returns
    /// A `HashSet` of (i, j) coordinates that surround the net
    fn get_surrounding_coordinates_for_net(&self, net_index: usize) -> HashSet<(usize, usize)> {
        let net = &self.nets[net_index];

        // First, collect all tile coordinates that are part of this net
        let net_tiles: HashSet<(usize, usize)> = net.iter().map(|node| (node.i, node.j)).collect();

        let mut surrounding: HashSet<(usize, usize)> = HashSet::new();

        // For each tile in the net, find its adjacent tiles
        for (i, j) in &net_tiles {
            let k = (*i + (3 * (*j % 2))) % 6;
            let coords = NODE_TO_COORD[k];

            // Check all 6 adjacent tile positions (first 6 entries in NODE_TO_COORD)
            // NODE_TO_COORD has 12 entries: first 6 are adjacent tiles, last 6 are diagonal/extended
            for node_idx in 0..6 {
                let coord = coords[node_idx];
                let x = add(*i, coord.0);
                let y = add(*j, coord.1);

                if let (Some(x), Some(y)) = (x, y) {
                    // Check bounds
                    if x < self.number_of_columns && y < self.number_of_rows {
                        // Only include if not already part of the net
                        if !net_tiles.contains(&(x, y)) {
                            surrounding.insert((x, y));
                        }
                    }
                }
            }
        }

        surrounding
    }

    /// Finds all surrounding coordinates for each net and returns them as a map.
    ///
    /// # Returns
    /// A `HashMap` mapping net index to the set of surrounding coordinates
    pub fn get_all_surrounding_coordinates(&self) -> HashMap<usize, HashSet<(usize, usize)>> {
        let mut result: HashMap<usize, HashSet<(usize, usize)>> = HashMap::new();

        for net_index in 0..self.nets.len() {
            if !self.nets[net_index].is_empty() {
                let surrounding = self.get_surrounding_coordinates_for_net(net_index);
                result.insert(net_index, surrounding);
            }
        }

        result
    }

    /// Gets all tile coordinates that are part of any net.
    fn get_all_net_tiles(&self) -> HashSet<(usize, usize)> {
        self.nets
            .iter()
            .flat_map(|net| net.iter().map(|node| (node.i, node.j)))
            .collect()
    }

    /// Finds common surrounding coordinates between two nets.
    ///
    /// These are coordinates that are adjacent to both nets but not part of any net.
    /// Such coordinates are candidates for connecting disjoint nets.
    ///
    /// # Arguments
    /// * `net_a` - Index of the first net
    /// * `net_b` - Index of the second net
    ///
    /// # Returns
    /// A `HashSet` of (i, j) coordinates that surround both nets but are not part of any net
    pub fn find_common_surrounding_coordinates(
        &self,
        net_a: usize,
        net_b: usize,
    ) -> HashSet<(usize, usize)> {
        let surrounding_a = self.get_surrounding_coordinates_for_net(net_a);
        let surrounding_b = self.get_surrounding_coordinates_for_net(net_b);

        // Get tiles that are part of any net
        let all_net_tiles = self.get_all_net_tiles();

        // Find intersection and exclude any tiles that are part of any net
        surrounding_a
            .intersection(&surrounding_b)
            .filter(|coord| !all_net_tiles.contains(coord))
            .cloned()
            .collect()
    }

    /// Finds all pairs of nets that share common surrounding coordinates.
    ///
    /// This identifies disjoint nets that could potentially be connected
    /// by modifying a tile that surrounds both of them.
    ///
    /// # Returns
    /// A `HashMap` mapping pairs of net indices (a, b) where a < b to the set of
    /// common surrounding coordinates between them (excluding tiles that are part of any net)
    pub fn find_all_common_surrounding_coordinates(
        &self,
    ) -> HashMap<(usize, usize), HashSet<(usize, usize)>> {
        let all_surrounding = self.get_all_surrounding_coordinates();
        let all_net_tiles = self.get_all_net_tiles();
        let mut result: HashMap<(usize, usize), HashSet<(usize, usize)>> = HashMap::new();

        // Sort net indices to ensure consistent ordering (a < b)
        let mut net_indices: Vec<usize> = all_surrounding.keys().cloned().collect();
        net_indices.sort();

        for (idx_a, &net_a) in net_indices.iter().enumerate() {
            for &net_b in net_indices.iter().skip(idx_a + 1) {
                let surrounding_a = &all_surrounding[&net_a];
                let surrounding_b = &all_surrounding[&net_b];

                // Find intersection and exclude any tiles that are part of any net
                let common: HashSet<(usize, usize)> = surrounding_a
                    .intersection(surrounding_b)
                    .filter(|coord| !all_net_tiles.contains(coord))
                    .cloned()
                    .collect();

                if !common.is_empty() {
                    // net_a < net_b is guaranteed by sorted indices and skip
                    result.insert((net_a, net_b), common);
                }
            }
        }

        result
    }

    /// Finds a pair of nets with common surrounding coordinates and selects one coordinate.
    ///
    /// This function finds all pairs of nets that share surrounding coordinates,
    /// then selects one pair and one coordinate from their common surroundings.
    ///
    /// # Returns
    /// `Some((net_a, net_b, (i, j)))` if a pair with common surroundings exists,
    /// `None` if no such pair exists
    pub fn select_common_surrounding_coordinate(
        &self,
    ) -> Option<(usize, usize, (usize, usize))> {
        let all_common = self.find_all_common_surrounding_coordinates();

        if all_common.is_empty() {
            return None;
        }

        // Select a random pair from all pairs with common surroundings
        let pairs: Vec<(usize, usize)> = all_common.keys().cloned().collect();
        let selected_pair = select_random(pairs, self.seed)?;

        // Select a random coordinate from the common surroundings
        let common_coords: Vec<(usize, usize)> =
            all_common[&selected_pair].iter().cloned().collect();
        let selected_coord = select_random(common_coords, self.seed)?;

        Some((selected_pair.0, selected_pair.1, selected_coord))
    }

    /// Finds common surrounding coordinates between a specific pair of nets
    /// and selects one of them.
    ///
    /// # Arguments
    /// * `net_a` - Index of the first net
    /// * `net_b` - Index of the second net
    ///
    /// # Returns
    /// `Some((i, j))` if common surrounding coordinates exist,
    /// `None` if the nets have no common surrounding coordinates
    pub fn select_common_surrounding_for_pair(
        &self,
        net_a: usize,
        net_b: usize,
    ) -> Option<(usize, usize)> {
        let common = self.find_common_surrounding_coordinates(net_a, net_b);

        if common.is_empty() {
            return None;
        }

        let coords: Vec<(usize, usize)> = common.iter().cloned().collect();
        select_random(coords, self.seed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::maze::net::{Net, Node, Nodes};
    use crate::maze::tile::Tile;
    use array2d::Array2D;

    /// Helper function to create a test maze with given nets
    fn create_test_maze_with_nets(nets: Net, cols: usize, rows: usize) -> Maze {
        let tiles = Array2D::filled_with(Tile::new(vec![], false), rows, cols);
        Maze {
            style: None,
            nets,
            width: 0.0,
            height: 0.0,
            seed: Some(42),
            number_of_columns: cols,
            number_of_rows: rows,
            tiles,
            boundaries: false,
            show_node_numbers: false,
        }
    }

    #[test]
    fn test_get_surrounding_coordinates_single_tile_net() {
        // Create a net with a single tile at (2, 2)
        let mut net0 = Nodes::new();
        net0.insert(Node::new(2, 2, 0));

        let maze = create_test_maze_with_nets(vec![net0], 5, 5);
        let surrounding = maze.get_surrounding_coordinates_for_net(0);

        // The tile at (2, 2) should have surrounding tiles
        assert!(!surrounding.is_empty());
        // The tile itself should not be in the surrounding set
        assert!(!surrounding.contains(&(2, 2)));
    }

    #[test]
    fn test_get_surrounding_coordinates_excludes_net_tiles() {
        // Create a net spanning two adjacent tiles
        let mut net0 = Nodes::new();
        net0.insert(Node::new(2, 2, 0));
        net0.insert(Node::new(3, 2, 0));

        let maze = create_test_maze_with_nets(vec![net0], 6, 6);
        let surrounding = maze.get_surrounding_coordinates_for_net(0);

        // Neither tile in the net should be in the surrounding set
        assert!(!surrounding.contains(&(2, 2)));
        assert!(!surrounding.contains(&(3, 2)));
    }

    #[test]
    fn test_find_common_surrounding_coordinates_disjoint_nets() {
        // Create two disjoint nets that might share surrounding coordinates
        let mut net0 = Nodes::new();
        net0.insert(Node::new(1, 2, 0));

        let mut net1 = Nodes::new();
        net1.insert(Node::new(3, 2, 0));

        let maze = create_test_maze_with_nets(vec![net0, net1], 6, 6);
        let common = maze.find_common_surrounding_coordinates(0, 1);

        // Tiles between the two nets might be common surroundings
        // The specific result depends on hex grid geometry
        println!("Common surrounding coordinates: {:?}", common);
    }

    #[test]
    fn test_find_common_surrounding_coordinates_no_overlap() {
        // Create two nets far apart with no common surroundings
        let mut net0 = Nodes::new();
        net0.insert(Node::new(0, 0, 0));

        let mut net1 = Nodes::new();
        net1.insert(Node::new(9, 9, 0));

        let maze = create_test_maze_with_nets(vec![net0, net1], 10, 10);
        let common = maze.find_common_surrounding_coordinates(0, 1);

        // These nets are too far apart to share surrounding coordinates
        assert!(common.is_empty());
    }

    #[test]
    fn test_find_all_common_surrounding_coordinates() {
        // Create three nets where some pairs might share surroundings
        let mut net0 = Nodes::new();
        net0.insert(Node::new(1, 2, 0));

        let mut net1 = Nodes::new();
        net1.insert(Node::new(3, 2, 0));

        let mut net2 = Nodes::new();
        net2.insert(Node::new(8, 8, 0));

        let maze = create_test_maze_with_nets(vec![net0, net1, net2], 10, 10);
        let all_common = maze.find_all_common_surrounding_coordinates();

        // Print for debugging
        for ((a, b), coords) in &all_common {
            println!("Nets ({}, {}) share surrounding coords: {:?}", a, b, coords);
        }
    }

    #[test]
    fn test_select_common_surrounding_coordinate() {
        // Create two nearby nets
        let mut net0 = Nodes::new();
        net0.insert(Node::new(2, 2, 0));

        let mut net1 = Nodes::new();
        net1.insert(Node::new(4, 2, 0));

        let maze = create_test_maze_with_nets(vec![net0, net1], 8, 8);
        let result = maze.select_common_surrounding_coordinate();

        if let Some((net_a, net_b, coord)) = result {
            println!(
                "Selected nets ({}, {}) with coordinate {:?}",
                net_a, net_b, coord
            );
            assert!(net_a < net_b);
        }
    }

    #[test]
    fn test_select_common_surrounding_for_pair() {
        // Create two nearby nets
        let mut net0 = Nodes::new();
        net0.insert(Node::new(2, 2, 0));

        let mut net1 = Nodes::new();
        net1.insert(Node::new(4, 2, 0));

        let maze = create_test_maze_with_nets(vec![net0, net1], 8, 8);
        let result = maze.select_common_surrounding_for_pair(0, 1);

        if let Some(coord) = result {
            println!("Selected coordinate: {:?}", coord);
        }
    }

    #[test]
    fn test_empty_nets() {
        let maze = create_test_maze_with_nets(vec![], 5, 5);
        let all_common = maze.find_all_common_surrounding_coordinates();
        assert!(all_common.is_empty());
    }

    #[test]
    fn test_single_net() {
        let mut net0 = Nodes::new();
        net0.insert(Node::new(2, 2, 0));

        let maze = create_test_maze_with_nets(vec![net0], 5, 5);
        let all_common = maze.find_all_common_surrounding_coordinates();

        // With only one net, there can be no pairs
        assert!(all_common.is_empty());
    }

    #[test]
    fn test_common_surrounding_excludes_net_tiles() {
        // Create two adjacent nets - net B is at a position that would be
        // in net A's surrounding coordinates
        let mut net0 = Nodes::new();
        net0.insert(Node::new(2, 2, 0));

        let mut net1 = Nodes::new();
        // Place net1 adjacent to net0 (this tile would normally be in net0's surroundings)
        net1.insert(Node::new(3, 2, 0));

        let maze = create_test_maze_with_nets(vec![net0, net1], 8, 8);

        // Get surrounding coords for net0 - should NOT include (3, 2) since that's net0's own tile
        let surrounding_0 = maze.get_surrounding_coordinates_for_net(0);
        assert!(
            !surrounding_0.contains(&(2, 2)),
            "Net 0's surrounding should not include its own tile (2, 2)"
        );

        // Get surrounding coords for net1 - should NOT include (3, 2) since that's net1's own tile
        let surrounding_1 = maze.get_surrounding_coordinates_for_net(1);
        assert!(
            !surrounding_1.contains(&(3, 2)),
            "Net 1's surrounding should not include its own tile (3, 2)"
        );

        // Common surrounding coordinates should not include tiles from either net
        let common = maze.find_common_surrounding_coordinates(0, 1);
        assert!(
            !common.contains(&(2, 2)),
            "Common surrounding should not include net 0's tile (2, 2)"
        );
        assert!(
            !common.contains(&(3, 2)),
            "Common surrounding should not include net 1's tile (3, 2)"
        );

        println!("Net 0 tiles: (2, 2)");
        println!("Net 1 tiles: (3, 2)");
        println!("Surrounding for net 0: {:?}", surrounding_0);
        println!("Surrounding for net 1: {:?}", surrounding_1);
        println!("Common surrounding (excludes both nets): {:?}", common);
    }

    #[test]
    fn test_find_all_common_excludes_net_tiles() {
        // Create three nets where one is positioned in what would be another's surrounding
        let mut net0 = Nodes::new();
        net0.insert(Node::new(2, 2, 0));

        let mut net1 = Nodes::new();
        net1.insert(Node::new(4, 2, 0));

        // Place net2 at a position that might be in the surrounding of both net0 and net1
        let mut net2 = Nodes::new();
        net2.insert(Node::new(3, 2, 0));

        let maze = create_test_maze_with_nets(vec![net0, net1, net2], 8, 8);
        let all_common = maze.find_all_common_surrounding_coordinates();

        // Common coords between net0 and net1 should NOT include (3, 2) which is net2's tile
        if let Some(common_0_1) = all_common.get(&(0, 1)) {
            assert!(
                !common_0_1.contains(&(3, 2)),
                "Common surrounding for nets 0,1 should not include net 2's tile (3, 2)"
            );
        }

        // No pair should include any tile that belongs to any of the nets
        for ((a, b), coords) in &all_common {
            println!("Pair ({}, {}): {:?}", a, b, coords);
            assert!(
                !coords.contains(&(2, 2)),
                "No common should include net 0's tile"
            );
            assert!(
                !coords.contains(&(4, 2)),
                "No common should include net 1's tile"
            );
            assert!(
                !coords.contains(&(3, 2)),
                "No common should include net 2's tile"
            );
        }
    }
}

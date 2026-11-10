use crate::maze::maze::Maze;
use common::consts::{N1, N2, N3, N4, N5, N6, NODE_TO_COORD};
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

/// Represents the proximity level between two nets.
///
/// Used to indicate how tiles were found when connecting nets:
/// - `Intersection`: Tiles that are part of both nets (closest)
/// - `Adjacent`: Tiles that are part of one net and adjacent to the other
/// - `Surrounding`: Tiles that are adjacent to both nets but not part of either (furthest)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProximityLevel {
    /// Tiles that are part of both nets (same tile coordinates appear in both)
    Intersection,
    /// Tiles that are part of one net and adjacent to (surrounding) the other
    Adjacent,
    /// Tiles that are adjacent to both nets but not part of either
    Surrounding,
}

/// Contains detailed information about a connection point between two nets.
///
/// Includes the net indices, their tile coordinates, the common coordinate,
/// and the proximity level between the nets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetConnectionInfo {
    /// Index of the first net
    pub net_a_index: usize,
    /// Index of the second net
    pub net_b_index: usize,
    /// The common coordinate where the nets can be connected
    pub common_coordinate: (usize, usize),
    /// All tile coordinates that are part of net A
    pub net_a_coordinates: HashSet<(usize, usize)>,
    /// All tile coordinates that are part of net B
    pub net_b_coordinates: HashSet<(usize, usize)>,
    /// The proximity level between the two nets
    pub proximity_level: ProximityLevel,
}

/// Methods for finding surrounding coordinates between nets.
impl Maze {
    /// Gets all tile coordinates that are part of a specific net.
    fn get_tiles_for_net(&self, net_index: usize) -> HashSet<(usize, usize)> {
        self.nets[net_index]
            .iter()
            .map(|node| (node.i, node.j))
            .collect()
    }

    /// Tier 1: Finds tiles that are part of both nets (plain intersection).
    ///
    /// These are tiles where both nets have nodes at the same (i, j) coordinates.
    /// This represents the closest possible connection between two nets.
    ///
    /// # Arguments
    /// * `net_a` - Index of the first net
    /// * `net_b` - Index of the second net
    ///
    /// # Returns
    /// A `HashSet` of (i, j) coordinates that are part of both nets
    fn find_intersection_tiles(
        &self,
        net_a: usize,
        net_b: usize,
    ) -> HashSet<(usize, usize)> {
        let tiles_a = self.get_tiles_for_net(net_a);
        let tiles_b = self.get_tiles_for_net(net_b);
        tiles_a.intersection(&tiles_b).cloned().collect()
    }

    /// Tier 2: Finds tiles that are part of one net and adjacent to the other.
    ///
    /// These are tiles where:
    /// - The tile is part of net A and adjacent to (surrounds) net B, OR
    /// - The tile is part of net B and adjacent to (surrounds) net A
    ///
    /// # Arguments
    /// * `net_a` - Index of the first net
    /// * `net_b` - Index of the second net
    ///
    /// # Returns
    /// A `HashSet` of (i, j) coordinates that are part of one net and adjacent to the other
    fn find_adjacent_to_other_tiles(
        &self,
        net_a: usize,
        net_b: usize,
    ) -> HashSet<(usize, usize)> {
        let tiles_a = self.get_tiles_for_net(net_a);
        let tiles_b = self.get_tiles_for_net(net_b);
        let surrounding_a = self.get_surrounding_coordinates_for_net(net_a);
        let surrounding_b = self.get_surrounding_coordinates_for_net(net_b);

        let mut result = HashSet::new();

        // Tiles in net A that are adjacent to net B (in B's surrounding)
        for tile in &tiles_a {
            if surrounding_b.contains(tile) {
                result.insert(*tile);
            }
        }

        // Tiles in net B that are adjacent to net A (in A's surrounding)
        for tile in &tiles_b {
            if surrounding_a.contains(tile) {
                result.insert(*tile);
            }
        }

        result
    }

    /// Tier 3: Finds tiles that are adjacent to both nets but not part of either.
    ///
    /// These are tiles in the intersection of both nets' surrounding coordinates,
    /// excluding any tiles that are part of any net.
    ///
    /// # Arguments
    /// * `net_a` - Index of the first net
    /// * `net_b` - Index of the second net
    ///
    /// # Returns
    /// A `HashSet` of (i, j) coordinates that surround both nets but are not part of any net
    fn find_surrounding_both_tiles(
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

    /// Finds the closest common coordinates between two nets using a tiered approach.
    ///
    /// This function tries three strategies in order of proximity:
    /// 1. **Intersection**: Tiles that are part of both nets (closest)
    /// 2. **Adjacent**: Tiles that are part of one net and adjacent to the other
    /// 3. **Surrounding**: Tiles that are adjacent to both nets but not part of either (furthest)
    ///
    /// Returns the first non-empty result along with the proximity level.
    ///
    /// # Arguments
    /// * `net_a` - Index of the first net
    /// * `net_b` - Index of the second net
    ///
    /// # Returns
    /// `Some((tiles, level))` if any common tiles exist at any proximity level,
    /// `None` if the nets have no proximity at any level
    pub fn find_closest_common_coordinates(
        &self,
        net_a: usize,
        net_b: usize,
    ) -> Option<(HashSet<(usize, usize)>, ProximityLevel)> {
        // Tier 1: Tiles that are part of both nets
        let intersection = self.find_intersection_tiles(net_a, net_b);
        if !intersection.is_empty() {
            return Some((intersection, ProximityLevel::Intersection));
        }

        // Tier 2: Tiles that are part of one net and adjacent to the other
        let adjacent = self.find_adjacent_to_other_tiles(net_a, net_b);
        if !adjacent.is_empty() {
            return Some((adjacent, ProximityLevel::Adjacent));
        }

        // Tier 3: Tiles that are adjacent to both nets
        let surrounding = self.find_surrounding_both_tiles(net_a, net_b);
        if !surrounding.is_empty() {
            return Some((surrounding, ProximityLevel::Surrounding));
        }

        None
    }

    /// Finds common surrounding coordinates between two nets.
    ///
    /// Uses a tiered approach to find the closest common tiles:
    /// 1. Tiles that are part of both nets (intersection)
    /// 2. Tiles that are part of one net and adjacent to the other
    /// 3. Tiles that are adjacent to both nets but not part of either
    ///
    /// Returns only the tiles (without proximity level). For the proximity level,
    /// use `find_closest_common_coordinates` instead.
    ///
    /// # Arguments
    /// * `net_a` - Index of the first net
    /// * `net_b` - Index of the second net
    ///
    /// # Returns
    /// A `HashSet` of (i, j) coordinates representing the closest common tiles
    pub fn find_common_surrounding_coordinates(
        &self,
        net_a: usize,
        net_b: usize,
    ) -> HashSet<(usize, usize)> {
        self.find_closest_common_coordinates(net_a, net_b)
            .map(|(tiles, _)| tiles)
            .unwrap_or_default()
    }

    /// Finds all pairs of nets with their closest common coordinates and proximity levels.
    ///
    /// Uses the tiered approach for each pair of nets to find the closest connection.
    ///
    /// # Returns
    /// A `HashMap` mapping pairs of net indices (a, b) where a < b to a tuple of
    /// (coordinates, proximity_level)
    pub fn find_all_closest_common_coordinates(
        &self,
    ) -> HashMap<(usize, usize), (HashSet<(usize, usize)>, ProximityLevel)> {
        let mut result: HashMap<(usize, usize), (HashSet<(usize, usize)>, ProximityLevel)> =
            HashMap::new();

        // Get all non-empty net indices
        let mut net_indices: Vec<usize> = (0..self.nets.len())
            .filter(|&i| !self.nets[i].is_empty())
            .collect();
        net_indices.sort();

        for (idx_a, &net_a) in net_indices.iter().enumerate() {
            for &net_b in net_indices.iter().skip(idx_a + 1) {
                if let Some((tiles, level)) = self.find_closest_common_coordinates(net_a, net_b) {
                    result.insert((net_a, net_b), (tiles, level));
                }
            }
        }

        result
    }

    /// Finds all pairs of nets that share common surrounding coordinates.
    ///
    /// Uses the tiered approach to find the closest connection for each pair.
    /// This identifies nets that could potentially be connected.
    ///
    /// # Returns
    /// A `HashMap` mapping pairs of net indices (a, b) where a < b to the set of
    /// closest common coordinates between them
    pub fn find_all_common_surrounding_coordinates(
        &self,
    ) -> HashMap<(usize, usize), HashSet<(usize, usize)>> {
        self.find_all_closest_common_coordinates()
            .into_iter()
            .map(|(pair, (tiles, _))| (pair, tiles))
            .collect()
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

    /// Finds a pair of nets with common surrounding coordinates and returns detailed information.
    ///
    /// This function extends `select_common_surrounding_coordinate` by also returning:
    /// - All coordinates that are part of net A
    /// - All coordinates that are part of net B
    /// - The proximity level between the nets
    ///
    /// # Returns
    /// `Some(NetConnectionInfo)` containing detailed information about the connection,
    /// `None` if no pair with common surroundings exists
    ///
    /// # Examples
    /// ```ignore
    /// if let Some(info) = maze.select_common_surrounding_coordinate_with_info() {
    ///     println!("Connecting nets {} and {}", info.net_a_index, info.net_b_index);
    ///     println!("  Common coordinate: {:?}", info.common_coordinate);
    ///     println!("  Net A has {} tiles", info.net_a_coordinates.len());
    ///     println!("  Net B has {} tiles", info.net_b_coordinates.len());
    ///     println!("  Proximity: {:?}", info.proximity_level);
    /// }
    /// ```
    pub fn select_common_surrounding_coordinate_with_info(&self) -> Option<NetConnectionInfo> {
        let all_closest = self.find_all_closest_common_coordinates();

        if all_closest.is_empty() {
            return None;
        }

        // Select a random pair from all pairs with common surroundings
        let pairs: Vec<(usize, usize)> = all_closest.keys().cloned().collect();
        let selected_pair = select_random(pairs, self.seed)?;

        // Get the common coordinates and proximity level for the selected pair
        let (common_coords, proximity_level) = &all_closest[&selected_pair];

        // Select a random coordinate from the common surroundings
        let coords_vec: Vec<(usize, usize)> = common_coords.iter().cloned().collect();
        let selected_coord = select_random(coords_vec, self.seed)?;

        // Get all tile coordinates for each net
        let net_a_coordinates = self.get_tiles_for_net(selected_pair.0);
        let net_b_coordinates = self.get_tiles_for_net(selected_pair.1);

        Some(NetConnectionInfo {
            net_a_index: selected_pair.0,
            net_b_index: selected_pair.1,
            common_coordinate: selected_coord,
            net_a_coordinates,
            net_b_coordinates,
            proximity_level: *proximity_level,
        })
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

    /// Finds pairs of adjacent tiles between two nets.
    ///
    /// Returns a vector of coordinate pairs `((i1, j1), (i2, j2))` where:
    /// - `(i1, j1)` is a tile coordinate in net A
    /// - `(i2, j2)` is a tile coordinate in net B
    /// - The two tiles are adjacent to each other
    ///
    /// This represents the closest possible connections between two nets
    /// where tiles from each net are directly adjacent.
    ///
    /// # Arguments
    /// * `net_a` - Index of the first net
    /// * `net_b` - Index of the second net
    /// * `find_one` - If `true`, returns immediately when the first adjacent pair is found
    ///
    /// # Returns
    /// A vector of adjacent tile pairs between the two nets.
    /// If `find_one` is `true`, returns a vector with at most one pair.
    pub fn find_adjacent_tile_pairs(
        &self,
        net_a: usize,
        net_b: usize,
        find_one: bool,
    ) -> Vec<((usize, usize), (usize, usize))> {
        let tiles_a = self.get_tiles_for_net(net_a);
        let tiles_b = self.get_tiles_for_net(net_b);
        let mut pairs = Vec::new();

        // Always iterate the smaller net first for better performance
        let (swapped, source_tiles, target_tiles) = if tiles_a.len() > tiles_b.len() {
            (true, &tiles_b, &tiles_a)
        } else {
            (false, &tiles_a, &tiles_b)
        };

        // For each tile in source net, check if any tile in target net is adjacent
        for &(i_src, j_src) in source_tiles {
            let k = (i_src + (3 * (j_src % 2))) % 6;
            let coords = NODE_TO_COORD[k];

            // Check all 6 adjacent positions
            for node_idx in 0..6 {
                let coord = coords[node_idx];
                if let (Some(x), Some(y)) = (add(i_src, coord.0), add(j_src, coord.1)) {
                    // Check bounds
                    if x < self.number_of_columns && y < self.number_of_rows {
                        if target_tiles.contains(&(x, y)) {
                            // Return the pair in the original net order (a, b)
                            if swapped {
                                pairs.push(((x, y), (i_src, j_src)));
                            } else {
                                pairs.push(((i_src, j_src), (x, y)));
                            }
                            if find_one {
                                return pairs;
                            }
                        }
                    }
                }
            }
        }

        pairs
    }

    /// Finds one adjacent tile pair between two nets, if any exists.
    ///
    /// This is a convenience method that calls `find_adjacent_tile_pairs(net_a, net_b, true)`
    /// and returns the result as an `Option`.
    ///
    /// # Arguments
    /// * `net_a` - Index of the first net
    /// * `net_b` - Index of the second net
    ///
    /// # Returns
    /// `Some(((i1, j1), (i2, j2)))` if an adjacent tile pair exists,
    /// `None` if no adjacent tiles exist between the nets
    pub fn find_one_adjacent_tile_pair(
        &self,
        net_a: usize,
        net_b: usize,
    ) -> Option<((usize, usize), (usize, usize))> {
        let pairs = self.find_adjacent_tile_pairs(net_a, net_b, true);
        pairs.first().cloned()
    }

    /// Returns the connecting node mask needed to join two adjacent tiles.
    ///
    /// Given two adjacent tiles at coordinates `(i1, j1)` and `(i2, j2)`,
    /// this function determines which node index they share (i.e., which
    /// side of the hexagon they are adjacent on), and returns the bit mask
    /// that needs to be applied to both tiles to connect them.
    ///
    /// # Arguments
    /// * `(i1, j1)` - Coordinates of the first tile
    /// * `(i2, j2)` - Coordinates of the second tile (must be adjacent to first)
    ///
    /// # Returns
    /// `Some((node_index, mask))` where:
    /// - `node_index` is the node index (0-5 for N1-N6) that connects the tiles
    /// - `mask` is the bit mask (e.g., N1, N2, etc.) to apply to both tiles
    /// Returns `None` if the tiles are not adjacent or cannot be determined
    ///
    pub fn get_connecting_node_mask(
        &self,
        tile_a: (usize, usize),
        tile_b: (usize, usize),
    ) -> Option<usize> {
        let (i1, j1) = tile_a;
        let (i2, j2) = tile_b;

        // Determine which node index connects tile_a to tile_b
        let k = (i1 + (3 * (j1 % 2))) % 6;
        let coords = NODE_TO_COORD[k];

        // Check each of the 6 adjacent positions from tile_a
        for node_idx in 0..6 {
            let coord = coords[node_idx];
            if let (Some(x), Some(y)) = (add(i1, coord.0), add(j1, coord.1)) {
                if x == i2 && y == j2 {
                    // Found the node index that connects tile_a to tile_b
                    let mask = match node_idx {
                        0 => N1,
                        1 => N2,
                        2 => N3,
                        3 => N4,
                        4 => N5,
                        5 => N6,
                        _ => unreachable!(),
                    };
                    return Some(mask);
                }
            }
        }

        // Try the reverse direction (tile_b to tile_a)
        let k2 = (i2 + (3 * (j2 % 2))) % 6;
        let coords2 = NODE_TO_COORD[k2];

        for node_idx in 0..6 {
            let coord = coords2[node_idx];
            if let (Some(x), Some(y)) = (add(i2, coord.0), add(j2, coord.1)) {
                if x == i1 && y == j1 {
                    // Found the node index that connects tile_b to tile_a
                    let mask = match node_idx {
                        0 => N1,
                        1 => N2,
                        2 => N3,
                        3 => N4,
                        4 => N5,
                        5 => N6,
                        _ => unreachable!(),
                    };
                    return Some(mask);
                }
            }
        }

        None
    }

    /// Returns connecting node masks for all adjacent tile pairs between two nets.
    ///
    /// For each pair of adjacent tiles found between `net_a` and `net_b`,
    /// this function returns the node index and bit mask needed to connect
    /// those tiles.
    ///
    /// # Arguments
    /// * `net_a` - Index of the first net
    /// * `net_b` - Index of the second net
    ///
    /// # Returns
    /// A vector of `((i1, j1), (i2, j2), (node_idx, mask))` where:
    /// - `(i1, j1)` and `(i2, j2)` are adjacent tile coordinates
    /// - `node_idx` is the connecting node index (0-5 for N1-N6)
    /// - `mask` is the bit mask (e.g., N1, N2, etc.) to apply to both tiles
    pub fn get_connecting_masks_for_nets(
        &self,
        net_a: usize,
        net_b: usize,
    ) -> Vec<((usize, usize), (usize, usize), usize)> {
        let adjacent_pairs = self.find_adjacent_tile_pairs(net_a, net_b, false);
        let mut masks = Vec::new();

        for &((i1, j1), (i2, j2)) in &adjacent_pairs {
            if let Some( mask) = self.get_connecting_node_mask((i1, j1), (i2, j2)) {
                masks.push(((i1, j1), (i2, j2), mask));
            }
        }

        masks
    }

    /// Returns one connecting node mask for an adjacent tile pair between two nets.
    ///
    /// Finds one pair of adjacent tiles between `net_a` and `net_b` and returns
    /// the connecting node mask needed to join them.
    ///
    /// # Arguments
    /// * `net_a` - Index of the first net
    /// * `net_b` - Index of the second net
    ///
    /// # Returns
    /// `Some(((i1, j1), (i2, j2), (node_idx, mask)))` if an adjacent pair exists,
    /// `None` if no adjacent tiles exist between the nets
    pub fn get_one_connecting_mask_for_nets(
        &self,
        net_a: usize,
        net_b: usize,
    ) -> Option<((usize, usize), (usize, usize), usize)> {
        if let Some(((i1, j1), (i2, j2))) = self.find_one_adjacent_tile_pair(net_a, net_b) {
            self.get_connecting_node_mask((i1, j1), (i2, j2))
                .map(|mask| ((i1, j1), (i2, j2),  mask))
        } else {
            None
        }
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
    fn test_select_common_surrounding_coordinate_with_info() {
        // Create two nearby nets with multiple tiles
        let mut net0 = Nodes::new();
        net0.insert(Node::new(2, 2, 0));
        net0.insert(Node::new(2, 3, 1));

        let mut net1 = Nodes::new();
        net1.insert(Node::new(4, 2, 0));
        net1.insert(Node::new(5, 2, 1));

        let maze = create_test_maze_with_nets(vec![net0, net1], 8, 8);
        let result = maze.select_common_surrounding_coordinate_with_info();

        if let Some(info) = result {
            println!("Net Connection Info:");
            println!("  Net A index: {}", info.net_a_index);
            println!("  Net B index: {}", info.net_b_index);
            println!("  Common coordinate: {:?}", info.common_coordinate);
            println!("  Net A coordinates: {:?}", info.net_a_coordinates);
            println!("  Net B coordinates: {:?}", info.net_b_coordinates);
            println!("  Proximity level: {:?}", info.proximity_level);

            // Verify the net indices are ordered
            assert!(info.net_a_index < info.net_b_index);

            // Verify that net A has the expected tiles
            assert!(info.net_a_coordinates.contains(&(2, 2)));
            assert!(info.net_a_coordinates.contains(&(2, 3)));

            // Verify that net B has the expected tiles
            assert!(info.net_b_coordinates.contains(&(4, 2)));
            assert!(info.net_b_coordinates.contains(&(5, 2)));

            // Verify the common coordinate is within bounds
            assert!(info.common_coordinate.0 < 8);
            assert!(info.common_coordinate.1 < 8);
        }
    }

    #[test]
    fn test_select_common_surrounding_coordinate_with_info_intersection() {
        // Create two nets that share a tile (intersection)
        let mut net0 = Nodes::new();
        net0.insert(Node::new(2, 2, 0));
        net0.insert(Node::new(3, 2, 1));

        let mut net1 = Nodes::new();
        net1.insert(Node::new(2, 2, 3)); // Same tile as net0
        net1.insert(Node::new(2, 3, 4));

        let maze = create_test_maze_with_nets(vec![net0, net1], 8, 8);
        let result = maze.select_common_surrounding_coordinate_with_info();

        if let Some(info) = result {
            println!("Intersection case:");
            println!("  Proximity level: {:?}", info.proximity_level);
            println!("  Common coordinate: {:?}", info.common_coordinate);

            // Should be intersection proximity
            assert_eq!(info.proximity_level, ProximityLevel::Intersection);

            // The common coordinate should be the shared tile
            assert_eq!(info.common_coordinate, (2, 2));
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
    fn test_surrounding_excludes_own_net_tiles() {
        // Create two adjacent nets - net B is at a position that would be
        // in net A's surrounding coordinates
        let mut net0 = Nodes::new();
        net0.insert(Node::new(2, 2, 0));

        let mut net1 = Nodes::new();
        // Place net1 adjacent to net0 (this tile would normally be in net0's surroundings)
        net1.insert(Node::new(3, 2, 0));

        let maze = create_test_maze_with_nets(vec![net0, net1], 8, 8);

        // Get surrounding coords for net0 - should NOT include its own tile (2, 2)
        let surrounding_0 = maze.get_surrounding_coordinates_for_net(0);
        assert!(
            !surrounding_0.contains(&(2, 2)),
            "Net 0's surrounding should not include its own tile (2, 2)"
        );

        // Get surrounding coords for net1 - should NOT include its own tile (3, 2)
        let surrounding_1 = maze.get_surrounding_coordinates_for_net(1);
        assert!(
            !surrounding_1.contains(&(3, 2)),
            "Net 1's surrounding should not include its own tile (3, 2)"
        );

        // With the tiered approach, find_common_surrounding_coordinates uses
        // Tier 2 (adjacent) when Tier 1 (intersection) is empty.
        // So it WILL include net tiles that are adjacent to the other net.
        let result = maze.find_closest_common_coordinates(0, 1);
        if let Some((tiles, level)) = result {
            println!("Proximity level: {:?}", level);
            println!("Common tiles: {:?}", tiles);

            // When nets are adjacent, Tier 2 returns net tiles that are adjacent to the other
            if level == ProximityLevel::Adjacent {
                // At least one net tile should be in the result
                assert!(
                    tiles.contains(&(2, 2)) || tiles.contains(&(3, 2)),
                    "Adjacent level should include net tiles that are adjacent to each other"
                );
            }
        }

        println!("Net 0 tiles: (2, 2)");
        println!("Net 1 tiles: (3, 2)");
        println!("Surrounding for net 0: {:?}", surrounding_0);
        println!("Surrounding for net 1: {:?}", surrounding_1);
    }

    #[test]
    fn test_find_all_common_with_tiered_approach() {
        // Create three nets where some are adjacent to each other
        let mut net0 = Nodes::new();
        net0.insert(Node::new(2, 2, 0));

        let mut net1 = Nodes::new();
        net1.insert(Node::new(4, 2, 0));

        // Place net2 between net0 and net1
        let mut net2 = Nodes::new();
        net2.insert(Node::new(3, 2, 0));

        let maze = create_test_maze_with_nets(vec![net0, net1, net2], 8, 8);
        let all_closest = maze.find_all_closest_common_coordinates();

        // With the tiered approach, adjacent nets will have their tiles included
        // in the result (Tier 2 - adjacent)
        for ((a, b), (coords, level)) in &all_closest {
            println!("Pair ({}, {}), level {:?}: {:?}", a, b, level, coords);

            match level {
                ProximityLevel::Intersection => {
                    // Intersection tiles should be in both nets
                    println!("  -> Intersection between nets {} and {}", a, b);
                }
                ProximityLevel::Adjacent => {
                    // Adjacent tiles should be net tiles adjacent to the other net
                    println!("  -> Adjacent tiles between nets {} and {}", a, b);
                }
                ProximityLevel::Surrounding => {
                    // Surrounding tiles should NOT be in any net
                    let all_net_tiles = maze.get_all_net_tiles();
                    for coord in coords {
                        assert!(
                            !all_net_tiles.contains(coord),
                            "Surrounding level should not include any net tiles"
                        );
                    }
                }
            }
        }

        // Verify that find_all_common_surrounding_coordinates returns the same tiles
        let all_common = maze.find_all_common_surrounding_coordinates();
        assert_eq!(all_closest.len(), all_common.len());
        for (pair, (tiles, _)) in &all_closest {
            assert_eq!(all_common.get(pair), Some(tiles));
        }
    }

    // ==================== Tiered Approach Tests ====================

    #[test]
    fn test_tier1_intersection_tiles() {
        // Create two nets that share a tile (same (i, j) coordinates)
        let mut net0 = Nodes::new();
        net0.insert(Node::new(2, 2, 0)); // node 0 at tile (2, 2)
        net0.insert(Node::new(3, 2, 1)); // node 1 at tile (3, 2)

        let mut net1 = Nodes::new();
        net1.insert(Node::new(2, 2, 3)); // node 3 at tile (2, 2) - SAME TILE as net0!
        net1.insert(Node::new(2, 3, 4)); // node 4 at tile (2, 3)

        let maze = create_test_maze_with_nets(vec![net0, net1], 8, 8);

        // find_intersection_tiles should find (2, 2) as it's in both nets
        let intersection = maze.find_intersection_tiles(0, 1);
        assert!(
            intersection.contains(&(2, 2)),
            "Intersection should contain tile (2, 2) which is in both nets"
        );
        assert_eq!(
            intersection.len(),
            1,
            "Should only have one intersection tile"
        );
    }

    #[test]
    fn test_tier1_returns_first_when_intersection_exists() {
        // Create two nets that share a tile
        let mut net0 = Nodes::new();
        net0.insert(Node::new(2, 2, 0));

        let mut net1 = Nodes::new();
        net1.insert(Node::new(2, 2, 3)); // Same tile, different node

        let maze = create_test_maze_with_nets(vec![net0, net1], 8, 8);

        let result = maze.find_closest_common_coordinates(0, 1);
        assert!(result.is_some());

        let (tiles, level) = result.unwrap();
        assert_eq!(
            level,
            ProximityLevel::Intersection,
            "Should return Intersection level when nets share tiles"
        );
        assert!(tiles.contains(&(2, 2)));
    }

    #[test]
    fn test_tier2_adjacent_tiles() {
        // Create two nets where one tile is adjacent to (in surrounding of) the other
        // Net 0 at (2, 2), Net 1 at (3, 2) which should be adjacent
        let mut net0 = Nodes::new();
        net0.insert(Node::new(2, 2, 0));

        let mut net1 = Nodes::new();
        net1.insert(Node::new(3, 2, 0));

        let maze = create_test_maze_with_nets(vec![net0, net1], 8, 8);

        // Check if (3, 2) is in net0's surrounding
        let surrounding_0 = maze.get_surrounding_coordinates_for_net(0);
        let surrounding_1 = maze.get_surrounding_coordinates_for_net(1);

        // If tiles are adjacent, one should be in the other's surrounding
        let adjacent = maze.find_adjacent_to_other_tiles(0, 1);

        println!("Net 0 at (2, 2), surrounding: {:?}", surrounding_0);
        println!("Net 1 at (3, 2), surrounding: {:?}", surrounding_1);
        println!("Adjacent tiles: {:?}", adjacent);

        // At least one of them should be adjacent to the other
        // (depends on hex grid layout, but adjacent tiles should find each other)
        if surrounding_0.contains(&(3, 2)) || surrounding_1.contains(&(2, 2)) {
            assert!(!adjacent.is_empty(), "Adjacent tiles should not be empty when nets are neighbors");
        }
    }

    #[test]
    fn test_tier2_returns_second_when_no_intersection() {
        // Create two adjacent nets (no intersection, but adjacent)
        let mut net0 = Nodes::new();
        net0.insert(Node::new(2, 2, 0));

        let mut net1 = Nodes::new();
        net1.insert(Node::new(3, 2, 0)); // Adjacent tile, not same tile

        let maze = create_test_maze_with_nets(vec![net0, net1], 8, 8);

        // First verify there's no intersection
        let intersection = maze.find_intersection_tiles(0, 1);
        assert!(intersection.is_empty(), "Should have no intersection tiles");

        let result = maze.find_closest_common_coordinates(0, 1);

        if let Some((_, level)) = result {
            // Should be either Adjacent or Surrounding (depending on hex grid layout)
            assert!(
                level == ProximityLevel::Adjacent || level == ProximityLevel::Surrounding,
                "Should return Adjacent or Surrounding when no intersection exists"
            );
        }
    }

    #[test]
    fn test_tier3_surrounding_tiles() {
        // Create two nets that are close but not touching
        // They should have tiles in their mutual surroundings
        let mut net0 = Nodes::new();
        net0.insert(Node::new(1, 2, 0));

        let mut net1 = Nodes::new();
        net1.insert(Node::new(4, 2, 0)); // A bit further away

        let maze = create_test_maze_with_nets(vec![net0, net1], 8, 8);

        let surrounding_both = maze.find_surrounding_both_tiles(0, 1);
        println!("Tiles surrounding both nets: {:?}", surrounding_both);
    }

    #[test]
    fn test_tiered_approach_priority() {
        // Test that intersection takes priority over adjacent
        let mut net0 = Nodes::new();
        net0.insert(Node::new(2, 2, 0));
        net0.insert(Node::new(3, 2, 1)); // Extends to adjacent tile

        let mut net1 = Nodes::new();
        net1.insert(Node::new(2, 2, 3)); // Same tile as net0's first node
        net1.insert(Node::new(4, 2, 4)); // Further away

        let maze = create_test_maze_with_nets(vec![net0, net1], 8, 8);

        let result = maze.find_closest_common_coordinates(0, 1);
        assert!(result.is_some());

        let (tiles, level) = result.unwrap();
        // Should return Intersection because (2, 2) is in both nets
        assert_eq!(
            level,
            ProximityLevel::Intersection,
            "Intersection should take priority"
        );
        assert!(tiles.contains(&(2, 2)));
        // Should NOT include tiles that are only adjacent
        assert!(!tiles.contains(&(3, 2)));
        assert!(!tiles.contains(&(4, 2)));

    #[test]
    fn test_get_connecting_node_mask() {
            // Create two nets with adjacent tiles
            let mut net0 = Nodes::new();
            net0.insert(Node::new(2, 2, 0));  // Tile at (2,2)

            let mut net1 = Nodes::new();
            net1.insert(Node::new(3, 2, 0));  // Tile at (3,2) - adjacent to (2,2) on node 0 (N1)

            let maze = create_test_maze_with_nets(vec![net0, net1], 8, 8);

            // Test getting connecting mask between adjacent tiles
            let result = maze.get_connecting_node_mask((2, 2), (3, 2));
            assert!(result.is_some(), "Should find connecting node mask for adjacent tiles");
            let mask = result.unwrap();
            assert_eq!(mask, N1, "Mask should be N1");

            // Test reverse direction (should give same result)
            let reverse_result = maze.get_connecting_node_mask((3, 2), (2, 2));
            assert!(reverse_result.is_some(), "Should work in reverse direction");
            let mask_rev = reverse_result.unwrap();
            assert_eq!(mask_rev, N1, "Mask should be N1 in reverse");

            // Test with non-adjacent tiles
            let non_adjacent = maze.get_connecting_node_mask((2, 2), (5, 5));
            assert!(non_adjacent.is_none(), "Should return None for non-adjacent tiles");

            // Test with out-of-bounds coordinates
            let out_of_bounds = maze.get_connecting_node_mask((0, 0), (10, 10));
            assert!(out_of_bounds.is_none(), "Should return None for out-of-bounds tiles");
        }

        #[test]
        fn test_get_connecting_masks_for_nets() {
            // Create two nets with multiple adjacent tiles
            let mut net0 = Nodes::new();
            net0.insert(Node::new(2, 2, 0));  // Tile at (2,2)
            net0.insert(Node::new(3, 3, 1));  // Tile at (3,3)

            let mut net1 = Nodes::new();
            net1.insert(Node::new(3, 2, 0));  // Tile at (3,2) - adjacent to (2,2) on node 0 (N1)
            net1.insert(Node::new(4, 3, 1));  // Tile at (4,3) - adjacent to (3,3) on node 1 (N2)

            let maze = create_test_maze_with_nets(vec![net0, net1], 8, 8);

            // Test getting all connecting masks
            let masks = maze.get_connecting_masks_for_nets(0, 1);
            // Should find at least 2 connections
            assert!(!masks.is_empty(), "Should find connecting masks for adjacent nets");

            // Verify each mask is valid
            for &((i1, j1), (i2, j2), mask) in &masks {
                assert_ne!((i1, j1), (i2, j2), "Tiles should be different");
                assert!(mask == N1 || mask == N2 || mask == N3 || mask == N4 || mask == N5 || mask == N6,
                    "Mask should be one of N1-N6");

                // Verify the tiles are from different nets
                let net0_tiles = maze.get_tiles_for_net(0);
                let net1_tiles = maze.get_tiles_for_net(1);
                assert!(
                    (net0_tiles.contains(&(i1, j1)) && net1_tiles.contains(&(i2, j2))) ||
                    (net1_tiles.contains(&(i1, j1)) && net0_tiles.contains(&(i2, j2))),
                    "Tiles should be from different nets"
                );
            }

            // Test with nets that have no adjacent tiles
            let mut net2 = Nodes::new();
            net2.insert(Node::new(0, 0, 0));
            let mut net3 = Nodes::new();
            net3.insert(Node::new(6, 6, 0));
            let maze2 = create_test_maze_with_nets(vec![net2, net3], 8, 8);

            let no_masks = maze2.get_connecting_masks_for_nets(0, 1);
            assert!(no_masks.is_empty(), "Should return empty vector when no adjacent tiles exist");
        }

        #[test]
        fn test_get_one_connecting_mask_for_nets() {
            // Create two nets with adjacent tiles
            let mut net0 = Nodes::new();
            net0.insert(Node::new(2, 2, 0));  // Tile at (2,2)
            net0.insert(Node::new(2, 3, 1));  // Another tile in same net

            let mut net1 = Nodes::new();
            net1.insert(Node::new(3, 2, 0));  // Tile at (3,2) - adjacent to (2,2)
            net1.insert(Node::new(4, 2, 1));  // Tile at (4,2) - adjacent to (3,2) but not to net0

            let maze = create_test_maze_with_nets(vec![net0, net1], 8, 8);

            // Test getting one connecting mask
            let one_mask = maze.get_one_connecting_mask_for_nets(0, 1);
            assert!(one_mask.is_some(), "Should find one connecting mask for adjacent nets");
            let ((i1, j1), (i2, j2), mask) = one_mask.unwrap();
            assert_ne!((i1, j1), (i2, j2), "Tiles should be different");
            assert!(mask == N1 || mask == N2 || mask == N3 || mask == N4 || mask == N5 || mask == N6,
                "Mask should be one of N1-N6");

            // Verify the tiles are from different nets
            let net0_tiles = maze.get_tiles_for_net(0);
            let net1_tiles = maze.get_tiles_for_net(1);
            assert!(
                (net0_tiles.contains(&(i1, j1)) && net1_tiles.contains(&(i2, j2))) ||
                (net1_tiles.contains(&(i1, j1)) && net0_tiles.contains(&(i2, j2))),
                "Tiles should be from different nets"
            );

            // Test with nets that have no adjacent tiles
            let mut net2 = Nodes::new();
            net2.insert(Node::new(0, 0, 0));
            let mut net3 = Nodes::new();
            net3.insert(Node::new(6, 6, 0));
            let maze2 = create_test_maze_with_nets(vec![net2, net3], 8, 8);

            let no_mask = maze2.get_one_connecting_mask_for_nets(0, 1);
            assert!(no_mask.is_none(), "Should return None when no adjacent tiles exist");
        }
    }

    #[test]
    fn test_find_all_closest_with_proximity_levels() {
        // Create nets with different proximity levels to test the all-pairs function
        let mut net0 = Nodes::new();
        net0.insert(Node::new(2, 2, 0));

        let mut net1 = Nodes::new();
        net1.insert(Node::new(2, 2, 3)); // Intersects with net0 at (2, 2)

        let mut net2 = Nodes::new();
        net2.insert(Node::new(8, 8, 0)); // Far from both

        let maze = create_test_maze_with_nets(vec![net0, net1, net2], 12, 12);

        let all_closest = maze.find_all_closest_common_coordinates();

        // Net 0 and Net 1 should have Intersection proximity
        if let Some((_, level)) = all_closest.get(&(0, 1)) {
            assert_eq!(
                *level,
                ProximityLevel::Intersection,
                "Nets 0 and 1 should have Intersection proximity"
            );
        }

        // Net 0 and Net 2, and Net 1 and Net 2 should have no proximity (too far)
        // They may or may not be in the result depending on grid size
        println!("All closest pairs: {:?}", all_closest.keys().collect::<Vec<_>>());
    }

    #[test]
    fn test_no_proximity_returns_none() {
        // Create two nets that are too far apart to have any proximity
        let mut net0 = Nodes::new();
        net0.insert(Node::new(0, 0, 0));

        let mut net1 = Nodes::new();
        net1.insert(Node::new(9, 9, 0));

        let maze = create_test_maze_with_nets(vec![net0, net1], 10, 10);

        let result = maze.find_closest_common_coordinates(0, 1);
        assert!(
            result.is_none(),
            "Should return None when nets have no proximity at any level"
        );
    }

    #[test]
    fn test_find_adjacent_tile_pairs() {
        // Create two nets with adjacent tiles
        let mut net0 = Nodes::new();
        net0.insert(Node::new(2, 2, 0));  // Tile at (2,2)
        net0.insert(Node::new(2, 3, 1));  // Another tile in same net

        let mut net1 = Nodes::new();
        net1.insert(Node::new(3, 2, 0));  // Tile at (3,2) - adjacent to (2,2)
        net1.insert(Node::new(4, 2, 1));  // Tile at (4,2) - adjacent to (3,2) but not to net0

        let maze = create_test_maze_with_nets(vec![net0, net1], 8, 8);

        // Test with find_one = false (get all pairs)
        let all_pairs = maze.find_adjacent_tile_pairs(0, 1, false);
        // Should find at least one adjacent pair: (2,2) adjacent to (3,2)
        assert!(!all_pairs.is_empty(), "Should find at least one adjacent tile pair");
        assert!(all_pairs.iter().any(|&((i1, j1), (i2, j2))|
            (i1 == 2 && j1 == 2 && i2 == 3 && j2 == 2) ||
            (i1 == 3 && j1 == 2 && i2 == 2 && j2 == 2)
        ), "Should find the adjacent pair between (2,2) and (3,2)");

        // Test with find_one = true (get just one pair)
        let one_pair = maze.find_adjacent_tile_pairs(0, 1, true);
        assert!(one_pair.len() <= 1, "Should return at most one pair when find_one is true");
        if !one_pair.is_empty() {
            // The pair should be a valid adjacent pair
            let ((i1, j1), (i2, j2)) = one_pair[0];
            assert_ne!((i1, j1), (i2, j2), "Pair should be two different tiles");
            // Verify they are from different nets (one from net0, one from net1)
            let net0_tiles = maze.get_tiles_for_net(0);
            let net1_tiles = maze.get_tiles_for_net(1);
            assert!(
                (net0_tiles.contains(&(i1, j1)) && net1_tiles.contains(&(i2, j2))) ||
                (net1_tiles.contains(&(i1, j1)) && net0_tiles.contains(&(i2, j2))),
                "Pair should consist of one tile from each net"
            );
        }

        // Test with nets that have no adjacent tiles
        let mut net2 = Nodes::new();
        net2.insert(Node::new(0, 0, 0));
        let mut net3 = Nodes::new();
        net3.insert(Node::new(6, 6, 0));
        let maze2 = create_test_maze_with_nets(vec![net2, net3], 8, 8);

        let no_pairs = maze2.find_adjacent_tile_pairs(0, 1, false);
        assert!(no_pairs.is_empty(), "Should return empty vector when no adjacent tiles exist");

        let no_pairs_one = maze2.find_adjacent_tile_pairs(0, 1, true);
        assert!(no_pairs_one.is_empty(), "Should return empty vector when no adjacent tiles exist even with find_one");
    }

    #[test]
    fn test_find_one_adjacent_tile_pair() {
        // Create two nets with adjacent tiles
        let mut net0 = Nodes::new();
        net0.insert(Node::new(2, 2, 0));  // Tile at (2,2)
        net0.insert(Node::new(2, 3, 1));  // Another tile in same net

        let mut net1 = Nodes::new();
        net1.insert(Node::new(3, 2, 0));  // Tile at (3,2) - adjacent to (2,2)
        net1.insert(Node::new(4, 2, 1));  // Tile at (4,2) - adjacent to (3,2) but not to net0

        let maze = create_test_maze_with_nets(vec![net0, net1], 8, 8);

        // Test with find_one_adjacent_tile_pair (should find one pair)
        let one_pair = maze.find_one_adjacent_tile_pair(0, 1);
        assert!(one_pair.is_some(), "Should find at least one adjacent tile pair");
        let ((i1, j1), (i2, j2)) = one_pair.unwrap();
        assert_ne!((i1, j1), (i2, j2), "Pair should be two different tiles");

        // Verify they are from different nets (one from net0, one from net1)
        let net0_tiles = maze.get_tiles_for_net(0);
        let net1_tiles = maze.get_tiles_for_net(1);
        assert!(
            (net0_tiles.contains(&(i1, j1)) && net1_tiles.contains(&(i2, j2))) ||
            (net1_tiles.contains(&(i1, j1)) && net0_tiles.contains(&(i2, j2))),
            "Pair should consist of one tile from each net"
        );

        // Test with nets that have no adjacent tiles
        let mut net2 = Nodes::new();
        net2.insert(Node::new(0, 0, 0));
        let mut net3 = Nodes::new();
        net3.insert(Node::new(6, 6, 0));
        let maze2 = create_test_maze_with_nets(vec![net2, net3], 8, 8);

        let no_pair = maze2.find_one_adjacent_tile_pair(0, 1);
        assert!(no_pair.is_none(), "Should return None when no adjacent tiles exist");
    }
}

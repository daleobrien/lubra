use crate::maze::maze::Maze;
use std::collections::{HashMap, HashSet};

/// Methods for finding intersections between nets.
impl Maze {
    /// Finds all intersections between different nets where they share the same tiles.
    ///
    /// This function analyzes coordinate maps for multiple nets and identifies where
    /// different nets pass through the same tile coordinates. It also tracks which nets
    /// are connected to each other through intermediary sets to avoid redundant checks.
    ///
    /// # Arguments
    /// * `coord_maps` - A vector where each entry is a HashMap mapping (i,j) coordinates
    ///                  to the list of node indices that net uses at that coordinate
    ///
    /// # Returns
    /// A HashMap mapping pairs of net indices (n, m) to the set of (i, j) coordinates
    /// where those nets intersect
    pub(crate) fn get_intersections(
        coord_maps: Vec<HashMap<(usize, usize), Vec<usize>>>,
    ) -> HashMap<(usize, usize), HashSet<(usize, usize)>> {
        let mut shared_coords: HashMap<(usize, usize), HashSet<(usize, usize)>> = HashMap::new();

        // <set #, set connected nodes >
        let mut node_to_set: Vec<HashSet<usize>> = vec![];

        for (n, coords_n) in coord_maps.iter().enumerate() {
            let n_keys: HashSet<(usize, usize)> = coords_n.keys().cloned().collect();
            for (m, coords_m) in coord_maps.iter().enumerate() {
                if m <= n {
                    continue; // It is symmetric!
                }

                // If n and m are in the same set, then we don't need to check them since they have
                // already have an intersection via some intermediary set.
                // For example: If I have some joins on say, 1/2 and 2/3 then I don't need to check 2/3
                let index_n = node_to_set.iter().position(|x| x.contains(&n));
                let index_m = node_to_set.iter().position(|x| x.contains(&m));
                if index_n.is_some() && index_n == index_m {
                    continue;
                }

                // Find any intersections between the two sets, i.e. where different nets share the same tile
                let m_keys: HashSet<(usize, usize)> = coords_m.keys().cloned().collect();
                let intersecting: HashSet<(usize, usize)> =
                    n_keys.intersection(&m_keys).cloned().collect();

                if !intersecting.is_empty() {
                    // Some intersections have been found!
                    match (index_n, index_m) {
                        (Some(index_n), Some(index_m)) => {
                            // These two sets are connected, so we can merge them
                            // None: `m > n` is always true.
                            let (left, right) = node_to_set.split_at_mut(index_m);
                            let set_n = &mut left[index_n];
                            let set_m = &right[0];
                            set_n.extend(set_m.iter());
                            node_to_set.remove(index_m);

                            shared_coords.insert((n, m), intersecting);
                        }
                        (Some(index_n), None) => {
                            // Add m to n's set
                            let set_n = node_to_set.get_mut(index_n).unwrap();
                            set_n.insert(m);
                            shared_coords.insert((n, m), intersecting);
                        }
                        (None, Some(index_m)) => {
                            // Add n to m's set
                            let set_m = node_to_set.get_mut(index_m).unwrap();
                            set_m.insert(n);
                            shared_coords.insert((n, m), intersecting);
                        }
                        _ => {
                            // Create a new set
                            node_to_set.push(vec![n, m].into_iter().collect());
                            shared_coords.insert((n, m), intersecting);
                        }
                    }
                }
            }
        }
        shared_coords
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_intersections_empty_input() {
        let coord_maps = vec![];
        let result = Maze::get_intersections(coord_maps);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_get_intersections_single_net() {
        let mut map1 = HashMap::new();
        map1.insert((0, 0), vec![0, 1]);
        map1.insert((0, 1), vec![2, 3]);

        let coord_maps = vec![map1];
        let result = Maze::get_intersections(coord_maps);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_get_intersections_no_shared_coordinates() {
        let mut map1 = HashMap::new();
        map1.insert((0, 0), vec![0, 1]);
        map1.insert((0, 1), vec![2, 3]);

        let mut map2 = HashMap::new();
        map2.insert((1, 0), vec![0, 1]);
        map2.insert((1, 1), vec![2, 3]);

        let coord_maps = vec![map1, map2];
        let result = Maze::get_intersections(coord_maps);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_get_intersections_single_shared_coordinate() {
        let mut map1 = HashMap::new();
        map1.insert((0, 0), vec![0, 1]);
        map1.insert((0, 1), vec![2, 3]);

        let mut map2 = HashMap::new();
        map2.insert((0, 0), vec![4, 5]);
        map2.insert((1, 1), vec![2, 3]);

        let coord_maps = vec![map1, map2];
        let result = Maze::get_intersections(coord_maps);

        assert_eq!(result.len(), 1);
        assert!(result.contains_key(&(0, 1)));

        let intersection = result.get(&(0, 1)).unwrap();
        assert_eq!(intersection.len(), 1);
        assert!(intersection.contains(&(0, 0)));
    }

    #[test]
    fn test_get_intersections_two_shared_coordinates() {
        // Two distinct nets share exactly two coordinates
        let mut map1 = HashMap::new();
        map1.insert((0, 0), vec![0, 1]);
        map1.insert((0, 1), vec![2, 3]);
        map1.insert((1, 0), vec![4, 5]);

        let mut map2 = HashMap::new();
        map2.insert((0, 0), vec![1, 2]);  // Shared with net 1
        map2.insert((0, 1), vec![3, 4]);  // Shared with net 1
        map2.insert((2, 2), vec![5, 0]);

        let coord_maps = vec![map1, map2];
        let result = Maze::get_intersections(coord_maps);

        assert_eq!(result.len(), 1);
        assert!(result.contains_key(&(0, 1)));

        let intersection = result.get(&(0, 1)).unwrap();
        assert_eq!(intersection.len(), 2);
        assert!(intersection.contains(&(0, 0)));
        assert!(intersection.contains(&(0, 1)));
    }

    #[test]
    fn test_get_intersections_three_nets_with_multiple_intersections() {
        // Net 0 and Net 1 share (0, 0)
        // Net 1 and Net 2 share (1, 1)
        // Net 0 and Net 2 should be skipped because they're connected via Net 1
        let mut map0 = HashMap::new();
        map0.insert((0, 0), vec![0, 1]);
        map0.insert((2, 0), vec![2, 3]);

        let mut map1 = HashMap::new();
        map1.insert((0, 0), vec![1, 2]);  // Shared with net 0
        map1.insert((1, 1), vec![3, 4]);  // Shared with net 2

        let mut map2 = HashMap::new();
        map2.insert((1, 1), vec![4, 5]);  // Shared with net 1
        map2.insert((3, 3), vec![0, 1]);

        let coord_maps = vec![map0, map1, map2];
        let result = Maze::get_intersections(coord_maps);

        // Should have intersections for (0,1) and (1,2)
        assert_eq!(result.len(), 2);

        // Net 0 and Net 1 intersect at (0, 0)
        assert!(result.contains_key(&(0, 1)));
        let intersection_01 = result.get(&(0, 1)).unwrap();
        assert_eq!(intersection_01.len(), 1);
        assert!(intersection_01.contains(&(0, 0)));

        // Net 1 and Net 2 intersect at (1, 1)
        assert!(result.contains_key(&(1, 2)));
        let intersection_12 = result.get(&(1, 2)).unwrap();
        assert_eq!(intersection_12.len(), 1);
        assert!(intersection_12.contains(&(1, 1)));

        // Net 0 and Net 2 should NOT have a direct entry because they're
        // connected via Net 1 (transitively connected)
        assert!(!result.contains_key(&(0, 2)));
    }

    #[test]
    fn test_get_intersections_multiple_shared_coordinates() {
        // Two nets share three coordinates
        let mut map1 = HashMap::new();
        map1.insert((0, 0), vec![0, 1]);
        map1.insert((0, 1), vec![2, 3]);
        map1.insert((1, 0), vec![4, 5]);
        map1.insert((1, 1), vec![0, 2]);
        map1.insert((2, 0), vec![1, 3]);

        let mut map2 = HashMap::new();
        map2.insert((0, 0), vec![1, 2]);  // Shared
        map2.insert((0, 1), vec![3, 4]);  // Shared
        map2.insert((1, 0), vec![5, 0]);  // Shared
        map2.insert((5, 5), vec![1, 2]);

        let coord_maps = vec![map1, map2];
        let result = Maze::get_intersections(coord_maps);

        assert_eq!(result.len(), 1);
        assert!(result.contains_key(&(0, 1)));

        let intersection = result.get(&(0, 1)).unwrap();
        assert_eq!(intersection.len(), 3);
        assert!(intersection.contains(&(0, 0)));
        assert!(intersection.contains(&(0, 1)));
        assert!(intersection.contains(&(1, 0)));
    }

    #[test]
    fn test_get_intersections_symmetry() {
        // Verify that (n, m) is stored but not (m, n) due to m > n check
        let mut map1 = HashMap::new();
        map1.insert((0, 0), vec![0, 1]);

        let mut map2 = HashMap::new();
        map2.insert((0, 0), vec![2, 3]);

        let coord_maps = vec![map1, map2];
        let result = Maze::get_intersections(coord_maps);

        assert!(result.contains_key(&(0, 1)));
        assert!(!result.contains_key(&(1, 0)));
    }

    #[test]
    fn test_get_intersections_four_nets_complex() {
        // Net 0, 1 share (0, 0)
        // Net 0, 2 share (1, 1)
        // Net 0, 1, 2 are all transitively connected
        // Net 3 shares (2, 2) with Net 2
        let mut map0 = HashMap::new();
        map0.insert((0, 0), vec![0]);
        map0.insert((1, 1), vec![1]);

        let mut map1 = HashMap::new();
        map1.insert((0, 0), vec![2]);

        let mut map2 = HashMap::new();
        map2.insert((1, 1), vec![3]);
        map2.insert((2, 2), vec![4]);

        let mut map3 = HashMap::new();
        map3.insert((2, 2), vec![5]);

        let coord_maps = vec![map0, map1, map2, map3];
        let result = Maze::get_intersections(coord_maps);

        // Should have (0,1), (0,2), and (2,3)
        // (1,2) should be skipped because 1 and 2 are connected via 0
        assert!(result.contains_key(&(0, 1)));
        assert!(result.contains_key(&(0, 2)));
        assert!(result.contains_key(&(2, 3)));
        assert!(!result.contains_key(&(1, 2)));
    }
}

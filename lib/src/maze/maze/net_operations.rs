use std::collections::{HashMap, HashSet};
use common::consts::{N1, N2, N3, N4, N5, N6, NODE_TO_COORD};
use common::types::coordinate::add;
use rand_distr::num_traits::ToPrimitive;
use crate::maze::tile::Tile;
use crate::maze::maze::Maze;

/// Net operation methods for the Maze struct.
/// These methods handle network management including cycle removal,
/// net reduction, and network updates.
impl Maze {
    pub fn remove_cycles(&mut self) {
        println!("Removing cycles ...");
        // We can do this by removing any tile that is part of a cycle
        let mut visited: HashSet<(usize, usize, usize)> = HashSet::new();

        for net in self.nets.iter() {
            let mut cycle = false;
            for [i, j, k] in net.iter() {
                //println!("@ {}, {}, {}", i, j, k);
                if visited.contains(&(*j, *i, *k)) {
                    cycle = true;
                    println!("Cycle detected @ ->: {}, {}, {}", i, j, k);
                    break;
                }
                visited.insert((*j, *i, *k));
            }

            if cycle {
                // TODO: Break the cycles
            }
        }
    }

    pub fn reduce_nets(&mut self, lower_limit: usize) {
        self.sort_nets_by_length();
        self.remove_short_nets(lower_limit);

        // Create a hashSet of the nets, but used just i,j coordinates as the keys
        let coord_maps = self.create_coord_maps();

        // NOTE: Do I only need get intersections one at a time, since we are merging them?
        // But faster to get them all at once, and then not use the later ones we no longer
        // need

        // We can do the below until there is only none list
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
                            if nodes.iter().any(|&node| net.contains(&[*i, *j, node])) {
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
                        self.nets.remove(other_index);
                    } else {
                        println!("Error, we don't have 2 indexes to merge {:?}", indexes);
                    }
                    found = true;
                    break;
                }
            }
            if !found {
                // We can't merge these nets in a simple way
                println!("Couldn't join: ({m},{n}){:?}", intersections);
                // TODO: Implement more complex joins ...
                // This occurs when at least one net passing though the same tile as the other net
            }

            // If I merge one, then all other mergers will be need to be updated
            // Example:
            // If 1/2 merged then 1/3 & 2/3 will effectively merged too, and so on. But we don't need to scan, so that's ok
            // thus, skip all 2/n provided joins if there is a 1/n, and I will iterate over a 1/n before a 2/n
        }

        //println!("nets  {:?}", self.nets);
        //println!("tiles  {:?}", self.tiles);
        // println!("Shared coords: {:?}", shared_coords);

        println!("\nWe have disjoint {:?} nets:", self.nets.len());

        // for (n, nets) in self.nets.iter_mut().enumerate() {
        //     println!("{n} -> {}, {:?}", nets.len(), nets);
        // }

        /*

        net_1 = [[i,j,k], [i,j,k], <n>] .. these are sets of tiles that are connected.
        net_2 = [[i,j,k], [i,j,k], <m>]

        Take the shortest net, and iterate over it looking for the first adjacent tile that is not part of the net.
        Then replace the matching tile such that the nets are connected and move the short net over to the larger net.

        Repeat until we only have one net.

        The surrounding_tiles are of the form:

                    1
                  ╱ ╲
                4      5
              ╱         ╲
             2  ━  6  ━   3

            Either up or down

            ▽△▽△▽      △▽△▽△:
            △▽▲▽△  or  ▽△▼△▽
            ▽△▽△▽      △▽△▽△

         */
    }

    fn get_intersections(
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
        return shared_coords;
    }

    fn create_coord_maps(&mut self) -> Vec<HashMap<(usize, usize), Vec<usize>>> {
        let mut coord_maps: Vec<HashMap<(usize, usize), Vec<usize>>> = vec![]; //HashMap::new();
        for (n, nets) in self.nets.iter().enumerate() {
            for [i, j, k] in nets.iter() {
                let map = coord_maps.get_mut(n);
                if map.is_none() {
                    coord_maps.push(HashMap::new());
                }
                let map = coord_maps.get_mut(n).unwrap();
                let entry = map.entry((*i, *j)).or_insert(vec![]);
                entry.push(*k);
            }
        }
        // for set in coord_maps.iter() {
        //     //println!("SET");
        //     for (coord, nets) in set.iter() {
        //         //  println!("{:?} -> {:?}", coord, nets);
        //     }
        // }
        return coord_maps;
    }

    fn sort_nets_by_length(&mut self) {
        // Sort such that the last net is the shortest, we will be deleting them as we go
        self.nets.sort_by(|a, b| (&b.len()).cmp(&a.len()));
    }

    fn remove_short_nets(&mut self, lower_limit: usize) {
        // Strip out small nets
        self.nets.retain(|nets| {
            // Remove small nets
            if nets.len() > lower_limit {
                return true;
            }
            // Of the form [(i,j,4), (...), ... ]
            for [i, j, n] in nets.iter() {
                //
                let tile = self.tiles.get(*j, *i);

                if tile.is_some() {
                    let reduced_tile = tile
                        .unwrap()
                        .nodes
                        .iter()
                        .filter(|&b| {
                            let mask = 1u32 << n;
                            return mask & *b != mask;
                        })
                        .map(|b| *b)
                        .collect::<Vec<u32>>();

                    let _ = self.tiles[(*j, *i)] = Tile::new(reduced_tile, true);
                }
            }
            return false;
        });
    }

    pub(crate) fn update_network(&mut self, tile: &Tile, i: usize, j: usize) {
        let tile_nets = tile.nodes.clone();

        let mapping = NODE_TO_COORD[(i + (3 * (j % 2))) % 6];

        for net in tile_nets {
            // TODO: Need to transform this into bits rather than use node indexes
            let tile_sub_nets: HashSet<[usize; 3]> = vec![N1, N2, N3, N4, N5, N6]
                .iter()
                .filter(|&node| net & *node == *node)
                .map(|&node| [i, j, node.trailing_zeros().to_usize().unwrap()])
                .collect();

            // For this sub net of the current tile, find which nets they are part of
            let mut net_indexes: Vec<usize> = Vec::new();

            for (n, nets) in self.nets.iter().enumerate() {
                for tile_sub_net in tile_sub_nets.iter() {
                    // Find adjacent tile so we can check if it is part of an existing net
                    let node_idx = tile_sub_net[2];
                    let coord = mapping[node_idx];
                    //
                    let x = add(i, coord.0);
                    let y = add(j, coord.1);

                    match (x, y) {
                        (Some(x), Some(y)) => {
                            if nets.contains(&[x, y, node_idx]) && !net_indexes.contains(&n) {
                                net_indexes.push(n);
                                //break;
                            }
                        }
                        _ => {}
                    }
                }
            }
            if net_indexes.len() == 1 {
                // Append to an existing net
                let n = net_indexes[0];
                self.nets[n].extend(tile_sub_nets);
            } else if net_indexes.len() == 0 {
                // Add a new net
                self.nets.push(tile_sub_nets);
            } else if net_indexes.len() > 1 {
                // Since this tile's subnet was part of 2 or more nets, we can say they are really
                // the same net. So we need to merge them.

                let n = net_indexes[0];
                // Keep the net[n] and merge the other nets into it
                let r = net_indexes[1..].to_vec();

                let mut new_net = HashSet::new();
                // Add in the bridging net
                new_net.extend(tile_sub_nets);
                // Add the rest of the nets
                for i in net_indexes {
                    new_net.extend(self.nets[i].clone());
                }

                self.nets[n] = new_net; // The new merged net

                // Remove the rest of the nets since they are now merged.
                // Note: This won't have very good performance. It might be the other nets are always towards the end of the list though
                // which will help.
                for i in r.iter().rev() {
                    self.nets.remove(*i);
                }
            }
        }
    }
}

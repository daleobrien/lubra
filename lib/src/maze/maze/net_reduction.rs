use crate::maze::maze::Maze;
use crate::maze::tile::Tile;

/// Net reduction methods for the Maze struct.
/// These methods handle reducing the number of nets by merging them.
impl Maze {
    pub fn reduce_nets(&mut self, _lower_limit: usize) {
        self.sort_nets_by_length();
      // remove_short_nets might be broken ... seem to create more disjoint nets!
       // self.remove_short_nets(lower_limit);

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
            println!("\nChecking nets: (net:{m},net:{n}){:?}", intersections);

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
                        println!("Merging nets at tile {n},{m}: ({i},{j})");
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
                // for these two nets, which points do the share the same tile?
                println!("Couldn't join: (net:{m},net:{n}){:?}", intersections);
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
}

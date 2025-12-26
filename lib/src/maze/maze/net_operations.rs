use crate::maze::maze::Maze;
use crate::maze::net::{Node, Nodes};
use crate::maze::tile::Tile;
use common::consts::{N1, N2, N3, N4, N5, N6, NODE_TO_COORD};
use common::types::coordinate::add;
use rand_distr::num_traits::ToPrimitive;
use std::collections::HashSet;

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
            for tile_node in net.iter() {
                //println!("@ {}, {}, {}", tile_node.i, tile_node.j, tile_node.node);
                if visited.contains(&(tile_node.j, tile_node.i, tile_node.node)) {
                    cycle = true;
                    println!(
                        "Cycle detected @ ->: {}, {}, {}",
                        tile_node.i, tile_node.j, tile_node.node
                    );
                    break;
                }
                visited.insert((tile_node.j, tile_node.i, tile_node.node));
            }

            if cycle {
                // TODO: Break the cycles
            }
        }
    }

    fn remove_short_nets(&mut self, lower_limit: usize) {
        // Strip out small nets
        self.nets.retain(|nets| {
            // Remove small nets
            if nets.len() > lower_limit {
                return true;
            }
            // Of the form [(i,j,4), (...), ... ]
            for tile_node in nets.iter() {
                //
                let tile = self.tiles.get(tile_node.j, tile_node.i);

                if tile.is_some() {
                    let reduced_tile = tile
                        .unwrap()
                        .nodes
                        .iter()
                        .filter(|&b| {
                            let mask = 1u32 << tile_node.node;
                            return mask & *b != mask;
                        })
                        .map(|b| *b)
                        .collect::<Vec<u32>>();

                    let _ = self.tiles[(tile_node.j, tile_node.i)] = Tile::new(reduced_tile, true);
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
            let tile_sub_nets: Nodes = vec![N1, N2, N3, N4, N5, N6]
                .iter()
                .filter(|&node| net & *node == *node)
                .map(|&node| Node::new(i, j, node.trailing_zeros().to_usize().unwrap()))
                .collect();

            // For this sub net of the current tile, find which nets they are part of
            let mut net_indexes: Vec<usize> = Vec::new();

            for (n, nets) in self.nets.iter().enumerate() {
                for tile_sub_net in tile_sub_nets.iter() {
                    // Find adjacent tile so we can check if it is part of an existing net
                    let node_idx = tile_sub_net.node;
                    let coord = mapping[node_idx];
                    //
                    let x = add(i, coord.0);
                    let y = add(j, coord.1);

                    match (x, y) {
                        (Some(x), Some(y)) => {
                            if nets.contains(&Node::new(x, y, node_idx))
                                && !net_indexes.contains(&n)
                            {
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

                let mut new_net = Nodes::new();
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

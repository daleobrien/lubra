use std::fmt;

use common::bit_utils::to_6bit_binary;
use common::consts::{IN, NL, WA};
use common::nodes::get_connected_coords_at_coords;
use common::types::bits;
use common::types::coordinate::{CataloguedNet, ComplexNet, Coordinate, CoordinateAndNet};

use crate::maze::nets_adjacent::ADJ_CHECKS;
use crate::renders::fragment::MAP_PATHS;

//use std::hash::{Hash, Hasher};

#[derive(Clone)]
pub struct Tile {
    pub nodes: Vec<u32>,
    pub initialised: bool,
    pub bits: u32,
}

impl Tile {
    pub fn new(nodes: Vec<u32>, initialised: bool) -> Tile {
        let bits = nodes.iter().fold(0, |a, b| a | b);
        return Tile {
            nodes,
            initialised,
            bits,
        };
    }

    pub fn to_string_vec(self) -> String {
        if self.nodes.is_empty() {
            return "-".to_string();
        }
        let s: String = self
            .nodes
            .iter()
            .map(|n| to_6bit_binary(n, true))
            .collect::<Vec<String>>()
            .join(", ");

        return format!("[{s}]", s = s.replace("\"", ""));
    }

    pub fn generate_hidden_and_visible_nets(
        self,
        coordinate: Coordinate,
        boundaries: bool,
        surrounding_tiles: &[Option<&Tile>; 12],
    ) -> CataloguedNet {
        //

        let mut render_nets = CataloguedNet::new();

        // // ...
        let s = self
            .nodes
            .iter()
            .map(|n| to_6bit_binary(n, true))
            .collect::<Vec<String>>()
            .join(", ");
        println!("NET: {coordinate} -> {:?}", s);
        //
        for net in self.nodes {
            // Now we need to look for related tiles ...

            // This needs to return a net to use, and a list of adjacent nets to remove.
            let complex_net = Self::find_best_path(coordinate, &surrounding_tiles, &net);

            if complex_net.is_some() {
                render_nets.add(complex_net.unwrap());
            } else {
                // Simple path
                let path = MAP_PATHS.get(&net);

                // A simple tile with no adjacent tiles to worry about?
                if path.is_some() {
                    render_nets
                        .visible
                        .push(CoordinateAndNet::new(coordinate, net));
                } else {
                    println!(
                        "No path for bits: {} @ {:?}",
                        to_6bit_binary(&net, true),
                        coordinate
                    );
                }
            }
        }

        if boundaries {
            let flipped = (coordinate.j + coordinate.i) % 2 == 1;
            let bits = if flipped { WA | IN } else { WA };
            let wall = MAP_PATHS.get(&bits);
            if wall.is_some() {
                render_nets
                    .visible
                    .push(CoordinateAndNet::new(coordinate, bits));
            }
        }

        return render_nets;
    }

    fn find_best_path(
        coordinate: Coordinate,
        surrounding_tiles: &&[Option<&Tile>; 12],
        net: &u32,
    ) -> Option<ComplexNet> {
        let list_of_adj_and_nets = ADJ_CHECKS.get(&net);

        let mut complex_net: ComplexNet = ComplexNet::new();

        if list_of_adj_and_nets.is_some() {
            for &adj_and_nets in list_of_adj_and_nets.unwrap().iter() {
                //
                if Self::all_match(surrounding_tiles, adj_and_nets) {
                    // For each subnet, merge them into one net
                    let adj_net = adj_and_nets
                        .iter()
                        .map(|(_, _m, n)| n)
                        .fold(0, |a, b| a << 6 | b);

                    let new_bits = (adj_net << 6) | net;

                    println!("BIT: {}", to_6bit_binary(&new_bits, true));

                    // Swap upper bits, if needed
                    // let upper_a = (adj_net) & NL;
                    // let upper_b = (adj_net) >> 6 & NL;
                    // if upper_b != 0 && upper_a.trailing_zeros() > upper_b.trailing_zeros() {
                    //     // Keep the adj bits sorted
                    //     new_bits = (upper_a << 12) | (upper_b << 6) | net;
                    // }
                    // println!("BIT: {}", to_6bit_binary(&new_bits, true));

                    // From here on, the bits are in the correct orientation rather then logical.
                    let bits = new_bits;

                    if MAP_PATHS.contains_key(&bits) {
                        // Adjacent tiles masks
                        let adj_nets_1 = (bits >> 6) & NL;
                        let adj_nets_2 = (bits >> 12) & NL;
                        let adj_nets_3 = (bits >> 18) & NL;
                        let adj_nets_4 = (bits >> 24) & NL;

                        for adj_net in vec![adj_nets_1, adj_nets_2, adj_nets_3, adj_nets_4] {
                            if adj_net != 0 && adj_net.count_ones() == 1 {
                                Self::add_single_node_to_hidden(
                                    coordinate,
                                    &mut complex_net,
                                    adj_nets_1,
                                );
                            }
                        }

                        complex_net.complex_net = CoordinateAndNet::new(coordinate, bits);
                        return Some(complex_net);
                    }
                }
            }
        }
        return None;
    }

    fn add_single_node_to_hidden(
        coordinate: Coordinate,
        complex_net: &mut ComplexNet,
        adj_net: u32,
    ) {
        // TODO: only works correctly for 1 bit !
        let index = adj_net.trailing_zeros() as usize;
        println!(">> {} {}", index, to_6bit_binary(&adj_net, true));
        let coord = get_connected_coords_at_coords(coordinate, index);
        if coord.is_some() {
            complex_net
                .hidden_neighbours
                .insert(CoordinateAndNet::new(coord.unwrap(), adj_net));
        }
    }

    fn all_match(
        surrounding_tiles: &[Option<&Tile>; 12],
        adj_and_nets: &[(usize, bool, u32)],
    ) -> bool {
        let mut all_matched = true;

        // All the adjacent tiles must contain the net to be considered hidden.
        for (adj_idx, same, pattern) in adj_and_nets.iter() {
            let &adj_tile = &surrounding_tiles[*adj_idx];

            //
            match adj_tile {
                Some(a) => {
                    if *same != a.contain_net(*pattern) {
                        all_matched = false;
                    }
                }
                _ => {
                    all_matched = false;
                }
            }
        }
        all_matched
    }

    pub fn rotate_cw(&self) -> Vec<u32> {
        return self
            .nodes
            .iter()
            .map(|n| bits::rotate_bits_345_left_and_012_right(*n))
            .collect();
    }

    pub fn rotate_ccw(&self) -> Vec<u32> {
        return self
            .nodes
            .iter()
            .map(|n| bits::rotate_bits_345_right_and_012_left(*n))
            .collect();
    }

    pub fn intersects(&self, other: &Tile) -> bool {
        return other.bits & self.bits != 0;
    }

    pub fn contains(&self, node_mask: u32) -> bool {
        return node_mask & self.bits != 0;
    }

    pub fn contain_net(&self, net_mask: u32) -> bool {
        return self.nodes.iter().any(|net| *net == net_mask);
    }

    pub fn is_empty(&self) -> bool {
        return self.nodes.is_empty();
    }
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.initialised {
            write!(f, "{:?}", self.nodes)
        } else {
            write!(f, "uninitialised")
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.initialised {
            write!(f, "{:?}", self.nodes)
        } else {
            write!(f, "?")
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{assert_eq, println, vec};

    use common::consts::{N1, N2, N3, N4, N5, N6};

    use crate::maze::tile::Tile;

    #[test]
    fn test_cw_rotation() {
        let tile = Tile::new(vec![N1 | N2, N5], true);
        let rotated_nodes = tile.rotate_cw();
        println!("CW: {:#06b} {:#06b}", rotated_nodes[0], rotated_nodes[1]);
        assert_eq!(rotated_nodes, vec![N1 | N3, N6]);
    }

    #[test]
    fn test_ccw_rotation() {
        let tile = Tile::new(vec![N1 | N2, N5], true);
        let rotated_nodes = tile.rotate_ccw();
        assert_eq!(rotated_nodes, vec![N2 | N3, N4]);
    }

    #[test]
    fn test_bits() {
        let tile = Tile::new(vec![N1 | N2, N5], true);
        assert_eq!(tile.bits, N1 | N2 | N5)
    }

    #[test]
    fn test_intersects() {
        let tile1 = Tile::new(vec![N1 | N2, N5], true);
        let tile2 = Tile::new(vec![N2 | N3, N6], true);
        assert_eq!(tile1.intersects(&tile2), true);

        let tile1 = Tile::new(vec![N1 | N2, N5], true);
        let tile2 = Tile::new(vec![N3, N6], true);
        assert_eq!(tile1.intersects(&tile2), false);
    }

    #[test]
    fn test_contains() {
        let tile1 = Tile::new(vec![N1 | N2, N5], true);
        assert_eq!(tile1.contains(N1), true);
        assert_eq!(tile1.contains(N2), true);
        assert_eq!(tile1.contains(N3), false);
        assert_eq!(tile1.contains(N4), false);
        assert_eq!(tile1.contains(N5), true);
        assert_eq!(tile1.contains(N6), false);

        let tile1 = Tile::new(vec![], true);
        assert_eq!(tile1.contains(N1), false);
    }

    #[test]
    fn test_contain_net() {
        let tile1 = Tile::new(vec![N1 | N2, N5], true);
        assert_eq!(tile1.contain_net(N1), false);
        assert_eq!(tile1.contain_net(N2), false);
        assert_eq!(tile1.contain_net(N1 | N2), true);
        assert_eq!(tile1.contain_net(N3), false);
        assert_eq!(tile1.contain_net(N4), false);
        assert_eq!(tile1.contain_net(N5), true);
        assert_eq!(tile1.contain_net(N6), false);

        let tile1 = Tile::new(vec![], true);
        assert_eq!(tile1.contains(N1), false);
    }

    #[test]
    fn test_is_empty() {
        let tile1 = Tile::new(vec![N1 | N2, N5], true);
        assert_eq!(tile1.is_empty(), false);

        let tile1 = Tile::new(vec![], true);
        assert_eq!(tile1.is_empty(), true);
    }
}

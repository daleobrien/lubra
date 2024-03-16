use crate::maze::nets_adjacent::ADJ_CHECKS;
use crate::renders::fragment::MAP_PATHS;
use common::bit_utils::to_6bit_binary;
use common::consts::{IN, WA};
use common::nodes::get_connected_coords_at_coords;
use common::types::bits;
use common::types::coordinate::{CataloguedNet, ComplexNet, Coordinate, CoordinateAndNet};
use std::fmt;
use std::hash::{Hash, Hasher};

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
            .map(|n| to_6bit_binary(n))
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
        let flipped = (coordinate.j + coordinate.i) % 2 == 1;

        // println!(
        //     "\n\nRender Tile: {:?} @ {:?}",
        //     coordinate,
        //     self.clone().to_string_vec()
        // );

        let index = coordinate.i % 3;

        let mut render_nets = CataloguedNet::new();

        for net in self.nodes {
            // println!("net: {} / {net}", to_6bit_binary(&net));
            // Now we need to look for related tiles ...

            // This needs to return a net to use, and a list of adjacent nets to remove.
            let complex_net =
                Self::find_best_path(coordinate, &surrounding_tiles, flipped, index, &net);

            if complex_net.is_some() {
                render_nets.add(complex_net.unwrap());
            } else {
                // Simple path
                let bits = bits::transform_tile_orientation(net, index, flipped);
                let path = MAP_PATHS.get(&bits);

                // A simple tile with no adjacent tiles to worry about?
                if path.is_some() {
                    render_nets
                        .visible
                        .push(CoordinateAndNet::new(coordinate, bits));
                } else {
                    println!(
                        "No path for bits: {} @ {:?}",
                        to_6bit_binary(&net),
                        coordinate
                    );
                }
            }
        }

        if boundaries {
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
        flipped: bool,
        index: usize,
        net: &u32,
    ) -> Option<ComplexNet> {
        let list_of_adj_and_nets = ADJ_CHECKS.get(&net);

        let mut complex_net: ComplexNet = ComplexNet::new();

        if list_of_adj_and_nets.is_some() {
            for &adj_and_nets in list_of_adj_and_nets.unwrap().iter() {
                if Self::all_match(surrounding_tiles, adj_and_nets) {
                    let adj_net = adj_and_nets
                        .iter()
                        .map(|(_, _m, n)| n)
                        .fold(0, |a, b| a | b);

                    let new_bits = (adj_net << 6) | net;
                    let bits = bits::transform_tile_orientation(new_bits, index, flipped);
                    if MAP_PATHS.contains_key(&bits) {
                        // Figure out the adjacent nets to remove.
                        /*
                          let mut remove_nets: Vec<u32> = vec![];
                        */
                        let adj_nets_1 = (bits >> 6) & 0b111_111;
                        //let adj_nets_2 = (bits >> 12) & 0b111_111;

                        match adj_nets_1.count_ones() {
                            0 => { // no need to remove any nets
                            }
                            1 => {
                                // Always remove the adj_nets_1 in this case
                                match adj_nets_1 {
                                    0b000_001 => {
                                        let coord = get_connected_coords_at_coords(coordinate, 0);
                                        complex_net
                                            .hidden_neighbours
                                            .insert(CoordinateAndNet::new(coord, 0b000_001));
                                    }
                                    0b000_010 => {
                                        // remove
                                        let coord = get_connected_coords_at_coords(coordinate, 1);
                                        complex_net
                                            .hidden_neighbours
                                            .insert(CoordinateAndNet::new(coord, 0b000_010));
                                    }
                                    0b000_100 => {
                                        // remove
                                        let coord = get_connected_coords_at_coords(coordinate, 2);
                                        complex_net
                                            .hidden_neighbours
                                            .insert(CoordinateAndNet::new(coord, 0b000_100));
                                    }
                                    0b001_000 => {
                                        // remove
                                        let coord = get_connected_coords_at_coords(coordinate, 3);
                                        complex_net
                                            .hidden_neighbours
                                            .insert(CoordinateAndNet::new(coord, 0b001_000));
                                    }
                                    0b010_000 => {
                                        // remove
                                        let coord = get_connected_coords_at_coords(coordinate, 4);
                                        complex_net
                                            .hidden_neighbours
                                            .insert(CoordinateAndNet::new(coord, 0b010_000));
                                    }
                                    0b100_000 => {
                                        // remove
                                        let coord = get_connected_coords_at_coords(coordinate, 5);
                                        complex_net
                                            .hidden_neighbours
                                            .insert(CoordinateAndNet::new(coord, 0b100_000));
                                    }
                                    _ => {}
                                }
                            }
                            2 => {}
                            _ => {
                                // remove the adj_nets_1
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

    // fn can_be_hidden(surrounding_tiles: &[Option<&Tile>; 12], net: &u32) -> bool {
    //     // First check if the tile should be blanked out, i.e. adjacent will render.
    //     let list_of_adj_and_nets = BLANK_CHECK_PATHS.get(&net);
    //
    //     if list_of_adj_and_nets.is_some() {
    //         for &adj_and_nets in list_of_adj_and_nets.unwrap().iter() {
    //             if Self::all_match(surrounding_tiles, adj_and_nets) {
    //                 return true;
    //             }
    //         }
    //     }
    //     return false;
    // }

    fn all_match(
        surrounding_tiles: &[Option<&Tile>; 12],
        adj_and_nets: &[(usize, bool, u32)],
    ) -> bool {
        let mut all_matched = true;

        // All the adjacent tiles must contain the net to be considered hidden.
        for (adj_idx, same, pattern) in adj_and_nets.iter() {
            let &adj_tile = &surrounding_tiles[*adj_idx];

            // println!(
            //     "<{}> -> {:?} / {:?} ::: {:?}",
            //     adj_idx, same, pattern, adj_tile
            // );
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
    use crate::maze::tile::Tile;
    use std::{assert_eq, println, vec};

    #[test]
    fn test_cw_rotation() {
        let tile = Tile::new(vec![0b000_011, 0b010_000], true);
        let rotated_nodes = tile.rotate_cw();
        println!("CW: {:#06b} {:#06b}", rotated_nodes[0], rotated_nodes[1]);
        assert_eq!(rotated_nodes, vec![0b000_101, 0b100_000]);
    }

    #[test]
    fn test_ccw_rotation() {
        let tile = Tile::new(vec![0b000_011, 0b010_000], true);
        let rotated_nodes = tile.rotate_ccw();
        assert_eq!(rotated_nodes, vec![0b000_110, 0b001_000]);
    }

    #[test]
    fn test_bits() {
        let tile = Tile::new(vec![0b000_011, 0b010_000], true);
        assert_eq!(tile.bits, 0b010011)
    }

    #[test]
    fn test_intersects() {
        let tile1 = Tile::new(vec![0b000_011, 0b010_000], true);
        let tile2 = Tile::new(vec![0b000_110, 0b100_000], true);
        assert_eq!(tile1.intersects(&tile2), true);

        let tile1 = Tile::new(vec![0b000_011, 0b010_000], true);
        let tile2 = Tile::new(vec![0b000_100, 0b100_000], true);
        assert_eq!(tile1.intersects(&tile2), false);
    }

    #[test]
    fn test_contains() {
        let tile1 = Tile::new(vec![0b000_011, 0b010_000], true);
        assert_eq!(tile1.contains(0b000_001), true);
        assert_eq!(tile1.contains(0b000_010), true);
        assert_eq!(tile1.contains(0b000_100), false);
        assert_eq!(tile1.contains(0b001_000), false);
        assert_eq!(tile1.contains(0b010_000), true);
        assert_eq!(tile1.contains(0b100_000), false);

        let tile1 = Tile::new(vec![], true);
        assert_eq!(tile1.contains(0b000_001), false);
    }
    #[test]
    fn test_contain_net() {
        let tile1 = Tile::new(vec![0b000_011, 0b010_000], true);
        assert_eq!(tile1.contain_net(0b000_001), false);
        assert_eq!(tile1.contain_net(0b000_010), false);
        assert_eq!(tile1.contain_net(0b000_011), true);
        assert_eq!(tile1.contain_net(0b000_100), false);
        assert_eq!(tile1.contain_net(0b001_000), false);
        assert_eq!(tile1.contain_net(0b010_000), true);
        assert_eq!(tile1.contain_net(0b100_000), false);

        let tile1 = Tile::new(vec![], true);
        assert_eq!(tile1.contains(0b000_001), false);
    }

    #[test]
    fn test_is_empty() {
        let tile1 = Tile::new(vec![0b000_011, 0b010_000], true);
        assert_eq!(tile1.is_empty(), false);

        let tile1 = Tile::new(vec![], true);
        assert_eq!(tile1.is_empty(), true);
    }
}

use crate::maze::nets_adjacent::ADJ_CHECKS;
use crate::renders::_point::Coordinate;
use crate::renders::fragment::MAP_PATHS;
use common::bit_utils::to_6bit_binary;
use common::consts::{IN, WA};
use rand_distr::num_traits::Signed;
use std::fmt;

#[derive(Clone)]
pub struct Tile {
    pub nodes: Vec<u32>,
    pub initialised: bool,
    pub bits: u32,
}

fn rotate_bits_345_left_and_012_right(value: u32) -> u32 {
    // Extract the bits to be rotated.
    let bits_345_1 = value >> 2 & 0b001_000_001_000_001_000;
    let bits_345_2 = value << 1 & 0b110_000_110_000_110_000;
    let bits_012_1 = value << 2 & 0b000_100_000_100_000_100;
    let bits_012_2 = value >> 1 & 0b000_011_000_011_000_011;
    return bits_345_1 | bits_345_2 | bits_012_1 | bits_012_2;
}

fn rotate_bits_345_right_and_012_left(value: u32) -> u32 {
    let bits_345_1 = value >> 1 & 0b011_000_011_000_011_000;
    let bits_345_2 = value << 2 & 0b100_000_100_000_100_000;
    let bits_012_1 = value << 1 & 0b000_110_000_110_000_110;
    let bits_012_2 = value >> 2 & 0b000_001_000_001_000_001;
    return bits_345_1 | bits_345_2 | bits_012_1 | bits_012_2;
}

fn transform_tile_orientation(value: u32, index: usize, flipped: bool) -> u32 {
    let rotated_net = match index {
        0 => value,
        1 => rotate_bits_345_right_and_012_left(value),
        2 => rotate_bits_345_left_and_012_right(value),
        _ => panic!("Invalid rotation"),
    };
    let bits: u32 = if flipped {
        IN | rotated_net
    } else {
        rotated_net
    };
    return bits;
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

    pub fn to_svg_groups(
        self,
        coordinate: Coordinate,
        boundaries: bool,
        surrounding_tiles: &[Option<&Tile>; 12],
    ) -> Vec<&'static str> {
        // Build the SVG groups and return them
        let mut groups: Vec<&str> = vec![];

        let flipped = (coordinate.j + coordinate.i) % 2 == 1;

        println!(
            "\n\nRender Tile: {:?} @ {:?}",
            coordinate,
            self.clone().to_string_vec()
        );

        let index = coordinate.i % 3;

        // i,j = nets
        //let mut current_nets: Vec<(u32, u32, u32)> = vec![];

        for net in self.nodes {
            println!("net: {} / {net}", to_6bit_binary(&net));
            // Now we need to look for related tiles ...

            // This needs to return a net to use, and a list of adjacent nets to remove.
            let complex_bits = Self::find_best_path(&surrounding_tiles, flipped, index, &net);

            if complex_bits.is_some() {
                groups.push(MAP_PATHS.get(&complex_bits.unwrap()).unwrap());
                // TODO: Need to remove any neighbouring tiles that no longer needed.
                //
                // We could add a new `duplicate` set that are used say it's not to be drawn.
                // Thus when we add a complex tile, we can add the adjacent tiles to duplicate set.
                // We really only need to store the coordinate of the tile, and the net.
                // We could also do a two pass system, a 2nd pass to remove any tiles that are not needed.
                // ...
            } else {
                // Simple path
                let bits = transform_tile_orientation(net, index, flipped);
                let path = MAP_PATHS.get(&bits);

                // A simple tile with no adjacent tiles to worry about?
                if path.is_some() {
                    groups.push(path.unwrap());
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
                groups.push(wall.unwrap());
            }
        }

        return groups;
    }

    fn find_best_path(
        surrounding_tiles: &&[Option<&Tile>; 12],
        flipped: bool,
        index: usize,
        net: &u32,
    ) -> Option<u32> {
        let list_of_adj_and_nets = ADJ_CHECKS.get(&net);

        if list_of_adj_and_nets.is_some() {
            for &adj_and_nets in list_of_adj_and_nets.unwrap().iter() {
                if Self::all_match(surrounding_tiles, adj_and_nets) {
                    let adj_net = adj_and_nets
                        .iter()
                        .map(|(_, _m, n)| n)
                        .fold(0, |a, b| a | b);

                    let new_bits = (adj_net << 6) | net;
                    let bits = transform_tile_orientation(new_bits, index, flipped);
                    if MAP_PATHS.contains_key(&bits) {
                        return Some(bits);
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
            .map(|n| rotate_bits_345_left_and_012_right(*n))
            .collect();
    }

    pub fn rotate_ccw(&self) -> Vec<u32> {
        return self
            .nodes
            .iter()
            .map(|n| rotate_bits_345_right_and_012_left(*n))
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

pub fn add(u: usize, i: i32) -> Option<usize> {
    if i.is_negative() {
        if u < i.wrapping_abs() as u32 as usize {
            return None;
        }
        let r = u - i.wrapping_abs() as u32 as usize;
        return Some(r);
    }
    let r = u + i as usize;
    return Some(r);
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

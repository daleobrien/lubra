use common::consts::{ADJ_4, ADJ_5, NODE_1, NODE_4, NODE_5};

use crate::maze::adjacent::{print_surrounding, ADJ_N5};
use crate::maze::nets_adjacent::ADJ_CHECKS;
use crate::renders::_point::Coordinate;
use crate::renders::fragment::MAP_PATHS;
use std::fmt;

#[derive(Clone)]
pub struct Tile {
    pub nodes: Vec<u32>,
    pub initialised: bool,
    pub bits: u32,
}

#[cfg(debug_assertions)]
fn pretty_print_int(b: &u32) -> String {
    let number_as_string = format!("{:08b}", *b);
    let mut s = String::new();
    for (idx, val) in number_as_string.chars().rev().enumerate() {
        if idx != 0 && idx % 3 == 0 {
            s.insert(0, '_');
        }
        s.insert(0, val);
    }
    return s;
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

    pub fn to_string_vec(self) -> Vec<String> {
        if self.nodes.is_empty() {
            return vec!["-".to_string()];
        }
        self.nodes
            .iter()
            .map(|n| format!("{:?}", pretty_print_int(n)))
            .collect::<Vec<String>>()
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

        let mut bits = 0b10_000_000;
        if flipped {
            bits |= 0b01_000_000;
        }

        // println!(
        //     "Tile: {:?} @ {:?}",
        //     coordinate,
        //     self.clone().to_string_vec()
        // );

        let index = coordinate.i % 3;
        let nodes = match index {
            0 => self.nodes,
            1 => self.rotate_ccw(),
            2 => self.rotate_cw(),
            _ => panic!("Invalid rotation"),
        };

        for net in nodes {
            let bits: u32 = if flipped { 0b1_000_000 | net } else { net };

            let path = MAP_PATHS.get(&bits);
            if path.is_some() {
                // TODO: Check if we don't want to render this!
                // Could be a simple lookup?
                groups.push(path.unwrap());
            } else {
                println!(
                    "Missed ({i},{j}) {:06b}",
                    net,
                    i = coordinate.i,
                    j = coordinate.j
                );
                // Now we need to look for related tiles ...
                match ADJ_CHECKS.get(&net) {
                    Some(&(adj_idx, posibles_nets)) => {
                        // TODO: We need to check is this is rendered? or is it always rendered!
                        let &adj_nets = &surrounding_tiles[*adj_idx];
                        print_surrounding(surrounding_tiles);
                        println!("adj >>> {adj_nets:?}");

                        match adj_nets {
                            None => {}
                            Some(adj_tile) => {
                                for adj_net in *posibles_nets {
                                    if adj_tile.contain_net(*adj_net) {
                                        let mut bits: u32 = (adj_net << 8) | net;
                                        if flipped {
                                            bits |= 0b1_000_000;
                                        }
                                        let path = MAP_PATHS.get(&bits);
                                        if path.is_some() {
                                            groups.push(path.unwrap());
                                        }
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    None => {
                        println!("No path for bits: {:016b} @ {:?}", net, coordinate);
                    }
                }
            }
        }

        if boundaries {
            let wall = MAP_PATHS.get(&bits);
            if wall.is_some() {
                groups.push(wall.unwrap());
            }
        }

        return groups;
    }

    fn rotate_bits_345_left_and_012_right(value: u32) -> u32 {
        // Extract the bits to be rotated.
        let bits_345 = (value & 0b00111000) >> 3;
        let bits_012 = value & 0b00000111;
        let rotated_bits_345 = ((bits_345 << 1) | (bits_345 >> 2)) & 0b00000111;
        let rotated_bits_012 = ((bits_012 >> 1) | (bits_012 << 2)) & 0b00000111;
        return (rotated_bits_345 << 3) | rotated_bits_012;
    }

    pub fn rotate_cw(&self) -> Vec<u32> {
        return self
            .nodes
            .iter()
            .map(|n| Tile::rotate_bits_345_left_and_012_right(*n))
            .collect();
    }

    fn rotate_bits_345_right_and_012_left(value: u32) -> u32 {
        let bits_345 = (value & 0b00111000) >> 3;
        let bits_012 = value & 0b00000111;
        let rotated_bits_345 = ((bits_345 >> 1) | (bits_345 << 2)) & 0b00000111;
        let rotated_bits_012 = ((bits_012 << 1) | (bits_012 >> 2)) & 0b00000111;
        return (rotated_bits_345 << 3) | rotated_bits_012;
    }

    pub fn rotate_ccw(&self) -> Vec<u32> {
        return self
            .nodes
            .iter()
            .map(|n| Tile::rotate_bits_345_right_and_012_left(*n))
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

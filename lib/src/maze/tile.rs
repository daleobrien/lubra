use crate::renders::_point::Coordinate;
use crate::renders::fragment::MAP_PATHS;
use std::fmt;

fn rotate_number_up(n: u32) -> u32 {
    return match n {
        1 => 3,
        2 => 1,
        3 => 2,
        4 => 5,
        5 => 6,
        6 => 4,
        _ => 0,
    };
}

fn rotate_number_dn(n: u32) -> u32 {
    return match n {
        1 => 2,
        2 => 3,
        3 => 1,
        4 => 6,
        5 => 4,
        6 => 5,
        _ => 0,
    };
}

#[derive(Clone)]
pub struct Tile {
    pub nodes: Vec<u32>,
    pub initialised: bool,
}

impl Tile {
    pub fn new(nodes: Vec<u32>, initialised: bool) -> Tile {
        Tile { nodes, initialised }
    }

    pub fn to_svg_groups(self, coordinate: Coordinate, boundaries: bool) -> Vec<&'static str> {
        // Build the SVG groups and return them
        let mut groups: Vec<&str> = vec![];

        let flipped = (coordinate.j + coordinate.i) % 2 == 1;

        let mut bits = 0b10_000_000;
        if flipped {
            bits |= 0b01_000_000;
        }

        let index = coordinate.i % 3;
        let nodes = match index {
            0 => self.nodes,
            1 => self.rotate_ccw(),
            2 => self.rotate_cw(),
            _ => panic!("Invalid rotation"),
        };

        for net in nodes {
            let bits: u32 = if flipped {
                0b1_000_000 | net.clone()
            } else {
                net
            };

            let path = MAP_PATHS.get(&bits);
            if path.is_some() {
                groups.push(path.unwrap());
            } else {
                println!(
                    "No path for bits: {:06b} / {:?} @ {:?}",
                    bits, net, coordinate
                );
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

    pub fn bits(&self) -> u32 {
        return self.nodes.iter().fold(0, |a, b| a | b);
    }

    fn rotate_bits_345_left_and_012_right(value: u32) -> u32 {
        // Extract the bits to be rotated.
        let bits_345 = (value & 0b00111000) >> 3;
        let bits_012 = value & 0b00000111;

        // Perform left rotation on bits 3, 4, 5.
        let rotated_bits_345 = ((bits_345 << 1) | (bits_345 >> 2)) & 0b00000111;

        // Perform right rotation on bits 0, 1, 2.
        let rotated_bits_012 = ((bits_012 >> 1) | (bits_012 << 2)) & 0b00000111;

        // Combine the rotated bits with the original value.
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
        // Extract the bits to be rotated.
        let bits_345 = (value & 0b00111000) >> 3;
        let bits_012 = value & 0b00000111;

        // Perform right rotation on bits 3, 4, 5.
        let rotated_bits_345 = ((bits_345 >> 1) | (bits_345 << 2)) & 0b00000111;

        // Perform left rotation on bits 0, 1, 2.
        let rotated_bits_012 = ((bits_012 << 1) | (bits_012 >> 2)) & 0b00000111;

        // Combine the rotated bits with the original value.
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
        let other_bits = other.bits();
        let bits = self.bits();

        if other_bits & bits != 0 {
            return true;
        }
        return false;
    }

    pub fn contains(&self, node_index: u32) -> bool {
        return self.nodes.iter().any(|net| *net & node_index != 0u32);
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
            write!(f, "-")
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
        assert_eq!(tile.bits(), 0b010011)
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
    fn test_is_empty() {
        let tile1 = Tile::new(vec![0b000_011, 0b010_000], true);
        assert_eq!(tile1.is_empty(), false);

        let tile1 = Tile::new(vec![], true);
        assert_eq!(tile1.is_empty(), true);
    }
}

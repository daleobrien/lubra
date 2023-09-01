use crate::renders::_point::Coordinate;
use crate::renders::fragment::MAP_PATHS;
use std::fmt;

fn rotate_number_up(n: u8) -> u8 {
    return match n {
        1 => 2,
        2 => 3,
        3 => 1,
        4 => 5,
        5 => 6,
        6 => 4,
        _ => 0,
    };
}

fn rotate_number_dn(n: u8) -> u8 {
    return match n {
        1 => 3,
        2 => 1,
        3 => 2,
        4 => 6,
        5 => 4,
        6 => 5,
        _ => 0,
    };
}

#[derive(Clone)]
pub struct Tile {
    pub nodes: Vec<Vec<u8>>,
    pub initialised: bool,
}

impl Tile {
    pub fn new(nodes: Vec<Vec<u8>>, initialised: bool) -> Tile {
        Tile { nodes, initialised }
    }

    pub fn to_svg_groups(self, coordinate: Coordinate) -> Vec<&'static str> {
        // Build the SVG groups and return them
        let mut groups: Vec<&str> = vec![];

        let flipped = (coordinate.j + coordinate.i) % 2 == 1;

        // ADD walls
        let mut bits = 0b10_000_000;
        if flipped {
            bits |= 0b01_000_000;
        }
        let wall = MAP_PATHS.get(&bits);
        if wall.is_some() {
            groups.push(wall.unwrap());
        }

        let index = coordinate.i % 3;
        let nodes = match index {
            0 => self.nodes,
            1 => self.rotate_ccw(),
            2 => self.rotate_cw(),
            _ => panic!("Invalid rotation"),
        };

        for net in nodes {
            let mut bits: u8 = 0;
            for n in net.clone() {
                bits |= 1 << (n - 1);
            }

            if flipped {
                bits |= 0b1_000_000;
            }

            let path = MAP_PATHS.get(&bits);
            if path.is_some() {
                groups.push(path.unwrap());
            } else {
                println!(
                    "No path for bits: {:b} / {:?} @ {:?}",
                    bits, net, coordinate
                );
            }
        }

        return groups;
    }

    pub fn bits(&self) -> u32 {
        let mut bits = 0;
        for nets in self.nodes.clone() {
            for n in nets {
                bits |= 1 << (n - 1);
            }
        }
        return bits;
    }

    pub fn rotate_cw(&self) -> Vec<Vec<u8>> {
        let mut nodes = Vec::new();
        for nets in self.nodes.clone() {
            let mut new_nets = Vec::new();
            for n in nets {
                new_nets.push(rotate_number_up(n.clone()));
            }
            new_nets.sort();
            nodes.push(new_nets);
        }
        //nodes.sort();
        return nodes;

        // return self
        //     .nodes
        //     .iter()
        //     .map(|row| row.iter().map(|n| rotate_number_up(*n)).sorted().collect())
        //     .collect();
    }
    pub fn rotate_ccw(&self) -> Vec<Vec<u8>> {
        let mut nodes = Vec::new();
        for nets in self.nodes.clone() {
            let mut new_nets = Vec::new();
            for n in nets {
                new_nets.push(rotate_number_dn(n.clone()));
            }
            new_nets.sort();
            nodes.push(new_nets);
        }
        //nodes.sort();
        return nodes;
    }

    pub fn intersects(&self, other: &Tile) -> bool {
        let other_bits = other.bits();
        let bits = self.bits();

        if other_bits & bits != 0 {
            return true;
        }
        return false;
    }

    pub fn contains(&self, node_index: u8) -> bool {
        for nets in self.nodes.clone() {
            for n in nets {
                if n == node_index {
                    return true;
                }
            }
        }
        return false;
    }

    pub fn is_empty(&self) -> bool {
        return self.nodes.len() == 0;
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
        let tile = Tile::new(vec![vec![1, 2], vec![5]], true);
        let rotated_nodes = tile.rotate_cw();
        assert_eq!(rotated_nodes, vec![vec![2, 3], vec![6]]);
    }

    #[test]
    fn test_ccw_rotation() {
        let tile = Tile::new(vec![vec![1, 2], vec![5]], true);
        let rotated_nodes = tile.rotate_ccw();
        assert_eq!(rotated_nodes, vec![vec![1, 3], vec![4]]);
    }

    #[test]
    fn test_bits() {
        let tile = Tile::new(vec![vec![1, 2], vec![5]], true);
        assert_eq!(tile.bits(), 0b010011)
    }

    #[test]
    fn test_intersects() {
        let tile1 = Tile::new(vec![vec![1, 2], vec![5]], true);
        let tile2 = Tile::new(vec![vec![2, 3], vec![6]], true);
        assert_eq!(tile1.intersects(&tile2), true);

        let tile1 = Tile::new(vec![vec![1, 2], vec![5]], true);
        let tile2 = Tile::new(vec![vec![3], vec![6]], true);
        assert_eq!(tile1.intersects(&tile2), false);
    }

    #[test]
    fn test_contains() {
        let tile1 = Tile::new(vec![vec![1, 2], vec![5]], true);
        assert_eq!(tile1.contains(1), true);
        assert_eq!(tile1.contains(2), true);
        assert_eq!(tile1.contains(3), false);
        assert_eq!(tile1.contains(4), false);
        assert_eq!(tile1.contains(5), true);
        assert_eq!(tile1.contains(6), false);

        let tile1 = Tile::new(vec![], true);
        assert_eq!(tile1.contains(1), false);
    }

    #[test]
    fn test_is_empty() {
        let tile1 = Tile::new(vec![vec![1, 2], vec![5]], true);
        assert_eq!(tile1.is_empty(), false);

        let tile1 = Tile::new(vec![], true);
        assert_eq!(tile1.is_empty(), true);
    }
}

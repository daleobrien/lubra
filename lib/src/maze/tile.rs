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
}

impl Tile {
    pub fn new(nodes: Vec<Vec<u8>>) -> Tile {
        Tile { nodes }
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

pub fn add(u: usize, i: i32) -> usize {
    if i.is_negative() {
        u - i.wrapping_abs() as u32 as usize
    } else {
        u + i as usize
    }
}

#[cfg(test)]
mod tests {
    use crate::maze::tile::Tile;

    #[test]
    fn test_cw_rotation() {
        let tile = Tile::new(vec![vec![1, 2], vec![5]]);
        let rotated_nodes = tile.rotate_cw();
        assert_eq!(rotated_nodes, vec![vec![2, 3], vec![6]]);
    }

    #[test]
    fn test_ccw_rotation() {
        let tile = Tile::new(vec![vec![1, 2], vec![5]]);
        let rotated_nodes = tile.rotate_ccw();
        assert_eq!(rotated_nodes, vec![vec![1, 3], vec![4]]);
    }

    #[test]
    fn test_bits() {
        let tile = Tile::new(vec![vec![1, 2], vec![5]]);
        assert_eq!(tile.bits(), 0b010011)
    }

    #[test]
    fn test_intersects() {
        let tile1 = Tile::new(vec![vec![1, 2], vec![5]]);
        let tile2 = Tile::new(vec![vec![2, 3], vec![6]]);
        assert_eq!(tile1.intersects(&tile2), true);

        let tile1 = Tile::new(vec![vec![1, 2], vec![5]]);
        let tile2 = Tile::new(vec![vec![3], vec![6]]);
        assert_eq!(tile1.intersects(&tile2), false);
    }

    #[test]
    fn test_contains() {
        let tile1 = Tile::new(vec![vec![1, 2], vec![5]]);
        assert_eq!(tile1.contains(1), true);
        assert_eq!(tile1.contains(2), true);
        assert_eq!(tile1.contains(3), false);
        assert_eq!(tile1.contains(4), false);
        assert_eq!(tile1.contains(5), true);
        assert_eq!(tile1.contains(6), false);

        let tile1 = Tile::new(vec![]);
        assert_eq!(tile1.contains(1), false);
    }

    #[test]
    fn test_is_empty() {
        let tile1 = Tile::new(vec![vec![1, 2], vec![5]]);
        assert_eq!(tile1.is_empty(), false);

        let tile1 = Tile::new(vec![]);
        assert_eq!(tile1.is_empty(), true);
    }
}

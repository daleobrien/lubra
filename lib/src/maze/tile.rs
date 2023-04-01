use crate::renders::primatives::{Line, Point};
use array2d::Array2D;
use bitflags::bitflags;
use serde::ser::{Serialize, Serializer};

use crate::renders::tile_subsets::random_side;

bitflags! {
  #[derive(Default, serde::Deserialize)]
  pub struct Tile: u32 {

    //              XXX XXX
    const G1 = 0b00_000_001_000_000_000_000_000_000_000_000;
    const G2 = 0b00_000_010_000_000_000_000_000_000_000_000;
    const G3 = 0b00_000_100_000_000_000_000_000_000_000_000;
    const G4 = 0b00_001_000_000_000_000_000_000_000_000_000;
    const G5 = 0b00_010_000_000_000_000_000_000_000_000_000;
    const G6 = 0b00_100_000_000_000_000_000_000_000_000_000;
    const GX = 0b00_111_111_000_000_000_000_000_000_000_000;

    //                      XXX
    const Z1 = 0b00_000_000_000_000_000_000_000_000_000_000; // Empty
    const Z2 = 0b00_000_000_010_000_000_000_000_000_000_000; // Not routed
    const W1 = 0b00_000_000_001_000_000_000_000_000_000_000; // Walls only, used to show the grid
    //                          XXX
    const X1 = 0b00_111_000_000_001_000_000_000_000_000_000;
    const X2 = 0b00_111_111_000_010_000_000_000_000_000_000;
    //                              XXX
    const Y1 = 0b00_011_110_000_000_010_000_000_000_100_000;
    const Y2 = 0b00_101_101_000_000_100_000_000_000_001_000;
    const Y3 = 0b00_110_011_000_000_001_000_000_000_010_000;
    //                                  XXX             XXX
    const C1 = 0b00_111_001_000_000_000_001_000_000_000_001;
    const C2 = 0b00_111_010_000_000_000_010_000_000_000_010;
    const C3 = 0b00_111_100_000_000_000_100_000_000_000_100;
    //                                      XXX
    const T1 = 0b00_100_000_000_000_000_000_001_000_000_000;
    const T2 = 0b00_010_000_000_000_000_000_010_000_000_000;
    const T3 = 0b00_001_000_000_000_000_000_100_000_000_000;
    //                                          XXX
    const N1 = 0b00_000_001_000_000_000_000_000_001_000_000;
    const N2 = 0b00_000_010_000_000_000_000_000_010_000_000;
    const N3 = 0b00_000_100_000_000_000_000_000_100_000_000;
    //                                              XXX
    const L1 = 0b00_100_001_000_000_000_000_000_000_001_000;
    const L2 = 0b00_010_010_000_000_000_000_000_000_010_000;
    const L3 = 0b00_001_100_000_000_000_000_000_000_100_000;
    const LX = 0b00_111_111_000_000_000_000_000_000_111_000;
    //                                                  XXX
    const A1 = 0b00_011_000_000_000_000_000_000_000_000_001;
    const A2 = 0b00_101_000_000_000_000_000_000_000_000_010;
    const A3 = 0b00_110_000_000_000_000_000_000_000_000_100;
    const AX = 0b00_111_000_000_000_000_000_000_000_000_111;

  }
}

impl Tile {
    pub fn get_nets(&self) -> Vec<Vec<usize>> {
        let mut nodes = Vec::new();
        match *self {
            Tile::X1 => nodes.push(vec![4, 5, 6]), // All edges connect to each other
            Tile::X2 => {
                nodes.push(vec![1, 6]); // Each edge and it's opposite point connect to each other
                nodes.push(vec![2, 5]);
                nodes.push(vec![3, 4]);
            }
            Tile::A1 => nodes.push(vec![4, 5]),
            Tile::A2 => nodes.push(vec![4, 6]),
            Tile::A3 => nodes.push(vec![5, 6]),
            Tile::L1 => nodes.push(vec![1, 6]),
            Tile::L2 => nodes.push(vec![2, 5]),
            Tile::L3 => nodes.push(vec![3, 4]),
            Tile::N1 => nodes.push(vec![1]),
            Tile::N2 => nodes.push(vec![2]),
            Tile::N3 => nodes.push(vec![3]),
            Tile::T1 => nodes.push(vec![6]),
            Tile::T2 => nodes.push(vec![5]),
            Tile::T3 => nodes.push(vec![4]),
            _ => {}
        }
        return nodes;
    }

    pub fn rotate_cw(&self) -> Tile {
        let mut rotated_tile = Self::Z1;

        if self.contains(Self::W1) {
            rotated_tile |= Self::W1;
        }
        if self.contains(Self::X1) {
            rotated_tile |= Self::X1;
        }
        if self.contains(Self::X2) {
            rotated_tile |= Self::X2;
        }

        for (a, b) in [
            (Self::Y1, Self::Y2),
            (Self::Y2, Self::Y3),
            (Self::Y3, Self::Y1),
            (Self::A1, Self::A2),
            (Self::A2, Self::A3),
            (Self::A3, Self::A1),
            (Self::C1, Self::C2),
            (Self::C2, Self::C3),
            (Self::C3, Self::C1),
            (Self::L1, Self::L2),
            (Self::L2, Self::L3),
            (Self::L3, Self::L1),
            (Self::N1, Self::N2),
            (Self::N2, Self::N3),
            (Self::N3, Self::N1),
            (Self::T1, Self::T2),
            (Self::T2, Self::T3),
            (Self::T3, Self::T1),
        ] {
            if self.contains(a) {
                rotated_tile |= b;
            }
        }
        return rotated_tile;
    }
    pub fn rotate_ccw(&self) -> Tile {
        let mut rotated_tile = Self::Z1;

        if self.contains(Self::W1) {
            rotated_tile |= Self::W1;
        }
        if self.contains(Self::X1) {
            rotated_tile |= Self::X1;
        }
        if self.contains(Self::X2) {
            rotated_tile |= Self::X2;
        }
        for (a, b) in [
            (Self::Y1, Self::Y3),
            (Self::Y2, Self::Y1),
            (Self::Y3, Self::Y2),
            (Self::A1, Self::A3),
            (Self::A2, Self::A1),
            (Self::A3, Self::A2),
            (Self::C1, Self::C3),
            (Self::C2, Self::C1),
            (Self::C3, Self::C2),
            (Self::L1, Self::L3),
            (Self::L2, Self::L1),
            (Self::L3, Self::L2),
            (Self::N1, Self::N3),
            (Self::N2, Self::N1),
            (Self::N3, Self::N2),
            (Self::T1, Self::T3),
            (Self::T2, Self::T1),
            (Self::T3, Self::T2),
        ] {
            if self.contains(a) {
                rotated_tile |= b;
            }
        }
        return rotated_tile;
    }
}
impl Serialize for Tile {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u32(self.bits)
    }
}

pub fn add(u: usize, i: i32) -> usize {
    if i.is_negative() {
        u - i.wrapping_abs() as u32 as usize
    } else {
        u + i as usize
    }
}

impl From<[u32; 3]> for Tile {
    fn from(masks: [u32; 3]) -> Tile {
        let [point_mask, line_mask, curve_mask] = masks;

        // Nx and Tx
        let mut value = Tile::from_bits_truncate(0);

        if point_mask & 0b000_001 == 0b000_001 {
            value |= Tile::N1;
        };
        if point_mask & 0b000_010 == 0b000_010 {
            value |= Tile::N2;
        };
        if point_mask & 0b000_100 == 0b000_100 {
            value |= Tile::N3;
        };

        if point_mask & 0b100_000 == 0b100_000 {
            value |= Tile::T1;
        };
        if point_mask & 0b010_000 == 0b010_000 {
            value |= Tile::T2;
        };
        if point_mask & 0b001_000 == 0b001_000 {
            value |= Tile::T3;
        };

        if curve_mask & 0b011_000 == 0b011_000 {
            value |= Tile::A1;
        };
        if curve_mask & 0b101_000 == 0b101_000 {
            value |= Tile::A2;
        };
        if curve_mask & 0b110_000 == 0b110_000 {
            value |= Tile::A3;
        };
        if line_mask & 0b001 == 0b001 {
            value |= Tile::L1;
        };
        if line_mask & 0b010 == 0b010 {
            value |= Tile::L2;
        };
        if line_mask & 0b100 == 0b100 {
            value |= Tile::L3;
        };

        // e.g if A1 and L1 then C1 and clear A1/L1
        // CXs
        if value & (Tile::A1 | Tile::L1) == (Tile::A1 | Tile::L1) {
            value ^= Tile::L1;
            value |= Tile::C1;
        }
        if value & (Tile::A2 | Tile::L2) == (Tile::A2 | Tile::L2) {
            value ^= Tile::L2;
            value |= Tile::C2;
        }
        if value & (Tile::A3 | Tile::L3) == (Tile::A3 | Tile::L3) {
            value ^= Tile::L3;
            value |= Tile::C3;
        }

        // X1
        if value & (Tile::AX) == (Tile::AX) {
            value ^= Tile::AX;
            value |= Tile::X1;
        }

        // X2
        if value & Tile::LX == Tile::LX {
            value ^= Tile::L1;
            value ^= Tile::L2;
            value |= Tile::X2;
            // L3 is solid
        } else {
            // Yxs
            if value & (Tile::L2 | Tile::L3) == (Tile::L2 | Tile::L3) {
                value ^= Tile::L2;
                value |= Tile::Y1;
                // L3 is solid
            }
            if value & (Tile::L1 | Tile::L3) == (Tile::L1 | Tile::L3) {
                value ^= Tile::L3;
                value |= Tile::Y2;
                // L1 is solid
            }
            if value & (Tile::L1 | Tile::L2) == (Tile::L1 | Tile::L2) {
                value ^= Tile::L1;
                value |= Tile::Y3;
                // L2 is solid
            }
        }

        value
    }
}

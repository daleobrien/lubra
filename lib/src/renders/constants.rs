use array2d::Array2D;
use bitflags::bitflags;
use serde::ser::{Serialize, Serializer};

use crate::renders::primatives::{Line, Point};
use crate::renders::tile_subsets::random_side;

bitflags! {
  #[derive(Default, serde::Deserialize)]
  pub struct TileFlags: u32 {
    // graph node
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
    const Z2 = 0b00_000_000_010_000_000_000_000_000_000_000; // Considered not routed.
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

impl Serialize for TileFlags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u32(self.bits)
    }
}

fn add(u: usize, i: i32) -> usize {
    if i.is_negative() {
        u - i.wrapping_abs() as u32 as usize
    } else {
        u + i as usize
    }
}

fn get_tile_relative_postion(
    pos: (usize, usize),
    rel: (i32, i32),
    tiles: &Array2D<TileFlags>,
) -> TileFlags {
  
    if pos.1 as i32 + rel.1 < 0 || pos.0 as i32 + rel.0 < 0 {
      return TileFlags::Z1;
    }
    
    let x:usize = add(pos.0, rel.0);
    let y:usize = add(pos.1, rel.1);
    
   let tile = tiles.get(y, x);
   return match tile {
     Some(t) => *t,
     None => TileFlags::Z1
   }
}

fn calc_vertex(gathered_tiles: [[TileFlags; 5]; 3], tile: TileFlags, pos: usize, is_top: bool, ) -> (bool, bool) {

    let this_row = gathered_tiles[1];
    let other_row = if is_top { gathered_tiles[2] } else { gathered_tiles[0] };
    
    let opposite_tile = other_row[4 - 2 * pos];

    if opposite_tile.contains(TileFlags::Z2) {
        if pos == 0
            && !this_row[3].contains(tile)
            && !this_row[4].contains(tile)
            && !other_row[2].contains(tile)
            && !other_row[3].contains(tile)
        {
            return (false, true);
        }
        if pos == 1
            && !this_row[1].contains(tile)
            && !this_row[3].contains(tile)
            && !other_row[1].contains(tile)
            && !other_row[3].contains(tile)
        {
            return (false, true);
        }
        if pos == 2
            && !this_row[0].contains(tile)
            && !this_row[1].contains(tile)
            && !other_row[1].contains(tile)
            && !other_row[2].contains(tile)
        {
            return (false, true);
        }
    } else if opposite_tile.contains(tile) {
        return (true, false);
    }
    (false, false)
}

fn calc_side(gathered_tiles: [[TileFlags; 5]; 3], tile: TileFlags, pos: usize, full_pos: usize) -> (bool, bool) {
  
  let j = if pos == 1 { if full_pos % 2 == 1 { 0 } else { 2 } } else { 1 };
  
  let adjacent = gathered_tiles[j][pos+1];
  
  if adjacent.contains(TileFlags::Z2) {
    return (false, true);
  }
  if adjacent.contains(tile) {
      return (true, false);
  }
  (false, false)
}

pub fn get_random_tile(i: usize, j: usize, tiles: &Array2D<TileFlags>) -> TileFlags {
    if i == 0 && j == 0 {
        return TileFlags::L3;
    }

    let mut dont_care = 0;
    let mut matches = 0;

    let pos1 = (4 + i + (3 * (j % 2))) % 6;
    let pos2 = (2 + i + (3 * (j % 2))) % 6;
    let pos3 = (0 + i + (3 * (j % 2))) % 6; // Alternates on each row , 1st row 0,1,2,3,4,5,0, ... 2nd row, 3,4,5,0,1,2,3,...

    let gathered_tiles = [
        [
            get_tile_relative_postion((i, j), (-2, -1), tiles),
            get_tile_relative_postion((i, j), (-1, -1), tiles),
            get_tile_relative_postion((i, j), (0, -1), tiles),
            get_tile_relative_postion((i, j), (1, -1), tiles),
            get_tile_relative_postion((i, j), (2, -1), tiles),
        ],
        [
            get_tile_relative_postion((i, j), (-2, 0), tiles),
            get_tile_relative_postion((i, j), (-1, 0), tiles),
            TileFlags::Z2,
            get_tile_relative_postion((i, j), (1, 0), tiles),
            get_tile_relative_postion((i, j), (2, 0), tiles),
        ],
        [
            get_tile_relative_postion((i, j), (-2, 1), tiles),
            get_tile_relative_postion((i, j), (-1, 1), tiles),
            get_tile_relative_postion((i, j), (0, 1), tiles),
            get_tile_relative_postion((i, j), (1, 1), tiles),
            get_tile_relative_postion((i, j), (2, 1), tiles),
        ],
    ];


    let (m, d) = calc_vertex(gathered_tiles, TileFlags::G1, (1 + pos3) % 3, pos1 < 3);
    if m {
        matches |= 0b000_001;
    }
    if d {
        dont_care |= 0b000_001;
    }
  

    let (m, d) = calc_vertex(gathered_tiles, TileFlags::G2, (2 + pos3) % 3, pos2 < 3);
    if m {
        matches |= 0b000_010;
    }
    if d {
        dont_care |= 0b000_010;
    }

    let (m, d) = calc_vertex(gathered_tiles, TileFlags::G3, (3 + pos3) % 3, pos3 < 3);
    if m {
        matches |= 0b000_100;
    }
    if d {
        dont_care |= 0b000_100;
    }
    
    let (m, d) = calc_side(gathered_tiles, TileFlags::G4, (3 + pos3) % 3, pos3);
    if m {
        matches |= 0b001_000;
    }
    if d {
        dont_care |= 0b001_000;
    }

   let (m, d) = calc_side(gathered_tiles, TileFlags::G5, (3 + pos2) % 3, pos3);
    if m {
        matches |= 0b010_000;
    }
    if d {
        dont_care |= 0b010_000;
    }
    
    let (m, d) = calc_side(gathered_tiles, TileFlags::G6, (3 + pos1) % 3, pos3);
    if m {
        matches |= 0b100_000;
    }
    if d {
        dont_care |= 0b100_000;
    }
    // Don't like Nx so much
  
    // TODO: stip out cells that only have Nx and Tx
  
    for _ in 0..45 {
      let first_tile = random_side(matches, dont_care);
      if !first_tile.contains(TileFlags::N1) && !first_tile.contains(TileFlags::N2) && !first_tile.contains(TileFlags::N3) {
        return first_tile;
      }
    } 
    return random_side(matches, dont_care);
}

impl From<[u32; 3]> for TileFlags {
    fn from(masks: [u32; 3]) -> TileFlags {
        let [point_mask, line_mask, curve_mask] = masks;

        // Nx and Tx
        let mut value = TileFlags::from_bits_truncate(0);

        if point_mask & 0b000_001 == 0b000_001 {
            value |= TileFlags::N1;
        };
        if point_mask & 0b000_010 == 0b000_010 {
            value |= TileFlags::N2;
        };
        if point_mask & 0b000_100 == 0b000_100 {
            value |= TileFlags::N3;
        };

        if point_mask & 0b100_000 == 0b100_000 {
            value |= TileFlags::T1;
        };
        if point_mask & 0b010_000 == 0b010_000 {
            value |= TileFlags::T2;
        };
        if point_mask & 0b001_000 == 0b001_000 {
            value |= TileFlags::T3;
        };

        if curve_mask & 0b011_000 == 0b011_000 {
            value |= TileFlags::A1;
        };
        if curve_mask & 0b101_000 == 0b101_000 {
            value |= TileFlags::A2;
        };
        if curve_mask & 0b110_000 == 0b110_000 {
            value |= TileFlags::A3;
        };
        if line_mask & 0b001 == 0b001 {
            value |= TileFlags::L1;
        };
        if line_mask & 0b010 == 0b010 {
            value |= TileFlags::L2;
        };
        if line_mask & 0b100 == 0b100 {
            value |= TileFlags::L3;
        };

        // e.g if A1 and L1 then C1 and clear A1/L1
        // CXs
        if value & (TileFlags::A1 | TileFlags::L1) == (TileFlags::A1 | TileFlags::L1) {
            value ^= TileFlags::L1;
            value |= TileFlags::C1;
        }
        if value & (TileFlags::A2 | TileFlags::L2) == (TileFlags::A2 | TileFlags::L2) {
            value ^= TileFlags::L2;
            value |= TileFlags::C2;
        }
        if value & (TileFlags::A3 | TileFlags::L3) == (TileFlags::A3 | TileFlags::L3) {
            value ^= TileFlags::L3;
            value |= TileFlags::C3;
        }

        // X1
        if value & (TileFlags::AX) == (TileFlags::AX) {
            value ^= TileFlags::AX;
            value |= TileFlags::X1;
        }

        // X2
        if value & TileFlags::LX == TileFlags::LX {
            value ^= TileFlags::L1;
            value ^= TileFlags::L2;
            value |= TileFlags::X2;
            // L3 is solid
        } else {
            // Yxs
            if value & (TileFlags::L2 | TileFlags::L3) == (TileFlags::L2 | TileFlags::L3) {
                value ^= TileFlags::L2;
                value |= TileFlags::Y1;
                // L3 is solid
            }
            if value & (TileFlags::L1 | TileFlags::L3) == (TileFlags::L1 | TileFlags::L3) {
                value ^= TileFlags::L3;
                value |= TileFlags::Y2;
                // L1 is solid
            }
            if value & (TileFlags::L1 | TileFlags::L2) == (TileFlags::L1 | TileFlags::L2) {
                value ^= TileFlags::L1;
                value |= TileFlags::Y3;
                // L2 is solid
            }
        }

        value
    }
}

pub const U: f64 = 120.0;
pub const Q1: f64 = 30.0;
pub const Q3: f64 = 90.0;
pub const M: f64 = 60.0;
pub const Z: f64 = 0.0;

pub const W: f64 = 20.0;

// U * f64::sqrt(3.0) / 2.0;
pub const H: f64 = 103.92304845413263761;

// U * f64::sqrt(3.0) / 6.0;
// pub const D: f64 = 34.641016151377545871;

pub const L01: Line = Line {
    start: Point { x: 40., y: 0. },
    delta: Point {
        x: -30.,
        y: -17.3205,
    },
};
pub const L02: Line = Line {
    start: Point {
        x: -10.,
        y: 17.3205,
    },
    delta: Point { x: 30., y: 17.3205 },
};

pub const L07: Line = Line {
    start: Point { x: 40., y: 69.282 },
    delta: Point { x: 0., y: 34.641 },
};
pub const L08: Line = Line {
    start: Point { x: 80., y: 103.923 },
    delta: Point { x: 0., y: -34.641 },
};

pub const L11: Line = Line {
    start: Point { x: 100., y: 34.641 },
    delta: Point {
        x: 30.,
        y: -17.3205,
    },
};
pub const L12: Line = Line {
    start: Point {
        x: 110.,
        y: -17.3205,
    },
    delta: Point {
        x: -30.,
        y: 17.3205,
    },
};
pub const L13: Line = Line {
    start: Point { x: 20., y: 34.641 },
    delta: Point {
        x: 22.918,
        y: -13.2317,
    },
};
pub const L14: Line = Line {
    start: Point { x: 40., y: 69.282 },
    delta: Point {
        x: 22.918,
        y: -13.2317,
    },
};
pub const L15: Line = Line {
    start: Point { x: 80., y: 69.282 },
    delta: Point {
        x: -22.918,
        y: -13.2317,
    },
};
pub const L16: Line = Line {
    start: Point { x: 100., y: 34.641 },
    delta: Point {
        x: -22.918,
        y: -13.2317,
    },
};
pub const L17: Line = Line {
    start: Point { x: 80., y: 0. },
    delta: Point { x: 0., y: 26.4634 },
};
pub const L18: Line = Line {
    start: Point { x: 40., y: 0. },
    delta: Point { x: 0., y: 26.4634 },
};
pub const L19: Line = Line {
    start: Point { x: 20., y: 34.641 },
    delta: Point {
        x: 37.082,
        y: 21.4093,
    },
};
pub const L20: Line = Line {
    start: Point { x: 40., y: 0. },
    delta: Point {
        x: 37.082,
        y: 21.4093,
    },
};
pub const L21: Line = Line {
    start: Point { x: 40., y: 26.4634 },
    delta: Point { x: 0., y: 42.8187 },
};
pub const L22: Line = Line {
    start: Point { x: 80., y: 26.4634 },
    delta: Point { x: 0., y: 42.8187 },
};
pub const L23: Line = Line {
    start: Point { x: 80., y: 0. },
    delta: Point {
        x: -37.082,
        y: 21.4093,
    },
};
pub const L24: Line = Line {
    start: Point { x: 100., y: 34.641 },
    delta: Point {
        x: -37.082,
        y: 21.4093,
    },
};
pub const L25: Line = Line {
    start: Point { x: 40., y: 103.923 },
    delta: Point {
        x: -30.,
        y: 17.3205,
    },
};
pub const L26: Line = Line {
    start: Point {
        x: -10.,
        y: 86.6025,
    },
    delta: Point {
        x: 30.,
        y: -17.3205,
    },
};

pub const L31: Line = Line {
    start: Point { x: 40., y: 34.641 },
    delta: Point { x: 0., y: -34.641 },
};
pub const L32: Line = Line {
    start: Point { x: 80., y: 0. },
    delta: Point { x: 0., y: 34.641 },
};

pub const L35: Line = Line {
    start: Point { x: 100., y: 69.282 },
    delta: Point { x: 30., y: 17.3205 },
};
pub const L36: Line = Line {
    start: Point {
        x: 110.,
        y: 121.244,
    },
    delta: Point {
        x: -30.,
        y: -17.3205,
    },
};
pub const L37: Line = Line {
    start: Point { x: 20., y: 69.282 },
    delta: Point {
        x: 22.918,
        y: 13.2317,
    },
};
pub const L38: Line = Line {
    start: Point { x: 40., y: 34.641 },
    delta: Point {
        x: 22.918,
        y: 13.2317,
    },
};
pub const L39: Line = Line {
    start: Point { x: 80., y: 34.641 },
    delta: Point {
        x: -22.918,
        y: 13.2317,
    },
};
pub const L40: Line = Line {
    start: Point { x: 100., y: 69.282 },
    delta: Point {
        x: -22.918,
        y: 13.2317,
    },
};
pub const L41: Line = Line {
    start: Point { x: 80., y: 103.923 },
    delta: Point { x: 0., y: -26.4634 },
};
pub const L42: Line = Line {
    start: Point { x: 40., y: 103.923 },
    delta: Point { x: 0., y: -26.4634 },
};
pub const L43: Line = Line {
    start: Point { x: 20., y: 69.282 },
    delta: Point {
        x: 37.082,
        y: -21.4093,
    },
};
pub const L44: Line = Line {
    start: Point { x: 40., y: 103.923 },
    delta: Point {
        x: 37.082,
        y: -21.4093,
    },
};
pub const L45: Line = Line {
    start: Point { x: 40., y: 77.4597 },
    delta: Point { x: 0., y: -42.8187 },
};
pub const L46: Line = Line {
    start: Point { x: 80., y: 77.4597 },
    delta: Point { x: 0., y: -42.8187 },
};
pub const L47: Line = Line {
    start: Point { x: 80., y: 103.923 },
    delta: Point {
        x: -37.082,
        y: -21.4093,
    },
};
pub const L48: Line = Line {
    start: Point { x: 100., y: 69.282 },
    delta: Point {
        x: -37.082,
        y: -21.4093,
    },
};
pub const L49: Line = Line {
    start: Point { x: 40., y: 0. },
    delta: Point { x: 0., y: 103.923 },
};
pub const L50: Line = Line {
    start: Point { x: 40., y: 69.282 },
    delta: Point {
        x: 90.,
        y: -51.9615,
    },
};
pub const L51: Line = Line {
    start: Point { x: 80., y: 69.282 },
    delta: Point {
        x: -90.,
        y: -51.9615,
    },
};
pub const L52: Line = Line {
    start: Point { x: 80., y: 0. },
    delta: Point { x: 0., y: 103.923 },
};
pub const L53: Line = Line {
    start: Point { x: 20., y: 34.641 },
    delta: Point {
        x: 90.,
        y: -51.9615,
    },
};
pub const L54: Line = Line {
    start: Point { x: 100., y: 34.641 },
    delta: Point {
        x: -90.,
        y: -51.9615,
    },
};

pub const L56: Line = Line {
    start: Point { x: 40., y: 34.641 },
    delta: Point { x: 90., y: 51.9615 },
};
pub const L57: Line = Line {
    start: Point { x: 80., y: 34.641 },
    delta: Point {
        x: -90.,
        y: 51.9615,
    },
};

pub const L59: Line = Line {
    start: Point { x: 20., y: 69.282 },
    delta: Point { x: 90., y: 51.9615 },
};
pub const L60: Line = Line {
    start: Point { x: 100., y: 69.282 },
    delta: Point {
        x: -90.,
        y: 51.9615,
    },
};

pub const L62: Line = Line {
    start: Point { x: 40., y: 23.094 },
    delta: Point { x: -20., y: 11.547 },
};
pub const L63: Line = Line {
    start: Point { x: 80., y: 23.094 },
    delta: Point { x: 0., y: -23.094 },
};

pub const L66: Line = Line {
    start: Point { x: 60., y: 57.735 },
    delta: Point { x: 20., y: 11.547 },
};
pub const L67: Line = Line {
    start: Point { x: 40., y: 80.829 },
    delta: Point {
        x: -20.,
        y: -11.547,
    },
};

pub const L69: Line = Line {
    start: Point { x: 80., y: 80.829 },
    delta: Point { x: 0., y: 23.094 },
};

pub const L72: Line = Line {
    start: Point { x: 60., y: 46.188 },
    delta: Point { x: 20., y: -11.547 },
};

pub const L74: Line = Line {
    start: Point { x: 60., y: 11.547 },
    delta: Point {
        x: -50.,
        y: -28.8675,
    },
};

pub const L76: Line = Line {
    start: Point { x: 40., y: 46.188 },
    delta: Point { x: 0., y: 57.735 },
};
pub const L77: Line = Line {
    start: Point { x: 80., y: 46.188 },
    delta: Point {
        x: 50.,
        y: -28.8675,
    },
};

pub const L80: Line = Line {
    start: Point { x: 60., y: 92.376 },
    delta: Point {
        x: -50.,
        y: 28.8675,
    },
};
pub const L81: Line = Line {
    start: Point { x: 40., y: 57.735 },
    delta: Point { x: 0., y: -57.735 },
};

pub const L83: Line = Line {
    start: Point { x: 80., y: 57.735 },
    delta: Point { x: 50., y: 28.8675 },
};

pub const L92: Line = Line {
    start: Point { x: 60., y: 11.547 },
    delta: Point { x: 40., y: 23.094 },
};

pub const L96: Line = Line {
    start: Point { x: 80., y: 46.188 },
    delta: Point { x: -40., y: 23.094 },
};

pub const L98: Line = Line {
    start: Point { x: 40., y: 80.829 },
    delta: Point { x: 40., y: 23.094 },
};
pub const L99: Line = Line {
    start: Point { x: 80., y: 80.829 },
    delta: Point { x: 0., y: -46.188 },
};

pub const L102: Line = Line {
    start: Point { x: 60., y: 46.188 },
    delta: Point { x: -40., y: 23.094 },
};
pub const L103: Line = Line {
    start: Point { x: 60., y: 57.735 },
    delta: Point {
        x: -40.,
        y: -23.094,
    },
};

pub const L106: Line = Line {
    start: Point { x: 40., y: 23.094 },
    delta: Point { x: 40., y: -23.094 },
};
pub const L110: Line = Line {
    start: Point { x: 40., y: 46.188 },
    delta: Point { x: 0., y: -46.188 },
};
pub const L116: Line = Line {
    start: Point { x: 60., y: 92.376 },
    delta: Point { x: 40., y: -23.094 },
};
pub const L117: Line = Line {
    start: Point { x: 40., y: 57.735 },
    delta: Point { x: 0., y: 46.188 },
};

pub const L120: Line = Line {
    start: Point { x: 80., y: 57.735 },
    delta: Point {
        x: -40.,
        y: -23.094,
    },
};

pub const L123: Line = Line {
    start: Point { x: 80., y: 23.094 },
    delta: Point { x: 0., y: 46.188 },
};

use crate::renders::primatives::{Line, Point};
use array2d::Array2D;
use bitflags::bitflags;
use serde::ser::{Serialize, Serializer};

use crate::renders::tile_subsets::random_side;

bitflags! {
  #[derive(Default, serde::Deserialize)]
  pub struct Tile: u32 {
    // Maybe alter these flags to be a mini-adjoint graph, so it will make it easier to use.
    /*
     0 0 0 0 0 0
     0 0 0 0 0 0
     0 0 0 0 0 0
     0 0 0 0 0 0
     0 0 0 0 0 0
     0 0 0 0 0 0  (36 bits), but not all are used,


     0 0 0 0 0 0
       0 0 0 0 0
         0 0 0 0
           0 0 0
             0 0
               0  (21 bits)

    Not all graphs are posible, so maybe could reduce more, I know I only have 193 distinct graphs, so could use
    8 bitsm but then it's all a lookup and would have to decode/enocde.

    We only really have T,N,L & A, and even then we can't have all those at once, ... that's < 12 bits


TODO: Would a grid be better now? (adj matrix)

  0 1 0 0 0 0
  1 0 0 0 0 0
  0 0 1 0 1 0
  0 0 0 0 0 0
  0 0 1 0 0 0
  0 0 0 0 0 0

-> 36 bits  ( many bits are redant, unless I consider directional mazes, could be interestings)


    */

    // Graph node
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

impl Tile {
    pub fn rotate_cw(&self) -> Tile {
        let mut tile = Self::Z1;

        if self.contains(Self::W1) {
            tile |= Self::W1;
        }
        if self.contains(Self::X1) {
            tile |= Self::X1;
        }
        if self.contains(Self::X2) {
            tile |= Self::X2;
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
                tile |= b;
            }
        }
        return tile;
    }
    pub fn rotate_ccw(&self) -> Tile {
        let mut tile = Self::Z1;

        if self.contains(Self::W1) {
            tile |= Self::W1;
        }
        if self.contains(Self::X1) {
            tile |= Self::X1;
        }
        if self.contains(Self::X2) {
            tile |= Self::X2;
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
                tile |= b;
            }
        }
        return tile;
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

fn add(u: usize, i: i32) -> usize {
    if i.is_negative() {
        u - i.wrapping_abs() as u32 as usize
    } else {
        u + i as usize
    }
}

fn get_tile_relative_position(
    pos: (usize, usize),
    rel: (i32, i32),
    tiles: &Array2D<Tile>, // Flip to Maze, od move this function into Maze
) -> Tile {
    if pos.1 as i32 + rel.1 < 0 || pos.0 as i32 + rel.0 < 0 {
        return Tile::Z1;
    }

    let x: usize = add(pos.0, rel.0);
    let y: usize = add(pos.1, rel.1);

    let tile = tiles.get(y, x);
    return match tile {
        Some(t) => *t,
        None => Tile::Z1,
    };
}

fn calc_vertex(
    gathered_tiles: [[Tile; 5]; 3],
    tile: Tile,
    pos: usize, // 0..2
    is_top: bool,
) -> (bool, bool) {
    /*
                         ____ ____ ____
                             /\   /\
           this row -> .    /  \ /  \
                            ---- ----

    The other row depends on where the tile is pointing up or down
        */

    let this_row = gathered_tiles[1];

    let other_row = if is_top {
        gathered_tiles[2]
    } else {
        gathered_tiles[0]
    };

    let opposite_tile = other_row[4 - 2 * pos]; // index 4, 2, 0

    if opposite_tile.contains(Tile::Z2) {
        // is not routed
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
        // Must be 1
        return (true, false);
    }
    // Must be 0
    (false, false)
}

fn calc_side(
    gathered_tiles: [[Tile; 5]; 3],
    tile: Tile,
    pos: usize,
    full_pos: usize,
) -> (bool, bool) {
    let j = if pos == 1 {
        if full_pos % 2 == 1 {
            0
        } else {
            2
        }
    } else {
        1
    };

    let adjacent = gathered_tiles[j][pos + 1];

    if adjacent.contains(Tile::Z2) {
        // Don't care
        return (false, true);
    }
    if adjacent.contains(tile) {
        // Must match, on a one
        return (true, false);
    }
    // Must match, on a zero
    (false, false)
}

pub fn get_surrounding_tiles(i: usize, j: usize, tiles: &Array2D<Tile>) -> [[Tile; 5]; 3] {
    let gathered_tiles = [
        [
            get_tile_relative_position((i, j), (-2, -1), tiles),
            get_tile_relative_position((i, j), (-1, -1), tiles),
            get_tile_relative_position((i, j), (0, -1), tiles),
            get_tile_relative_position((i, j), (1, -1), tiles),
            get_tile_relative_position((i, j), (2, -1), tiles),
        ],
        [
            get_tile_relative_position((i, j), (-2, 0), tiles),
            get_tile_relative_position((i, j), (-1, 0), tiles),
            Tile::Z2,
            get_tile_relative_position((i, j), (1, 0), tiles),
            get_tile_relative_position((i, j), (2, 0), tiles),
        ],
        [
            get_tile_relative_position((i, j), (-2, 1), tiles),
            get_tile_relative_position((i, j), (-1, 1), tiles),
            get_tile_relative_position((i, j), (0, 1), tiles),
            get_tile_relative_position((i, j), (1, 1), tiles),
            get_tile_relative_position((i, j), (2, 1), tiles),
        ],
    ];
    return gathered_tiles;
}

pub fn get_surrounding_tile_connections(i: usize, j: usize, tiles: &Array2D<Tile>) -> (u32, u32) {
    let surrounding_tiles = get_surrounding_tiles(i, j, tiles);

    let mut dont_care = 0u32;
    let mut matches = 0u32;

    let pos1 = (4 + i + (3 * (j % 2))) % 6;
    let pos2 = (2 + i + (3 * (j % 2))) % 6;
    let pos3 = (0 + i + (3 * (j % 2))) % 6; // Alternates on each row , 1st row 0,1,2,3,4,5,0, ... 2nd row, 3,4,5,0,1,2,3,...

    // Node 1
    let (m, d) = calc_vertex(surrounding_tiles, Tile::G1, (1 + pos3) % 3, pos1 < 3);
    if m {
        matches |= 0b000_001;
    }
    if d {
        dont_care |= 0b000_001;
    }

    // Node 2
    let (m, d) = calc_vertex(surrounding_tiles, Tile::G2, (2 + pos3) % 3, pos2 < 3);
    if m {
        matches |= 0b000_010;
    }
    if d {
        dont_care |= 0b000_010;
    }

    // Node3
    let (m, d) = calc_vertex(surrounding_tiles, Tile::G3, (3 + pos3) % 3, pos3 < 3);
    if m {
        matches |= 0b000_100;
    }
    if d {
        dont_care |= 0b000_100;
    }

    let (m, d) = calc_side(surrounding_tiles, Tile::G4, (3 + pos3) % 3, pos3);
    if m {
        matches |= 0b001_000;
    }
    if d {
        dont_care |= 0b001_000;
    }

    let (m, d) = calc_side(surrounding_tiles, Tile::G5, (3 + pos2) % 3, pos3);
    if m {
        matches |= 0b010_000;
    }
    if d {
        dont_care |= 0b010_000;
    }

    let (m, d) = calc_side(surrounding_tiles, Tile::G6, (3 + pos1) % 3, pos3);
    if m {
        matches |= 0b100_000;
    }
    if d {
        dont_care |= 0b100_000;
    }
    return (dont_care, matches);
}

pub fn get_random_tile(i: usize, j: usize, tiles: &Array2D<Tile>) -> Tile {
    if i == 0 && j == 0 {
        return Tile::L3;
    }

    let (dont_care, matches) = get_surrounding_tile_connections(i, j, tiles);

    // TODO: strip out cells that only have Nx and Tx
    for _ in 0..45 {
        let first_tile = random_side(matches, dont_care);
        if !first_tile.contains(Tile::N1)
            && !first_tile.contains(Tile::N2)
            && !first_tile.contains(Tile::N3)
        {
            return first_tile;
        }
    }
    return random_side(matches, dont_care);
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

pub const SMALL_ANGLE: f64 = 0.2709185204562202f64;

pub const U: f64 = 120.0;
pub const Q1: f64 = 30.0;
pub const Q3: f64 = 90.0;
pub const M: f64 = 60.0;
pub const Z: f64 = 0.0;

pub const W: f64 = 20.0;

// U * f64::sqrt(3.0) / 2.0;
pub const H: f64 = 103.92304845413263761f64;

// U * f64::sqrt(3.0) / 6.0;
// pub const D: f64 = 34.641016151377545871;

pub const L001: Line = Line {
    start: Point { x: 40., y: 0. },
    delta: Point {
        x: -30.,
        y: -17.3205,
    },
};
pub const L002: Line = Line {
    start: Point {
        x: -10.,
        y: 17.3205,
    },
    delta: Point { x: 30., y: 17.3205 },
};
pub const L003: Line = Line {
    start: Point { x: 80., y: 69.282 },
    delta: Point {
        x: -30.,
        y: -17.3205,
    },
};
pub const L004: Line = Line {
    start: Point { x: 100., y: 34.641 },
    delta: Point {
        x: -30.,
        y: -17.3205,
    },
};
pub const L005: Line = Line {
    start: Point { x: 40., y: 0. },
    delta: Point { x: 0., y: 34.641 },
};
pub const L006: Line = Line {
    start: Point { x: 80., y: 0. },
    delta: Point { x: 0., y: 34.641 },
};
pub const L007: Line = Line {
    start: Point { x: 40., y: 69.282 },
    delta: Point { x: 0., y: 34.641 },
};
pub const L008: Line = Line {
    start: Point { x: 80., y: 103.923 },
    delta: Point { x: 0., y: -34.641 },
};
pub const L009: Line = Line {
    start: Point { x: 40., y: 69.282 },
    delta: Point {
        x: 30.,
        y: -17.3205,
    },
};
pub const L010: Line = Line {
    start: Point { x: 20., y: 34.641 },
    delta: Point {
        x: 30.,
        y: -17.3205,
    },
};
pub const L011: Line = Line {
    start: Point { x: 100., y: 34.641 },
    delta: Point {
        x: 30.,
        y: -17.3205,
    },
};
pub const L012: Line = Line {
    start: Point {
        x: 110.,
        y: -17.3205,
    },
    delta: Point {
        x: -30.,
        y: 17.3205,
    },
};
pub const L013: Line = Line {
    start: Point { x: 20., y: 34.641 },
    delta: Point {
        x: 22.918,
        y: -13.2317,
    },
};
pub const L014: Line = Line {
    start: Point { x: 40., y: 69.282 },
    delta: Point {
        x: 22.918,
        y: -13.2317,
    },
};
pub const L015: Line = Line {
    start: Point { x: 80., y: 69.282 },
    delta: Point {
        x: -22.918,
        y: -13.2317,
    },
};
pub const L016: Line = Line {
    start: Point { x: 100., y: 34.641 },
    delta: Point {
        x: -22.918,
        y: -13.2317,
    },
};
pub const L017: Line = Line {
    start: Point { x: 80., y: 0. },
    delta: Point { x: 0., y: 26.4634 },
};
pub const L018: Line = Line {
    start: Point { x: 40., y: 0. },
    delta: Point { x: 0., y: 26.4634 },
};
pub const L019: Line = Line {
    start: Point { x: 20., y: 34.641 },
    delta: Point {
        x: 37.082,
        y: 21.4093,
    },
};
pub const L020: Line = Line {
    start: Point { x: 40., y: 0. },
    delta: Point {
        x: 37.082,
        y: 21.4093,
    },
};
pub const L021: Line = Line {
    start: Point { x: 40., y: 26.4634 },
    delta: Point { x: 0., y: 42.8187 },
};
pub const L022: Line = Line {
    start: Point { x: 80., y: 26.4634 },
    delta: Point { x: 0., y: 42.8187 },
};
pub const L023: Line = Line {
    start: Point { x: 80., y: 0. },
    delta: Point {
        x: -37.082,
        y: 21.4093,
    },
};
pub const L024: Line = Line {
    start: Point { x: 100., y: 34.641 },
    delta: Point {
        x: -37.082,
        y: 21.4093,
    },
};
pub const L025: Line = Line {
    start: Point { x: 40., y: 103.923 },
    delta: Point {
        x: -30.,
        y: 17.3205,
    },
};
pub const L026: Line = Line {
    start: Point {
        x: -10.,
        y: 86.6025,
    },
    delta: Point {
        x: 30.,
        y: -17.3205,
    },
};
pub const L027: Line = Line {
    start: Point { x: 80., y: 34.641 },
    delta: Point {
        x: -30.,
        y: 17.3205,
    },
};
pub const L028: Line = Line {
    start: Point { x: 100., y: 69.282 },
    delta: Point {
        x: -30.,
        y: 17.3205,
    },
};
pub const L033: Line = Line {
    start: Point { x: 40., y: 34.641 },
    delta: Point { x: 30., y: 17.3205 },
};
pub const L034: Line = Line {
    start: Point { x: 20., y: 69.282 },
    delta: Point { x: 30., y: 17.3205 },
};
pub const L035: Line = Line {
    start: Point { x: 100., y: 69.282 },
    delta: Point { x: 30., y: 17.3205 },
};
pub const L036: Line = Line {
    start: Point {
        x: 110.,
        y: 121.244,
    },
    delta: Point {
        x: -30.,
        y: -17.3205,
    },
};
pub const L037: Line = Line {
    start: Point { x: 20., y: 69.282 },
    delta: Point {
        x: 22.918,
        y: 13.2317,
    },
};
pub const L038: Line = Line {
    start: Point { x: 40., y: 34.641 },
    delta: Point {
        x: 22.918,
        y: 13.2317,
    },
};
pub const L039: Line = Line {
    start: Point { x: 80., y: 34.641 },
    delta: Point {
        x: -22.918,
        y: 13.2317,
    },
};
pub const L040: Line = Line {
    start: Point { x: 100., y: 69.282 },
    delta: Point {
        x: -22.918,
        y: 13.2317,
    },
};
pub const L041: Line = Line {
    start: Point { x: 80., y: 103.923 },
    delta: Point { x: 0., y: -26.4634 },
};
pub const L042: Line = Line {
    start: Point { x: 40., y: 103.923 },
    delta: Point { x: 0., y: -26.4634 },
};
pub const L043: Line = Line {
    start: Point { x: 20., y: 69.282 },
    delta: Point {
        x: 37.082,
        y: -21.4093,
    },
};
pub const L044: Line = Line {
    start: Point { x: 40., y: 103.923 },
    delta: Point {
        x: 37.082,
        y: -21.4093,
    },
};
pub const L045: Line = Line {
    start: Point { x: 40., y: 77.4597 },
    delta: Point { x: 0., y: -42.8187 },
};
pub const L046: Line = Line {
    start: Point { x: 80., y: 77.4597 },
    delta: Point { x: 0., y: -42.8187 },
};
pub const L047: Line = Line {
    start: Point { x: 80., y: 103.923 },
    delta: Point {
        x: -37.082,
        y: -21.4093,
    },
};
pub const L048: Line = Line {
    start: Point { x: 100., y: 69.282 },
    delta: Point {
        x: -37.082,
        y: -21.4093,
    },
};
pub const L049: Line = Line {
    start: Point { x: 40., y: 0. },
    delta: Point { x: 0., y: 103.923 },
};
pub const L050: Line = Line {
    start: Point { x: 40., y: 69.282 },
    delta: Point {
        x: 90.,
        y: -51.9615,
    },
};
pub const L051: Line = Line {
    start: Point { x: 80., y: 69.282 },
    delta: Point {
        x: -90.,
        y: -51.9615,
    },
};
pub const L052: Line = Line {
    start: Point { x: 80., y: 0. },
    delta: Point { x: 0., y: 103.923 },
};
pub const L053: Line = Line {
    start: Point { x: 20., y: 34.641 },
    delta: Point {
        x: 90.,
        y: -51.9615,
    },
};
pub const L054: Line = Line {
    start: Point { x: 100., y: 34.641 },
    delta: Point {
        x: -90.,
        y: -51.9615,
    },
};
pub const L056: Line = Line {
    start: Point { x: 40., y: 34.641 },
    delta: Point { x: 90., y: 51.9615 },
};
pub const L057: Line = Line {
    start: Point { x: 80., y: 34.641 },
    delta: Point {
        x: -90.,
        y: 51.9615,
    },
};
pub const L059: Line = Line {
    start: Point { x: 20., y: 69.282 },
    delta: Point { x: 90., y: 51.9615 },
};
pub const L060: Line = Line {
    start: Point { x: 100., y: 69.282 },
    delta: Point {
        x: -90.,
        y: 51.9615,
    },
};
pub const L061: Line = Line {
    start: Point { x: 40., y: 23.094 },
    delta: Point { x: 0., y: -23.094 },
};
pub const L062: Line = Line {
    start: Point { x: 40., y: 23.094 },
    delta: Point { x: -20., y: 11.547 },
};
pub const L063: Line = Line {
    start: Point { x: 80., y: 23.094 },
    delta: Point { x: 0., y: -23.094 },
};
pub const L064: Line = Line {
    start: Point { x: 80., y: 23.094 },
    delta: Point { x: 20., y: 11.547 },
};
pub const L065: Line = Line {
    start: Point { x: 60., y: 57.735 },
    delta: Point { x: -20., y: 11.547 },
};
pub const L066: Line = Line {
    start: Point { x: 60., y: 57.735 },
    delta: Point { x: 20., y: 11.547 },
};
pub const L067: Line = Line {
    start: Point { x: 40., y: 80.829 },
    delta: Point {
        x: -20.,
        y: -11.547,
    },
};
pub const L068: Line = Line {
    start: Point { x: 40., y: 80.829 },
    delta: Point { x: 0., y: 23.094 },
};
pub const L069: Line = Line {
    start: Point { x: 80., y: 80.829 },
    delta: Point { x: 0., y: 23.094 },
};
pub const L070: Line = Line {
    start: Point { x: 80., y: 80.829 },
    delta: Point { x: 20., y: -11.547 },
};
pub const L071: Line = Line {
    start: Point { x: 60., y: 46.188 },
    delta: Point {
        x: -20.,
        y: -11.547,
    },
};
pub const L072: Line = Line {
    start: Point { x: 60., y: 46.188 },
    delta: Point { x: 20., y: -11.547 },
};
pub const L073: Line = Line {
    start: Point { x: 60., y: 11.547 },
    delta: Point {
        x: 50.,
        y: -28.8675,
    },
};
pub const L074: Line = Line {
    start: Point { x: 60., y: 11.547 },
    delta: Point {
        x: -50.,
        y: -28.8675,
    },
};
pub const L075: Line = Line {
    start: Point { x: 40., y: 46.188 },
    delta: Point {
        x: -50.,
        y: -28.8675,
    },
};
pub const L076: Line = Line {
    start: Point { x: 40., y: 46.188 },
    delta: Point { x: 0., y: 57.735 },
};
pub const L077: Line = Line {
    start: Point { x: 80., y: 46.188 },
    delta: Point {
        x: 50.,
        y: -28.8675,
    },
};
pub const L078: Line = Line {
    start: Point { x: 80., y: 46.188 },
    delta: Point { x: 0., y: 57.735 },
};
pub const L079: Line = Line {
    start: Point { x: 60., y: 92.376 },
    delta: Point { x: 50., y: 28.8675 },
};
pub const L080: Line = Line {
    start: Point { x: 60., y: 92.376 },
    delta: Point {
        x: -50.,
        y: 28.8675,
    },
};
pub const L081: Line = Line {
    start: Point { x: 40., y: 57.735 },
    delta: Point { x: 0., y: -57.735 },
};
pub const L082: Line = Line {
    start: Point { x: 40., y: 57.735 },
    delta: Point {
        x: -50.,
        y: 28.8675,
    },
};
pub const L083: Line = Line {
    start: Point { x: 80., y: 57.735 },
    delta: Point { x: 50., y: 28.8675 },
};
pub const L084: Line = Line {
    start: Point { x: 80., y: 57.735 },
    delta: Point { x: 0., y: -57.735 },
};
pub const L085: Line = Line {
    start: Point { x: 40., y: 57.735 },
    delta: Point { x: 40., y: -23.094 },
};
pub const L086: Line = Line {
    start: Point { x: 40., y: 57.735 },
    delta: Point { x: 0., y: 46.188 },
};
pub const L087: Line = Line {
    start: Point { x: 60., y: 92.376 },
    delta: Point { x: 40., y: -23.094 },
};
pub const L088: Line = Line {
    start: Point { x: 60., y: 92.376 },
    delta: Point {
        x: -40.,
        y: -23.094,
    },
};
pub const L089: Line = Line {
    start: Point { x: 80., y: 57.735 },
    delta: Point {
        x: -40.,
        y: -23.094,
    },
};
pub const L090: Line = Line {
    start: Point { x: 80., y: 57.735 },
    delta: Point { x: 0., y: 46.188 },
};
pub const L091: Line = Line {
    start: Point { x: 60., y: 11.547 },
    delta: Point { x: -40., y: 23.094 },
};
pub const L092: Line = Line {
    start: Point { x: 60., y: 11.547 },
    delta: Point { x: 40., y: 23.094 },
};
pub const L093: Line = Line {
    start: Point { x: 40., y: 46.188 },
    delta: Point { x: 0., y: -46.188 },
};
pub const L094: Line = Line {
    start: Point { x: 40., y: 46.188 },
    delta: Point { x: 40., y: 23.094 },
};
pub const L095: Line = Line {
    start: Point { x: 80., y: 46.188 },
    delta: Point { x: 0., y: -46.188 },
};
pub const L096: Line = Line {
    start: Point { x: 80., y: 46.188 },
    delta: Point { x: -40., y: 23.094 },
};
pub const L097: Line = Line {
    start: Point { x: 40., y: 80.829 },
    delta: Point { x: 0., y: -46.188 },
};
pub const L098: Line = Line {
    start: Point { x: 40., y: 80.829 },
    delta: Point { x: 40., y: 23.094 },
};
pub const L099: Line = Line {
    start: Point { x: 80., y: 80.829 },
    delta: Point { x: 0., y: -46.188 },
};
pub const L100: Line = Line {
    start: Point { x: 80., y: 80.829 },
    delta: Point { x: -40., y: 23.094 },
};
pub const L101: Line = Line {
    start: Point { x: 60., y: 46.188 },
    delta: Point { x: 40., y: 23.094 },
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
pub const L104: Line = Line {
    start: Point { x: 60., y: 57.735 },
    delta: Point { x: 40., y: -23.094 },
};
pub const L105: Line = Line {
    start: Point { x: 40., y: 23.094 },
    delta: Point { x: 0., y: 46.188 },
};
pub const L106: Line = Line {
    start: Point { x: 40., y: 23.094 },
    delta: Point { x: 40., y: -23.094 },
};
pub const L107: Line = Line {
    start: Point { x: 80., y: 23.094 },
    delta: Point {
        x: -40.,
        y: -23.094,
    },
};
pub const L121: Line = Line {
    start: Point { x: 80., y: 23.094 },
    delta: Point { x: 0., y: 46.188 },
};

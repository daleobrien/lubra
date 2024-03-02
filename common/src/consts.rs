/*
Each row is indexed in a strange way:  _,1,_,5,_,3,_,6,_,2,_,4
where the _ is too the left and right of the vertex, (vertex are number 1,2 & 3).
*/

pub const N1: u32 = 0b00_000_001;
pub const N2: u32 = 0b00_000_010;
pub const N3: u32 = 0b00_000_100;
pub const N4: u32 = 0b00_001_000;
pub const N5: u32 = 0b00_010_000;
pub const N6: u32 = 0b00_100_000;
pub const INVERT: u32 = 0b01_000_000;
pub const NORMAL: u32 = 0b00_000_000;
pub const WALL: u32 = 0b10_000_000;

pub const A1: u32 = 0b00_000_001_00_000_000;
pub const A2: u32 = 0b00_000_010_00_000_000;
pub const A3: u32 = 0b00_000_100_00_000_000;
pub const A4: u32 = 0b00_001_000_00_000_000;
pub const A5: u32 = 0b00_010_000_00_000_000;
pub const A6: u32 = 0b00_100_000_00_000_000;

pub const ADJ_N1: usize = 0usize;
pub const ADJ_N2: usize = 1usize;
pub const ADJ_N3: usize = 2usize;
pub const ADJ_N4: usize = 3usize;
pub const ADJ_N5: usize = 4usize;
pub const ADJ_N6: usize = 5usize;

pub const FILL: &str = "white";

// For a tile in the very first position, (0,0), the coordinates are:
const K0: [(i32, i32); 12] = [
    (0, -1),
    (-2, 1),
    (2, 1),
    (-1, 0),
    (1, 0),
    (0, 1),
    (-1, -1),
    (1, -1),
    (-2, 0),
    (-1, 1),
    (2, 0),
    (1, 1),
];
const K1: [(i32, i32); 12] = [
    (-2, -1),
    (2, -1),
    (0, 1),
    (0, -1),
    (-1, 0),
    (1, 0),
    (-1, -1),
    (-2, 0),
    (2, 0),
    (1, -1),
    (1, 1),
    (-1, 1),
];
const K2: [(i32, i32); 12] = [
    (2, 1),
    (0, -1),
    (-2, 1),
    (1, 0),
    (0, 1),
    (-1, 0),
    (2, 0),
    (1, 1),
    (-1, -1),
    (1, -1),
    (-2, 0),
    (-1, 1),
];
const K3: [(i32, i32); 12] = [
    (0, 1),
    (-2, -1),
    (2, -1),
    (-1, 0),
    (1, 0),
    (0, -1),
    (1, 1),
    (-1, 1),
    (-1, -1),
    (-2, 0),
    (2, 0),
    (1, -1),
];
const K4: [(i32, i32); 12] = [
    (-2, 1),
    (2, 1),
    (0, -1),
    (0, 1),
    (-1, 0),
    (1, 0),
    (-2, 0),
    (-1, 1),
    (2, 0),
    (1, 1),
    (-1, -1),
    (1, -1),
];
const K5: [(i32, i32); 12] = [
    (2, -1),
    (0, 1),
    (-2, -1),
    (1, 0),
    (0, -1),
    (-1, 0),
    (2, 0),
    (1, -1),
    (1, 1),
    (-1, 1),
    (-1, -1),
    (-2, 0),
];

pub const NODE_TO_COORD: [[(i32, i32); 12]; 6] = [K3, K4, K5, K0, K1, K2];

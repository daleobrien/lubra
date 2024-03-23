use crate::bit_utils::sort_upper_bits;
use crate::consts::IN;
use crate::types::coordinate::Coordinate;

//use memoize::memoize;

pub fn rotate_bits_345_left_and_012_right(value: u32) -> u32 {
    // Extract the bits to be rotated.
    let bits_345_1 = value >> 2 & 0b001_000_001_000_001_000_001_000_001_000;
    let bits_345_2 = value << 1 & 0b110_000_110_000_110_000_110_000_110_000;
    let bits_012_1 = value << 2 & 0b000_100_000_100_000_100_000_100_000_100;
    let bits_012_2 = value >> 1 & 0b000_011_000_011_000_011_000_011_000_011;
    return bits_345_1 | bits_345_2 | bits_012_1 | bits_012_2;
}

pub fn rotate_bits_345_right_and_012_left(value: u32) -> u32 {
    let bits_345_1 = value >> 1 & 0b011_000_011_000_011_000_011_000_011_000;
    let bits_345_2 = value << 2 & 0b100_000_100_000_100_000_100_000_100_000;
    let bits_012_1 = value << 1 & 0b000_110_000_110_000_110_000_110_000_110;
    let bits_012_2 = value >> 2 & 0b000_001_000_001_000_001_000_001_000_001;
    return bits_345_1 | bits_345_2 | bits_012_1 | bits_012_2;
}

//#[memoize] // Not sure if this will really help, so leave it out for now.
fn coordinate_to_index_and_flip(coordinate: Coordinate) -> (usize, bool) {
    return (coordinate.i % 3, (coordinate.j + coordinate.i) % 2 == 1);
}

pub fn transform_tile(value: u32, coordinate: Coordinate) -> u32 {
    let (index, flipped) = coordinate_to_index_and_flip(coordinate);
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
    return sort_upper_bits(bits);
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use crate::consts::{N1, N2, N3, N4, N5, N6};

    use super::*;

    #[test]
    fn test_transform_vertex() {
        assert_eq!(transform_tile(N1, Coordinate { i: 0, j: 0 }), N1);
        assert_eq!(transform_tile(N1, Coordinate { i: 1, j: 0 }), IN | N2);
        assert_eq!(transform_tile(N1, Coordinate { i: 2, j: 0 }), N3);

        // Check wrapping around.
        assert_eq!(transform_tile(N1, Coordinate { i: 3, j: 0 }), IN | N1);

        assert_eq!(transform_tile(N1, Coordinate { i: 0, j: 1 }), IN | N1);
        assert_eq!(transform_tile(N1, Coordinate { i: 1, j: 1 }), N2);
        assert_eq!(transform_tile(N1, Coordinate { i: 2, j: 1 }), IN | N3);

        // Other vertexes
        assert_eq!(transform_tile(N2, Coordinate { i: 1, j: 0 }), IN | N3);
        assert_eq!(transform_tile(N3, Coordinate { i: 1, j: 0 }), IN | N1);
    }

    #[test]
    fn test_transform_edges() {
        assert_eq!(transform_tile(N4, Coordinate { i: 0, j: 0 }), N4);
        assert_eq!(transform_tile(N4, Coordinate { i: 1, j: 0 }), IN | N6);
        assert_eq!(transform_tile(N4, Coordinate { i: 2, j: 0 }), N5);

        assert_eq!(transform_tile(N4, Coordinate { i: 0, j: 1 }), IN | N4);
        assert_eq!(transform_tile(N4, Coordinate { i: 1, j: 1 }), N6);
        assert_eq!(transform_tile(N4, Coordinate { i: 2, j: 1 }), IN | N5);

        // Other edges
        assert_eq!(transform_tile(N5, Coordinate { i: 1, j: 0 }), IN | N4);
        assert_eq!(transform_tile(N6, Coordinate { i: 1, j: 0 }), IN | N5);
    }
}

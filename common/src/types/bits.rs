use crate::consts::IN;

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

pub fn transform_tile_orientation(value: u32, index: usize, flipped: bool) -> u32 {
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

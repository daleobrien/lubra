use itertools::Itertools;

use crate::consts::{IN, NL, WA};

pub fn get_mass(nets: &Vec<usize>) -> usize {
    let mut mass = 1 * nets.len();
    for net in nets {
        match net.count_ones() {
            2 => mass *= 5,
            3 => mass *= 5,
            4 => mass *= 2,
            _ => mass *= 1,
        }
    }
    mass
}

pub fn to_bits(nets: &Vec<usize>) -> usize {
    if nets.is_empty() {
        return 0;
    }
    nets.iter().fold(0, |a, b| a | b)
}

pub fn intersects(nets: &Vec<usize>) -> bool {
    let mut combined = 0;
    !nets.is_empty()
        && nets.len() != 1
        && nets.iter().any(|&byte| {
            let intersects = combined & byte != 0;
            combined |= byte;
            intersects
        })
}

pub fn dont_care_to_bits(value: usize) -> usize {
    //# 0b101 -> 0b10_00_10
    let mut out = 0;
    for j in 0..=5 {
        let m = 1 << j;
        if value & m == m {
            let b = 2 << (2 * j);
            out |= b;
        }
    }
    return out;
}

pub fn mask_to_bits(value: usize) -> usize {
    // 0b101 -> 0b01_00_01
    let mut out = 0;
    for j in 0..=5 {
        let m = 1 << j;
        if value & m == m {
            let b = 1 << (2 * j);
            out |= b;
        }
    }
    return out;
}

pub fn to_base4(number: usize) -> usize {
    // e.g. to_base4(5) -> "11"
    if number < 4 {
        return number;
    }
    let mut result = 0;
    let mut decimal = number.clone();
    let mut multiplier = 1;

    while decimal > 0 {
        let digit = decimal % 4;
        result += digit * multiplier;
        multiplier *= 10;
        decimal /= 4;
    }

    return result;
}

pub fn bit_sequence(n: usize) -> Vec<(usize, usize)> {
    return (0..n)
        .into_iter()
        .map(|n| (n, dont_care_to_bits(n)))
        .collect();
}

pub fn to_6bit_binary(value: &usize, reduce: bool) -> String {
    let mut s = format!("{:032b}", value);
    let len = s.len();
    for i in (0..len).rev() {
        if i % 7 == 2 {
            s.insert(len - i, '_');
        }
    }
    let mut text = format!("0b{}", s);
    if reduce {
        for s in vec![
            "0b00_000000_000000_000000_000000_",
            "0b00_000000_000000_000000_",
            "0b00_000000_000000_",
            "0b00_000000_",
        ] {
            if text.starts_with(s) {
                //text = text.replace(s, "0b00_..._");
                text = text.replace(s, "");
            }
        }
        for s in vec![
            "0b01_000000_000000_000000_000000_",
            "0b01_000000_000000_000000_",
            "0b01_000000_000000_",
            "0b01_000000_",
        ] {
            if text.starts_with(s) {
                text = text.replace(s, "0b01_..._");
            }
        }
        for s in vec![
            "0b11_000000_000000_000000_000000_",
            "0b11_000000_000000_000000_",
            "0b11_000000_000000_",
            "0b11_000000_",
        ] {
            if text.starts_with(s) {
                text = text.replace(s, "0b11_..._");
            }
        }
    }

    return text;
}

const SHIFT_VALUES: [usize; 4] = [6, 12, 18, 24];

pub fn sort_upper_bits(value: usize) -> usize {
    // We take all the 6bits nibbles, except the lowest 6bits and sorted them
    SHIFT_VALUES
        .iter()
        .filter_map(|&shift| {
            let val = (value >> shift) & NL;
            if val != 0 {
                Some((val.trailing_zeros(), val))
            } else {
                None
            }
        })
        .sorted()
        .enumerate()
        .fold(value & (NL | WA | IN), |result, (i, (_, val))| {
            result | (val << SHIFT_VALUES[i])
        })
}

fn nibble_to_enum_string(value: usize) -> String {
    (1..=6).fold(String::new(), |mut s, i| {
        if value & (1 << (i - 1)) != 0 {
            s.push_str(&i.to_string());
        }
        s
    })
}

const ALL_SHIFT_VALUES: [usize; 5] = [0, 6, 12, 18, 24];

pub fn print_bits(value: usize) -> String {
    ALL_SHIFT_VALUES
        .iter()
        .copied() // Use copied() to avoid creating references
        .map(|shift| (value >> shift) & NL)
        .filter(|&x| x != 0) // Exclude values that are all zeros
        .map(|x| nibble_to_enum_string(x))
        .join("/")
}

pub fn print_nets(nets: &Vec<usize>) -> String {
    nets.iter().map(|x| print_bits(*x)).join(" ")
}

#[cfg(test)]
mod tests {
  use std::{assert_eq, format, vec};

  use crate::consts::{A1, A3, A4, A5, B2, B3, B4, B6, N1, N2, N3, N4, N5, N6};

  use super::*;

  #[test]
    fn test_nibble_to_enum_string() {
        assert_eq!(nibble_to_enum_string(N1), "1");
        assert_eq!(nibble_to_enum_string(N2), "2");
        assert_eq!(nibble_to_enum_string(N3), "3");
        assert_eq!(nibble_to_enum_string(N4), "4");
        assert_eq!(nibble_to_enum_string(N5), "5");
        assert_eq!(nibble_to_enum_string(N6), "6");
    }

    #[test]
    fn test_print_nodes() {
        assert_eq!(print_bits(N2 | N5), "25");
        assert_eq!(print_bits(N2 | N6), "26");
        assert_eq!(print_bits(N2 | N6 | A1 | A5 | B3 | B6), "26/15/36");
    }

    #[test]
    fn test_print_nets() {
        let n = vec![N2 | N5, N1 | N6];
        assert_eq!(print_nets(&n), "25 16");
    }
    #[test]
    fn test_sort_upper_bits() {
        assert_eq!(sort_upper_bits(WA | N2), WA | N2);
        assert_eq!(sort_upper_bits(IN | WA | N2 | A3), IN | WA | N2 | A3);
        assert_eq!(
            sort_upper_bits(IN | WA | N2 | A3 | B4),
            IN | WA | N2 | A3 | B4
        );
        assert_eq!(
            sort_upper_bits(IN | WA | N1 | A4 | B3),
            IN | WA | N1 | A3 | B4
        );
        assert_eq!(
            sort_upper_bits(IN | WA | N4 | N5 | A1 | A5 | B2 | B4),
            IN | WA | N4 | N5 | A1 | A5 | B2 | B4
        );
    }
    #[test]
    fn test_to_bits() {
        assert_eq!(to_bits(&vec![]), 0);

        assert_eq!(to_bits(&vec![2]), 2);
        assert_eq!(to_bits(&vec![1, 2]), 3);

        let b = to_bits(&vec![3, 0b100_000]);
        assert_eq!(format!("{:b}", b), "100011");
    }

    #[test]
    fn test_dont_care_to_bits() {
        // 101 -> 10 00 10
        let b = dont_care_to_bits(5);
        assert_eq!(format!("{:b}", b), "100010");
    }

    #[test]
    fn test_mask_to_bits() {
        // 0b101 -> 0b01_00_01
        let b = mask_to_bits(5);
        assert_eq!(format!("{:b}", b), "10001");
    }
    #[test]
    fn test_to_base4() {
        let b = to_base4(5);
        assert_eq!(b, 11);

        let b = to_base4(3);
        assert_eq!(b, 3);

        let b = to_base4(13414);
        assert_eq!(b, 3101212);
    }

    #[test]
    fn test_intersections() {
        assert_eq!(intersects(&vec![]), false);
        assert_eq!(intersects(&vec![0b010_101]), false);
        assert_eq!(intersects(&vec![0b000_001, 0b000_010]), false);
        assert_eq!(intersects(&vec![0b000_001, 0b000_011]), true);
        assert_eq!(intersects(&vec![0b000_001, 0b000_010, 0b000_100]), false);
        assert_eq!(
          intersects(&vec![0b000_001, 0b000_010, 0b000_100, 0b001_000]),
          false
        );
        assert_eq!(
          intersects(&vec![0b000_001, 0b001_000, 0b000_010, 0b001_000]),
          true
        );
    }

    #[test]
    fn test_bit_sequence() {
        let sequence = bit_sequence(10);
        assert_eq!(
            sequence,
            vec![
                (0, 0),
                (1, 2),
                (2, 8),
                (3, 10),
                (4, 32),
                (5, 34),
                (6, 40),
                (7, 42),
                (8, 128),
                (9, 130)
            ]
        );
    }
}

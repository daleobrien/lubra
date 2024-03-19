pub fn get_mass(nets: &Vec<u32>) -> u32 {
    let mut mass = 1;
    for net in nets {
        match net.count_ones() {
            2 => mass *= 11,
            3 => mass *= 10,
            _ => (),
        }
        if *net == 0b000_111 {
            mass *= 12;
        }
    }
    return mass;
}

pub fn to_bits(nets: &Vec<u32>) -> u32 {
    if nets.is_empty() {
        return 0u32;
    }
    return nets.iter().fold(0, |a, b| a | b);
}

pub fn interects(nets: &Vec<u32>) -> bool {
    if nets.is_empty() || nets.len() == 1 {
        return false;
    }
    if nets.len() == 2 {
        return nets[0] & nets[1] != 0;
    }

    for i in 0..nets.len() - 1 {
        for j in i + 1..nets.len() {
            if nets[i] & nets[j] != 0 {
                return true;
            }
        }
    }

    return false;
}

pub fn dont_care_to_bits(value: u32) -> u32 {
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

pub fn mask_to_bits(value: u32) -> u32 {
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

pub fn to_base4(number: u32) -> u32 {
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

pub fn bit_sequence(n: u32) -> Vec<(u32, u32)> {
    return (0..n)
        .into_iter()
        .map(|n| (n, dont_care_to_bits(n)))
        .collect();
}

pub fn to_6bit_binary(value: &u32) -> String {
    let mut s = format!("{:032b}", value);
    let len = s.len();
    for i in (0..len).rev() {
        if i % 7 == 2 {
            s.insert(len - i, '_');
        }
    }
    let mut text = format!("0b{}", s);
    for s in vec![
        "0b00_000000_000000_000000_000000_",
        "0b00_000000_000000_000000_",
        "0b00_000000_000000_",
        "0b00_000000_",
    ] {
        if text.starts_with(s) {
            text = text.replace(s, "0b00_..._");
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

    return text;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{assert_eq, format, vec};

    #[test]
    fn test_to_bits() {
        assert_eq!(to_bits(&vec![]), 0);

        assert_eq!(to_bits(&vec![2u32]), 2);
        assert_eq!(to_bits(&vec![1u32, 2u32]), 3);

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
        assert_eq!(interects(&vec![]), false);
        assert_eq!(interects(&vec![0b010_101]), false);
        assert_eq!(interects(&vec![0b000_001, 0b000_010]), false);
        assert_eq!(interects(&vec![0b000_001, 0b000_011]), true);
        assert_eq!(interects(&vec![0b000_001, 0b000_010, 0b000_100]), false);
        assert_eq!(
            interects(&vec![0b000_001, 0b000_010, 0b000_100, 0b001_000]),
            false
        );
        assert_eq!(
            interects(&vec![0b000_001, 0b000_010, 0b001_000, 0b001_000]),
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

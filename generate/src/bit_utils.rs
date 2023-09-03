pub fn get_mass(nets: Vec<Vec<u32>>) -> u32 {
    let mut mass = 1;
    for net in nets {
        match net.len() {
            2 => mass *= 4,
            3 => mass *= 5,
            _ => (),
        }
    }
    return mass;
}

pub fn to_bits(nets: Vec<Vec<u32>>) -> u32 {
    let nodes = nets.clone().into_iter().flatten().collect::<Vec<u32>>();

    let mut bits = 0;
    for n in nodes {
        bits |= 1 << (n - 1);
    }
    return bits;
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::{assert_eq, format, vec};

    #[test]
    fn test_to_bits() {
        assert_eq!(to_bits(vec![]), 0);
        assert_eq!(to_bits(vec![vec![2]]), 2);
        assert_eq!(to_bits(vec![vec![1, 2]]), 3);

        let b = to_bits(vec![vec![1, 2], vec![6]]);
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

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

#[cfg(test)]
mod tests {
    use super::*;

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
}

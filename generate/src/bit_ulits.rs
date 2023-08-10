fn to_bits(nets: Vec<Vec<u32>>) -> u32 {
    let nodes = nets.clone().into_iter().flatten().collect::<Vec<u32>>();

    let mut bits = 0;
    for n in nodes {
        bits |= 1 << (n - 1);
    }
    return bits;
}


fn dont_care_to_trit(bits:u32)-> str {
//# 0b101 -> 0b10_00_10
let mut trits = 0;
for j in 1..=6 {
    let b = 2 << (2 * j);
    if bits & (1 << j) == (1 << j){
        trits |=b;
    }
}
return trits
}


fn mask_to_trit(bits:u32)-> str {
    // 0b101 -> 0b01_00_01
    let mut trits = 0
    for j in 1..=6 {
        let b = 1 << (2 * j);
        if bits & (1 << j) == (1 << j){
            trits |= b
        }
    }
    return trits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_bits() {
        assert_eq!(to_bits(vec![]), 0);
        assert_eq!(to_bits(vec![vec![2]]), 2);
    }
}

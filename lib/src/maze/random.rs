use crate::maze::nets::list_of_posible_nets_for_a_mask;
use rand::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand_chacha::ChaCha8Rng;

pub fn random_tile(mask: u32, seed: Option<u64>) -> Vec<Vec<u8>> {
    let possible_nets = list_of_posible_nets_for_a_mask(mask);

    let mut rng = match seed {
        Some(s) => ChaCha8Rng::seed_from_u64(s),
        None => ChaCha8Rng::from_rng(thread_rng()).unwrap(),
    };

    let possible_tile = possible_nets.choose(&mut rng);
    match possible_tile {
        Some(tile) => return tile.clone(),
        None => return vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::maze::nets::list_of_posible_nets_for_a_mask;
    use std::assert_eq;

    #[test]
    fn test_random_tile() {
        let mask = 1;
        let tile = random_tile(mask, None);
        let possible_nets = list_of_posible_nets_for_a_mask(mask);
        assert_eq!(possible_nets[0], tile);
    }
}

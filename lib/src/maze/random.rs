use crate::maze::nets::list_of_posible_nets_for_a_mask;
use rand::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand_chacha::ChaCha8Rng;

fn max_length(lists: &Vec<Vec<u8>>) -> usize {
    return lists
        .iter()
        .map(|list| list.len())
        .max()
        .unwrap_or_default();
}

pub fn _tile_for_mask(mask: u32, possible_nets: Vec<Vec<Vec<u8>>>) -> Vec<Vec<u8>> {
    let possible_tile = possible_nets.choose(&mut thread_rng());
    return match possible_tile {
        Some(tile) => return tile.clone(),
        None => return vec![],
    };
}

pub fn random_tile(mask: u32, seed: Option<u64>) -> Vec<Vec<u8>> {
    let possible_nets = list_of_posible_nets_for_a_mask(mask);

    match seed {
        Some(s) => ChaCha8Rng::seed_from_u64(s),
        None => ChaCha8Rng::from_rng(thread_rng()).unwrap(),
    };

    // Favour complex tiles
    let mut count = 0;
    while count < 10 {
        count += 1;
        let nets = _tile_for_mask(mask, possible_nets.clone());
        // Maybe better to count the number of nets that are length 1, since we don't want too many
        // of those.

        // Complex
        if max_length(&nets) >= 3 {
            return nets;
        }
        // More complex
        if max_length(&nets) >= 2 && count > 5 {
            return nets;
        }
    }

    return _tile_for_mask(mask, possible_nets);
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::maze::nets::list_of_posible_nets_for_a_mask;
//     use std::assert_eq;

//     #[test]
//     fn test_random_tile() {
//         let mask = 1;
//         let tile = random_tile(mask, None);
//         let possible_nets = list_of_posible_nets_for_a_mask(mask);
//         assert_eq!(possible_nets[0], tile);
//     }
// }

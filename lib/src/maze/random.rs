use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use weighted_rand::builder::{NewBuilder, WalkerTableBuilder};

use crate::maze::generated::mass_large::list_of_possible_mass_for_a_mask as mass_large;
use crate::maze::generated::mass_small::list_of_possible_mass_for_a_mask as mass_small;
use crate::maze::generated::nets_large::list_of_possible_nets_for_a_mask as nets_large;
use crate::maze::generated::nets_small::list_of_possible_nets_for_a_mask as nets_small;

pub fn random_tile(mask: usize, seed: Option<u64>, reduced_lookup: bool) -> Vec<usize> {
    let possible_nets = if reduced_lookup {
        nets_small(mask)
    } else {
        nets_large(mask)
    };

    if possible_nets.len() == 1 {
        return possible_nets[0].to_vec();
    }

    let possible_mass = if reduced_lookup {
        mass_small(mask)
    } else {
        mass_large(mask)
    };

    let mut rng = match seed {
        Some(s) => ChaCha8Rng::seed_from_u64(s),
        None => ChaCha8Rng::from_rng(thread_rng()).unwrap(),
    };

    // Weighted Random pick
    let builder = WalkerTableBuilder::new(possible_mass);
    let nets_table = builder.build();

    let index = nets_table.next_rng(&mut rng);
    let mut nets = possible_nets[index].to_vec();
    // Shuffle the nets so that the render order changes
    nets.shuffle(&mut rng);
    nets
}

/// Selects a random element from a vector, seeded when `seed` is given.
pub fn select_random_element(vec: Vec<usize>, seed: Option<u64>) -> Option<usize> {
    if vec.is_empty() {
        return None;
    }

    let mut rng = match seed {
        Some(s) => ChaCha8Rng::seed_from_u64(s),
        None => ChaCha8Rng::from_rng(thread_rng()).unwrap(),
    };

    let index = rng.gen_range(0..vec.len());
    Some(vec[index])
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::bit_utils::get_mass;

    /// Every constraint mask: one decimal digit (0, 1 or 2) per node.
    fn all_masks() -> impl Iterator<Item = usize> {
        (0..729).map(|n: usize| {
            (0..6).fold(0, |mask, digit| {
                mask + (n / 3usize.pow(digit) % 3) * 10usize.pow(digit)
            })
        })
    }

    #[test]
    fn test_mass_tables_match_net_tables() {
        for mask in all_masks() {
            for (nets, mass) in [
                (nets_small(mask), mass_small(mask)),
                (nets_large(mask), mass_large(mask)),
            ] {
                if mass.is_empty() {
                    continue; // mask not in the table
                }
                assert_eq!(nets.len(), mass.len(), "mask {mask}");
                for (net_set, &weight) in nets.iter().zip(mass) {
                    let expected = get_mass(net_set) as u32;
                    assert_eq!(weight, expected, "mask {mask}, nets {net_set:?}");
                }
            }
        }
    }
}

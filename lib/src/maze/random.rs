use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use weighted_rand::builder::{NewBuilder, WalkerTableBuilder};

use crate::maze::generated::mass_large::list_of_possible_mass_for_a_mask as mass_large;
use crate::maze::generated::mass_small::list_of_possible_mass_for_a_mask as mass_small;
use crate::maze::generated::nets_large::list_of_possible_nets_for_a_mask as nets_large;
use crate::maze::generated::nets_small::list_of_possible_nets_for_a_mask as nets_small;
use crate::maze::maze::touching::crossing_in_same_tile;

pub fn random_tile(mask: usize, seed: Option<u64>, reduced_lookup: bool) -> Vec<usize> {
    random_tile_with_crossing_weight(mask, seed, reduced_lookup, 1.0)
}

/// As `random_tile`, with each choice that has tracks crossing in the tile weighted
/// `crossing_weight` times as much as its mass. A weight of 1 or less leaves the
/// masses as they are (and the pick exactly as `random_tile` makes it).
pub fn random_tile_with_crossing_weight(
    mask: usize,
    seed: Option<u64>,
    reduced_lookup: bool,
    crossing_weight: f64,
) -> Vec<usize> {
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
    let nets_table = if crossing_weight > 1.0 {
        let mass: Vec<u32> = possible_nets
            .iter()
            .zip(possible_mass)
            .map(|(&nets, &mass)| {
                if crosses(nets) {
                    (f64::from(mass) * crossing_weight).round() as u32
                } else {
                    mass
                }
            })
            .collect();
        WalkerTableBuilder::new(&mass).build()
    } else {
        WalkerTableBuilder::new(possible_mass).build()
    };

    let index = nets_table.next_rng(&mut rng);
    let mut nets = possible_nets[index].to_vec();
    // Shuffle the nets so that the render order changes
    nets.shuffle(&mut rng);
    nets
}

/// As `random_tile`, but leaving out any choice with tracks that cross (one passing
/// under another), when there is one without. Used for mazes with no crossings.
pub fn random_tile_without_crossings(
    mask: usize,
    seed: Option<u64>,
    reduced_lookup: bool,
) -> Vec<usize> {
    let (possible_nets, possible_mass) = if reduced_lookup {
        (nets_small(mask), mass_small(mask))
    } else {
        (nets_large(mask), mass_large(mask))
    };
    let keep: Vec<usize> = (0..possible_nets.len())
        .filter(|&index| !crosses(possible_nets[index]))
        .collect();
    if keep.is_empty() {
        return random_tile(mask, seed, reduced_lookup);
    }
    if keep.len() == 1 {
        return possible_nets[keep[0]].to_vec();
    }

    let mut rng = match seed {
        Some(s) => ChaCha8Rng::seed_from_u64(s),
        None => ChaCha8Rng::from_rng(thread_rng()).unwrap(),
    };
    // Weighted random pick among the choices kept; with no weight among them
    // (only empty tiles), each is as likely.
    let mut mass: Vec<u32> = keep
        .iter()
        .map(|&index| possible_mass.get(index).copied().unwrap_or(0))
        .collect();
    if mass.iter().all(|&weight| weight == 0) {
        mass.fill(1);
    }
    let nets_table = WalkerTableBuilder::new(&mass).build();
    let mut nets = possible_nets[keep[nets_table.next_rng(&mut rng)]].to_vec();
    // Shuffle the nets so that the render order changes
    nets.shuffle(&mut rng);
    nets
}

/// Whether any two of a tile's nets cross.
fn crosses(nets: &[usize]) -> bool {
    nets.iter().enumerate().any(|(k, &net)| {
        nets[k + 1..]
            .iter()
            .any(|&other| crossing_in_same_tile(net, other))
    })
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

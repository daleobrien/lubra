use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use weighted_rand::builder::{NewBuilder, WalkerTableBuilder};

use crate::maze::generated::mass_large::list_of_possible_mass_for_a_mask as mass_large;
use crate::maze::generated::mass_small::list_of_possible_mass_for_a_mask as mass_small;
use crate::maze::generated::nets_large::list_of_possible_nets_for_a_mask as nets_large;
use crate::maze::generated::nets_small::list_of_possible_nets_for_a_mask as nets_small;

pub fn random_tile(mask: u32, seed: Option<u64>, reduced_lookup: bool) -> Vec<u32> {
    let possible_nets = if reduced_lookup {
        nets_small(mask)
    } else {
        nets_large(mask)
    };

    if possible_nets.len() == 1 {
        return possible_nets[0].clone();
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
    let builder = WalkerTableBuilder::new(&possible_mass);
    let nets_table = builder.build();

    let index = nets_table.next_rng(&mut rng);
    let mut nets = if reduced_lookup {
        possible_nets[index].clone()
    } else {
        possible_nets[index].clone()
    };
    // Shuffle the nets so that the render order changes
    nets.shuffle(&mut rng);
    return nets;
}

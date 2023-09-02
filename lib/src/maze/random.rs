use crate::maze::mass::list_of_possible_mass_for_a_mask;
use crate::maze::nets::list_of_possible_nets_for_a_mask;
use rand::prelude::*;
use rand::thread_rng;
use rand_chacha::ChaCha8Rng;
use weighted_rand::builder::{NewBuilder, WalkerTableBuilder};

pub fn random_tile(mask: u32, seed: Option<u64>) -> Vec<Vec<u32>> {
    let possible_nets = list_of_possible_nets_for_a_mask(mask);

    if possible_nets.len() == 1 {
        return possible_nets[0].clone();
    }

    let possible_mass = list_of_possible_mass_for_a_mask(mask);

    let mut rng = match seed {
        Some(s) => ChaCha8Rng::seed_from_u64(s),
        None => ChaCha8Rng::from_rng(thread_rng()).unwrap(),
    };

    // Weighted Random pick
    let builder = WalkerTableBuilder::new(&possible_mass);
    let nets_table = builder.build();

    let index = nets_table.next_rng(&mut rng);
    return possible_nets[index].clone();
}

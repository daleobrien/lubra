/*
 This file is generated from a template, ... edit the template instead.
*/
use crate::maze::tile::Tile;
use rand::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand_chacha::ChaCha8Rng;

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn find_tiles(mask:u32) -> Vec<u32> {
	match mask { {% for key in side_keys %}
		0b{{side_key_as_binary[key]}} => vec!{{side[key]}},{% endfor %}
	   _ => vec!(),
   }
}

pub fn random_side(matches: u32, dont_cares: u32, seed: Option<u64>) -> Tile {
    let mut index: u32 = 0;
    for i in 0..6 {
        let b: u32 = 1 << i;
        if dont_cares & b == b {
            index |= 2 << (i * 2);
        } else if matches & b == b {
            index |= 1 << (i * 2);
        }
    }
    let possible_tiles = find_tiles(index);

    let mut rng = match seed {
        Some(s) => ChaCha8Rng::seed_from_u64(s),
        None => ChaCha8Rng::from_rng(thread_rng()).unwrap(),
    };
    let tile = possible_tiles.choose(&mut rng);
    return Tile::from_bits_truncate(*tile.unwrap_or(&Tile::Z1.bits()));
}

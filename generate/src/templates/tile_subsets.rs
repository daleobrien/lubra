/*
 This file is generated from a template, ... edit the template instead.
*/
use rand::seq::SliceRandom;
use crate::maze::constants::Tile;

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn find_tiles (mask:u32) -> Vec<u32> {
	match mask { {% for key in side_keys %}
		0b{{side_key_as_binary[key]}} => vec!{{side[key]}}, {% endfor %}
	   _ => vec!(),
  }
}

pub fn random_side (matches:u32, dont_cares: u32) -> Tile {
  let mut index:u32 = 0;
  for i in 0..6 {
      let b:u32 = 1 << i;
      if dont_cares & b == b {
        index |= 2 << (i*2);
      } else if matches & b == b {
        index |= 1 << (i*2);
      }
  }
	let possible_tiles = find_tiles(index);
	let tile = possible_tiles.choose(&mut rand::thread_rng());
	return Tile::from_bits_truncate(*tile.unwrap_or(&Tile::Z1.bits()))
}

/*
 This file is generated from a template, ... edit the template instead.
*/
use crate::maze::tile::Tile;

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn find_tiles (tile:Tile) -> [u32; 6] {
	match tile { {% for tile in tiles %}
	   {{ tile }} => [0,0,0,0,0,0],{% endfor %}
       _ => [0,0,0,0,0,0]
	}
}

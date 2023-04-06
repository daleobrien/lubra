/*
 This file is generated from a template, ... edit the template instead.
*/
use crate::maze::tile::Tile;

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn find_tiles (tile:u32) -> Vec<Vec<u32>> {
	match tile { {% for key in keys %}
	   0b{{ side_key_as_binary[key] }} => vec![{% for t in tiles[key] %} vec!{{ t }}{% if loop.last != true %},{% else %} {% endif %}{% endfor %}],{% endfor %}
       _ => vec![],
	}
}

mod bit_utils;
mod frag_12;
mod frag_123;
mod frag_15;
mod frag_155;
mod frag_15_dot;
mod frag_15_long;
mod frag_15_mid;
mod frag_15_straight;
mod frag_15_tight;
mod frag_16;
mod frag_45_halfs;
mod frag_arcs;
mod frag_math;
mod frag_wall;
mod lookups;
mod node_combinations;
mod path_adj;
mod path_blank;
mod svg_fragments;

use crate::path_adj::generate_adjacent_paths;
use crate::path_blank::generate_blank_paths;
use crate::svg_fragments::generate_all_shapes;

fn main() {
    println!("Generating Rust code for all SVG nets.");
    generate_all_shapes();
    generate_adjacent_paths();
    generate_blank_paths();
}

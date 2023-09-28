mod bit_utils;
mod frag_12;
mod frag_123;
mod frag_15;
mod frag_15_dot;
mod frag_15_long;
mod frag_15_mid;
mod frag_15_straight;
mod frag_15_tight;
mod frag_16;
mod frag_45_halfs;
mod frag_arcs;
mod frag_blank;
mod frag_math;
mod frag_wall;
mod lookups;
mod node_combinations;
mod svg_fragments;

use crate::svg_fragments::generate_all_shapes;

fn main() {
    println!("Generating Rust code for all SVG nets.");
    generate_all_shapes();
}

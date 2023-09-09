mod bit_utils;
mod lookups;
mod node_combinations;
mod svg_fragments;
mod writer;

use crate::svg_fragments::generate_all_shapes;

fn main() {
    println!("Generating Rust code for all SVG nets.");
    generate_all_shapes();
}

mod bit_utils;
mod data;
mod lookups;
mod node_combinations;
mod writer;

use crate::data::generate_all_shapes;

fn main() {
    println!("Generating Rust code for all SVG nets.");
    generate_all_shapes();
}

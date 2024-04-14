use crate::path_adj::generate_adjacent_paths;
use crate::svg_fragments::generate_all_shapes;

mod frag_1;
mod frag_12;
mod frag_123;
mod frag_125;
mod frag_135;
mod frag_145;
mod frag_15;
mod frag_155;
mod frag_156;
mod frag_15_dot;
mod frag_15_long;
mod frag_15_straight;
mod frag_16;
mod frag_arc_halfs;
mod frag_arc_long;
mod frag_arc_semi;
mod frag_arcs;
mod frag_complex_arcs;
mod frag_math;
mod frag_wall;
mod lookups;
mod path_adj;
mod svg_fragments;

fn main() {
    println!("Generating Rust code for all SVG nets.");
    generate_all_shapes();
    generate_adjacent_paths();
}

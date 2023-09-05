mod bit_utils;
mod data;
mod lookups;
mod node_combinations;
mod writer;

use crate::data::generate_all_shapes;
use crate::node_combinations::NodeCombinations;
use crate::writer::{write_mass_to_file, write_nets_to_file};
use std::collections::HashMap;

fn main() {
    println!("Generating Rust code for all possible nets.");
    generate_all_shapes();

    let seq = NodeCombinations::new(vec![
        vec![1], // Points
        vec![2],
        vec![3],
        vec![4],
        vec![5],
        vec![6],
        vec![4, 5], // Arcs
        vec![5, 6],
        vec![4, 6],
        vec![4, 5, 6],
        vec![1, 6], // Lines
        vec![2, 5],
        vec![3, 4],
        vec![1, 2], // Almost lines, vertex to vertex
        vec![2, 3],
        vec![1, 3],
        vec![1, 2, 3], // All vertexes connected
    ]);

    let mut nets_by_mask: HashMap<u32, Vec<Vec<Vec<u32>>>> = HashMap::new();
    let mut mass_by_mask: HashMap<u32, Vec<u32>> = HashMap::new();

    lookups::create_lookups(seq, &mut nets_by_mask, &mut mass_by_mask);

    write_nets_to_file(nets_by_mask, "../lib/src/maze/nets.rs");
    write_mass_to_file(mass_by_mask, "../lib/src/maze/mass.rs");

    let seq = NodeCombinations::new(vec![
        vec![4, 5], // Arcs
        vec![5, 6],
        vec![4, 6],
        vec![4, 5, 6],
        vec![1, 6], // Lines
        vec![2, 5],
        vec![3, 4],
        vec![1, 2], // Almost lines, vertex to vertex
        vec![2, 3],
        vec![1, 3],
        vec![1, 2, 3], // All vertexes connected
    ]);

    let mut nets_by_mask: HashMap<u32, Vec<Vec<Vec<u32>>>> = HashMap::new();
    let mut mass_by_mask: HashMap<u32, Vec<u32>> = HashMap::new();

    lookups::create_lookups(seq, &mut nets_by_mask, &mut mass_by_mask);

    write_nets_to_file(nets_by_mask, "../lib/src/maze/nets_reducded.rs");
    write_mass_to_file(mass_by_mask, "../lib/src/maze/mass_redcuded.rs");
}

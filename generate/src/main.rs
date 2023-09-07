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
    println!("Generating Rust code for all SVG nets.");
    generate_all_shapes();

    println!("Generating Rust code for all Lookups nets.");

    let seq = NodeCombinations::new(vec![
        0b000_001, 0b000_010, 0b000_100, 0b001_000, 0b010_000, 0b100_000, // Points
        0b011_000, 0b110_000, 0b101_000, 0b111_000, // Arcs
        0b100_001, 0b010_010, 0b001_100, // Lines
        0b000_011, 0b000_110, 0b000_101, // Almost lines, vertex to vertex
        0b000_111, // All vertexes connected
        0b001_001, 0b010_001, 0b001_010, 0b100_010, 0b010_100, 0b100_100,
    ]);

    let mut nets_by_mask: HashMap<u32, Vec<Vec<Vec<u32>>>> = HashMap::new();
    let mut mass_by_mask: HashMap<u32, Vec<u32>> = HashMap::new();

    lookups::create_lookups(seq, &mut nets_by_mask, &mut mass_by_mask);

    write_nets_to_file(nets_by_mask, "../lib/src/maze/nets_large.rs");
    write_mass_to_file(mass_by_mask, "../lib/src/maze/mass_large.rs");

    // Instead of a vec![], could have a bitmask!
    // e.g. vec![vec![1, 2], vec![6]] -> vec![0b000_011, 0b100_000] -> vec![3, 32], maybe a tuple?
    // [3,32,0,0,0,0] since we have a max of 6 layers
    // it would pack into 64 bits, 6*u8
    // or a large number, 0b000_011__100_000__000_000__000_000__000_000__000_000 < 64bit number
    // some matrix!
    // [[u32; 6]; 6] or [u32; 36]?
    // So that would be compact, but we'd need to do a lot of bit shifting to get the values out

    // let mut nets_by_mask: HashMap<u32, Vec<Vec<Vec<u32>>>> = HashMap::new();
    // let mut mass_by_mask: HashMap<u32, Vec<u32>> = HashMap::new();
    //
    // let seq = NodeCombinations::new(vec![
    //     vec![4, 5], // Arcs
    //     vec![5, 6],
    //     vec![4, 6],
    //     vec![4, 5, 6],
    //     vec![1, 6], // Lines
    //     vec![2, 5],
    //     vec![3, 4],
    //     vec![1, 2], // Almost lines, vertex to vertex
    //     vec![2, 3],
    //     vec![1, 3],
    //     vec![1, 2, 3], // All vertexes connected
    //     vec![1, 4],    // Vertex to edge short
    //     vec![1, 5],
    //     vec![2, 4],
    //     vec![2, 6],
    //     vec![3, 5],
    //     vec![3, 6],
    // ]);
    //
    // lookups::create_lookups(seq, &mut nets_by_mask, &mut mass_by_mask);
    //
    // write_nets_to_file(nets_by_mask, "../lib/src/maze/nets_small.rs");
    // write_mass_to_file(mass_by_mask, "../lib/src/maze/mass_small.rs");
}

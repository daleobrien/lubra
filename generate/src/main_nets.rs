mod lookups;
mod writer;

use crate::writer::{write_mass_to_file, write_nets_to_file};
use common::node_combinations::NodeCombinations;
use std::collections::HashMap;

fn main() {
    println!("Generating Rust code for all lookups nets (large network).");

    let seq = NodeCombinations::new(vec![
        0b000_001, 0b000_010, 0b000_100, 0b001_000, 0b010_000, 0b100_000, // Points
        0b011_000, 0b110_000, 0b101_000, 0b111_000, // Arcs
        0b100_001, 0b010_010, 0b001_100, // Lines
        0b000_011, 0b000_110, 0b000_101, // Almost lines, vertex to vertex
        0b000_111, // All vertexes connected
        0b010_001, 0b001_001, 0b001_010, 0b100_010, 0b010_100, 0b100_100,
    ]);

    let mut nets_by_mask: HashMap<u32, Vec<Vec<u32>>> = HashMap::new();
    let mut mass_by_mask: HashMap<u32, Vec<u32>> = HashMap::new();

    lookups::create_lookups(seq, &mut nets_by_mask, &mut mass_by_mask);

    write_nets_to_file(nets_by_mask, "../lib/src/maze/nets_large.rs");
    write_mass_to_file(mass_by_mask, "../lib/src/maze/mass_large.rs");

    println!("Generating Rust code for all lookups nets (small network).");
    let seq = NodeCombinations::new(vec![
        0b011_000, 0b110_000, 0b101_000, 0b111_000, // Arcs
        0b100_001, 0b010_010, 0b001_100, // Lines
        0b000_011, 0b000_110, 0b000_101, // Almost lines, vertex to vertex
        0b000_111, // All vertexes connected
        0b001_001, 0b010_001, 0b001_010, 0b100_010, 0b010_100, 0b100_100,
    ]);

    let mut nets_by_mask: HashMap<u32, Vec<Vec<u32>>> = HashMap::new();
    let mut mass_by_mask: HashMap<u32, Vec<u32>> = HashMap::new();

    lookups::create_lookups(seq, &mut nets_by_mask, &mut mass_by_mask);

    write_nets_to_file(nets_by_mask, "../lib/src/maze/nets_small.rs");
    write_mass_to_file(mass_by_mask, "../lib/src/maze/mass_small.rs");
}

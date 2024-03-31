use std::collections::HashMap;

use common::node_combinations::NodeCombinations;

use crate::writer::{write_mass_to_file, write_nets_to_file};

mod lookups;
mod writer;

fn main() {
    println!("Generating Rust code for all lookups nets (large network).");

    let seq = NodeCombinations::new(
        vec![
            // Points
            0b000_001, 0b000_010, 0b000_100, 0b001_000, 0b010_000, 0b100_000, //
            // Lines
            0b011_000, 0b110_000, 0b101_000, 0b100_001, 0b010_010, 0b001_100, //
            0b000_011, 0b000_110, 0b000_101, // Almost lines, vertex to vertex
            0b010_001, 0b001_001, 0b001_010, 0b100_010, 0b010_100, 0b100_100, //
            // 3-ways
            0b000_111,
            0b111_000,
            // Additional 3-ways
            // 0b010_101, 0b100_110, 0b001_011, //
            // 0b110_001, 0b101_001, 0b011_100, 0b101_100, 0b110_010, 0b011_010, //
            // 0b010_011, 0b010_110, 0b100_011, 0b100_101, 0b001_101, 0b001_110, //
            // 0b011_001, 0b101_010, 0b110_100, //
        ],
        0,
    );

    // Combinations, with duplicates => n! +  ...
    // Given it is related to factorial, it is not possible to calculate it for large numbers
    // but we are limited but the number of bits, e.g. n < 6 * 2^6 * 6

    let mut nets_by_mask: HashMap<u32, Vec<Vec<u32>>> = HashMap::new();
    let mut mass_by_mask: HashMap<u32, Vec<u32>> = HashMap::new();

    let count = lookups::create_lookups(seq, &mut nets_by_mask, &mut mass_by_mask);
    println!("Generated {} lookups.", count);

    write_nets_to_file(nets_by_mask, "../lib/src/maze/nets_large.rs");
    write_mass_to_file(mass_by_mask, "../lib/src/maze/mass_large.rs");

    println!("Generating Rust code for all lookups nets (small network).");
    let seq = NodeCombinations::new(
        vec![
            // Lines
            0b011_000, 0b110_000, 0b101_000, 0b100_001, 0b010_010, 0b001_100, //
            //
            0b000_011, 0b000_110, 0b000_101, //
            // Almost lines, vertex to vertex
            0b001_001, 0b010_001, 0b001_010, 0b100_010, 0b010_100, 0b100_100,
            // 3 way
            0b000_111, 0b111_000,
        ],
        0,
    );

    let mut nets_by_mask: HashMap<u32, Vec<Vec<u32>>> = HashMap::new();
    let mut mass_by_mask: HashMap<u32, Vec<u32>> = HashMap::new();

    lookups::create_lookups(seq, &mut nets_by_mask, &mut mass_by_mask);

    write_nets_to_file(nets_by_mask, "../lib/src/maze/nets_small.rs");
    write_mass_to_file(mass_by_mask, "../lib/src/maze/mass_small.rs");
}

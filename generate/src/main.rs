use std::env;
use std::collections::HashMap;

use common::node_combinations::NodeCombinations;

mod fragments;
mod generators;
mod writer;

use generators::path_adj::generate_adjacent_paths;
use generators::svg_fragments::generate_all_shapes;
use generators::lookups;
use writer::{write_mass_to_file, write_nets_to_file};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "svg" => generate_svg(),
            "nets" => generate_nets(),
            _ => {
                eprintln!("Usage: {} [svg|nets]", args[0]);
                eprintln!("  svg  - Generate SVG fragments");
                eprintln!("  nets - Generate net lookup tables");
                eprintln!("  (no args) - Generate both");
                std::process::exit(1);
            }
        }
    } else {
        // Generate both by default
        generate_svg();
        generate_nets();
    }
}

fn generate_svg() {
    println!("Generating Rust code for all SVG nets.");
    generate_all_shapes();
    generate_adjacent_paths();
}

fn generate_nets() {
    println!("Generating Rust code for all lookups nets (large network).");

    let seq = NodeCombinations::new(
        vec![
            // Points
            0b000_001, 0b000_010, 0b000_100, 0b001_000, 0b010_000, 0b100_000, //
            // Lines
            0b000_011, 0b000_101, 0b000_110, 0b001_001, 0b001_010, 0b001_100, 0b010_001, 0b010_010,
            0b010_100, 0b011_000, 0b100_001, 0b100_010, 0b100_100, 0b101_000, 0b110_000, //
            // 3-ways
            0b000_111, 0b001_011, 0b001_101, 0b001_110, 0b010_011, 0b010_101, 0b010_110, 0b011_001,
            0b011_010, 0b011_100, 0b100_011, 0b100_101, 0b100_110, 0b101_001, 0b101_010, 0b101_100,
            0b110_001, 0b110_010, 0b110_100, 0b111_000, //
            // 4-ways
            0b001_111, 0b010_111, 0b011_011, 0b011_101, 0b011_110, 0b100_111, 0b101_011, 0b101_101,
            0b101_110, 0b110_011, 0b110_101, 0b110_110, 0b111_001, 0b111_010, 0b111_100,
            // 5-Ways
            0b111_110, 0b111_101, 0b111_011, 0b110_111, 0b101_111, 0b011_111, //
            // 6-way
            0b111_111,
        ],
        0,
    );

    let mut nets_by_mask: HashMap<u32, Vec<Vec<u32>>> = HashMap::new();
    let mut mass_by_mask: HashMap<u32, Vec<u32>> = HashMap::new();

    let count = lookups::create_lookups(seq, &mut nets_by_mask, &mut mass_by_mask);
    println!("Generated {} lookups.", count);

    write_nets_to_file(nets_by_mask, "../lib/src/maze/generated/nets_large.rs");
    write_mass_to_file(mass_by_mask, "../lib/src/maze/generated/mass_large.rs");

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
            0b000_111, 0b111_000, // 3-ways
        ],
        0,
    );

    let mut nets_by_mask: HashMap<u32, Vec<Vec<u32>>> = HashMap::new();
    let mut mass_by_mask: HashMap<u32, Vec<u32>> = HashMap::new();

    lookups::create_lookups(seq, &mut nets_by_mask, &mut mass_by_mask);

    write_nets_to_file(nets_by_mask, "../lib/src/maze/generated/nets_small.rs");
    write_mass_to_file(mass_by_mask, "../lib/src/maze/generated/mass_small.rs");
}

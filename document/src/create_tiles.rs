use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::io::Write;

use itertools::Itertools;

use common::consts::{A1, A2, A3, B1, B2, B3, N1, N2, N3, WA};
use common::node_combinations::NodeCombinations;
use common::types::point::{Point, TILE_HEIGHT, TILE_WIDTH};
use lib::renders::generated::fragment::MAP_PATHS;
use lib::renders::svg::Svg;

use crate::empty::empty_fragments;
use crate::variants::list_of_variants;

pub fn create_tiles() {
    // Clean folder first
    for entry in fs::read_dir("./../typst/tiles/").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() {
            fs::remove_file(path).unwrap();
        }
    }

    let sequences = NodeCombinations::new(
        vec![
            // Skip points, they are no interesting to look at
            // 0b000_001, 0b000_010, 0b000_100, 0b001_000, 0b010_000, 0b100_000, // Points
            0b011_000, 0b110_000, 0b101_000, 0b100_001, 0b010_010, 0b001_100, // Lines
            0b000_011, 0b000_110, 0b000_101, // Almost lines, vertex to vertex
            0b010_001, 0b001_001, 0b001_010, 0b100_010, 0b010_100, 0b100_100, //
            // 3 way
            0b111_000, 0b000_111, //
            // Additional 3-ways
            0b010_101, 0b100_110, 0b001_011, //
            0b110_001, 0b101_001, 0b011_100, 0b101_100, 0b110_010, 0b011_010, //
            0b010_011, 0b010_110, 0b100_011, 0b100_101, 0b001_101, 0b001_110, //
            0b011_001, 0b101_010, 0b110_100, //
        ],
        0,
    );

    println!(
        "Generating All tiles code for {:#?} tiles",
        sequences.clone().count()
    );

    let mut hashes_written: HashSet<String> = HashSet::new();
    let mut index = 1;

    for nets in sequences {
        let len = nets.clone().len();
        // Only interested in 2 or more nets, e.g. tiles with 1 net are not interesting
        // as we can see those in another document
        if len < 2 {
            continue;
        }

        let mut all_variants:Vec<usize> = vec![];
        for net in nets.clone() {
            for variant in list_of_variants(net) {
                all_variants.push(variant);
            }
        }

        // Why is this done during run-time, it's not much faster than before
        let nets_variants = NodeCombinations::new(all_variants, len);

        for variant in nets_variants {
            // Check the variant is valid, e.g. a vertex isn't used by more than 1 net
            let hash = variant
                .clone()
                .iter()
                .filter(|&x| !empty_fragments(*x))
                .sorted()
                .map(|x| format!("{x}"))
                .join("-");

            if hashes_written.contains(&hash.clone()) {
                continue;
            }
            hashes_written.insert(hash.clone());

            if nets_are_valid(variant.clone()) {
                if save_nets_to_svg(variant, index) {
                    index += 1;
                }
            }
        }
    }
    println!("Set: {:#?}", hashes_written.len());
}

pub fn save_nets_to_svg(nets: Vec<usize>, index: usize) -> bool {
    let mut svg = Svg::new(3f64 * TILE_WIDTH, 3f64 * TILE_HEIGHT);

    let wall = MAP_PATHS.get(&WA).unwrap();
    svg.add_fragment(wall, Point { x: 1f64, y: 1f64 }, 0);

    let mut fragment_count = 0;
    for net in nets.clone() {
        // Skip Empty nets, some are drawn by their neighbours
        let path = MAP_PATHS.get(&net).unwrap();
        if path.len() == 0 {
            continue;
        }
        fragment_count += 1;
        svg.add_fragment(path, Point { x: 1f64, y: 1f64 }, 0);
    }

    if fragment_count == 0 {
        return false;
    }
    let content = svg.to_string(false);
    // Check if this content has been used before ... if so, don't save it
    let mut f =
        File::create(format!("./../typst/tiles/{index:04}.svg")).expect("File Creation failed");
    let _ = f.write_all(content.as_bytes());
    return true;
}

fn nets_are_valid(nets: Vec<usize>) -> bool {
    let mut v1 = 0;
    let mut v2 = 0;
    let mut v3 = 0;

    for net in nets.clone() {
        if net & A1 == A1 || net & B1 == B1 || net & N1 == N1 {
            v1 += 1;
        }
        if net & A2 == A2 || net & B2 == B2 || net & N2 == N2 {
            v2 += 1;
        }
        if net & A3 == A3 || net & B3 == B3 || net & N3 == N3 {
            v3 += 1;
        }
        if v1 > 1 || v2 > 1 || v3 > 1 {
            return false;
        }
    }
    return true;
}

// Add tests here
#[cfg(test)]
mod tests {
  use common::consts::B1;

  use super::*;

  #[test]
    fn test_valild_match() {
        assert_eq!(nets_are_valid(vec![A1 | B1, A2, A3, A1 | B1]), false);
        assert_eq!(nets_are_valid(vec![A1 | B1, A2, A3]), true);
    }
}

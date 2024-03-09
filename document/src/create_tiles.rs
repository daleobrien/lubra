use crate::variants::list_of_variants;
use common::bit_utils::to_bits;
use common::consts::WA;
use common::node_combinations::NodeCombinations;
use common::types::point::{Point, TILE_HEIGHT, TILE_WIDTH};
use lib::renders::fragment::MAP_PATHS;
use lib::renders::svg::Svg;
use std::fs;
use std::fs::File;
use std::io::Write;

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
            //0b000_001, 0b000_010, 0b000_100, 0b001_000, 0b010_000, 0b100_000, // Points
            0b011_000, 0b110_000, 0b101_000, 0b111_000, // Arcs
            //0b100_001, 0b010_010, 0b001_100, // Lines
            0b000_011, 0b000_110, 0b000_101, // Almost lines, vertex to vertex
            //0b000_111, // All vertexes connected
            0b010_001, 0b001_001, 0b001_010, 0b100_010, 0b010_100, 0b100_100,
        ],
        0,
    );

    println!(
        "Generating All tiles code for {:#?} tiles",
        sequences.clone().count()
    );
    let mut index = 0;
    for nets in sequences {
        let len = nets.clone().len();

        let mut all_variants = vec![];
        for net in nets.clone() {
            for variant in list_of_variants(net) {
                all_variants.push(variant);
            }
        }
        let nets_variants = NodeCombinations::new(all_variants, len);

        for variant in nets_variants {
            index += 1;
            save_nets_to_svg(variant, index);
        }
    }
}

pub fn save_nets_to_svg(nets: Vec<u32>, index: usize) {
    let mut svg = Svg::new(4f64 * TILE_WIDTH, 4f64 * TILE_HEIGHT);

    let wall = MAP_PATHS.get(&WA).unwrap();
    svg.add_fragment(wall, Point { x: 1f64, y: 1f64 }, 0);

    for net in nets {
        // TODO: Look for variants too! (expand the list of nets into a list of list of nets
        let path = MAP_PATHS.get(&net).unwrap();
        svg.add_fragment(path, Point { x: 1f64, y: 1f64 }, 0);
    }
    let content = svg.to_string();
    let mut f =
        File::create(format!("./../typst/tiles/{index:04}.svg")).expect("File Creation failed");
    let _ = f.write_all(content.as_bytes());
}

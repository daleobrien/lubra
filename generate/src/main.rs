use std::collections::HashMap;
use std::fs;
use std::time::Instant;

use itertools::Itertools;
use tera::{Context, Tera};

use lib::renders::constants::TileFlags;

/*
       1
[1]  4  5   [2]
   2  6  3
     [3]
*/

// This a way to represent tiles to help with rendering
// TODO: Rename, they are graph nodes
const POINT_GRAPH: [u32; 6] = [0b000001, 0b000010, 0b000100, 0b001000, 0b010000, 0b100000];
const LINE_GRAPH: [u32; 3] = [0b100001, 0b010010, 0b001100];
const CURVE_COMINATION_GRAPH: [u32; 5] = [0b000_000, 0b011_000, 0b110_000, 0b101_000, 0b111_000];

struct Combinations {
    tiles_by_index: HashMap<u32, Vec<TileFlags>>,
}

impl Combinations {
    pub fn new() -> Self {
        return Self {
            tiles_by_index: HashMap::new(),
        };
    }
    fn mask_from_list_of_masks(&self, list_of_mask: Vec<&u32>) -> u32 {
        let mask: u32 = list_of_mask.iter().fold(0b0, |mut sum, &x| {
            sum = sum | x;
            sum
        });
        return mask;
    }

    fn dont_care_to_trit (&self, bits:u32) -> u32 {
       // 0b101 -> 0b10_00_10 
       let mut trits: u32 = 0;
       for j in 0..6 {
           let b: u32 = 2 << (2 * j);
           if bits & (1<<j) == (1<<j) {
               trits |= b;
           }
       }
       return trits;
    }
    
    fn mask_to_trit (&self, bits:u32) -> u32 {
       // 0b101 -> 0b01_00_01 
        let mut trits: u32 = 0;
        for j in 0..6 {
            let b: u32 = 1 << (2 * j);
            if bits & (1<<j) == (1<<j) {
                trits |= b;
            }
        }
        return trits;
     }
     
    fn create_tile_subset_rust_files(&self) {
        //
        let mut side: HashMap<u32, Vec<TileFlags>> = HashMap::new();

        let mut side_key_as_binary: HashMap<u32, String> = HashMap::new();

        for n in 0..64 {
            
            let dont_care = self.dont_care_to_trit(n);
            //
            for (key, tiles) in self.tiles_by_index.iter() {
                let index = self.mask_to_trit(key & !n & 0b111_111) | dont_care;
               
                side.entry(index).or_insert_with(Vec::new).extend(tiles);
                let s = format!("{index:012b}").to_string();
                side_key_as_binary.entry(index).or_insert_with(||s);
            }
        }

        let tera = match Tera::new("src/templates/*.rs") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        let mut context = Context::new();

        context.insert("side", &side);
        //
        let mut side_keys: Vec<u32> = side.into_keys().collect();
        //
        side_keys.sort_unstable();

        context.insert("side_keys", &side_keys);
        context.insert("side_key_as_binary", &side_key_as_binary);

        let rendered = tera.render("tile_subsets.rs", &context);

        fs::write("../lib/src/renders/tile_subsets.rs", rendered.unwrap())
            .expect("Unable to write file");

        println!("Created 'lib/src/renders/tile_subsets.rs'");
    }

    fn build_all_posible_tiles(&mut self) {
        let mut count = 0;

        for point_graph_k in 0..=POINT_GRAPH.len() {
            for perm_point_graph in POINT_GRAPH.iter().combinations(point_graph_k).unique() {
                let point_mask = self.mask_from_list_of_masks(perm_point_graph);

                for point_graph_l in 0..=LINE_GRAPH.len() {
                    for perm_line_graph in LINE_GRAPH.iter().combinations(point_graph_l).unique() {
                        let line_mask = self.mask_from_list_of_masks(perm_line_graph);

                        // Check they if they intersect, if so, skip
                        if point_mask & line_mask != 0 {
                            continue;
                        }

                        for curve_mask in CURVE_COMINATION_GRAPH.iter() {
                            // Check they don't intersect
                            if curve_mask & point_mask != 0 || curve_mask & line_mask != 0 {
                                continue;
                            }

                            count += 1;
                            let key = point_mask | line_mask | curve_mask;

                            let value: TileFlags =
                                TileFlags::from([point_mask, line_mask, *curve_mask]);

                            println!("{:?}", value);
                            // Encode into better bit structure. Use bit enum!
                            self.tiles_by_index
                                .entry(key)
                                .or_insert_with(Vec::new)
                                .push(value);
                        }
                    }
                }
            }
        }

        println!("Found {count} unique tiles");
    }
}

fn main() {
    let now = Instant::now();

    let mut combinations = Combinations::new();
    combinations.build_all_posible_tiles();
    combinations.create_tile_subset_rust_files();

    let elapsed_time = now.elapsed();
    println!(
        "Running slow_function() took {} ms.",
        elapsed_time.as_millis()
    );
}

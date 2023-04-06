use std::collections::HashMap;
use std::fs;
use std::time::Instant;

use itertools::Itertools;
use tera::{Context, Tera};

// TODO: Ssould remove the Tile depend.
use lib::maze::tile::Tile;

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
const CURVE_COMBINATION_GRAPH: [u32; 5] = [0b000_000, 0b011_000, 0b110_000, 0b101_000, 0b111_000];

struct Combinations {
    tiles_by_index: HashMap<u32, Vec<Tile>>,
    tiles_by_value: HashMap<Tile, Vec<Vec<u32>>>, //
}

impl Combinations {
    pub fn new() -> Self {
        return Self {
            tiles_by_index: HashMap::new(),
            tiles_by_value: HashMap::new(),
        };
    }

    fn mask_from_list_of_masks(&self, list_of_mask: Vec<&u32>) -> u32 {
        let mask: u32 = list_of_mask.iter().fold(0b0, |mut sum, &x| {
            sum = sum | x;
            sum
        });
        return mask;
    }

    fn dont_care_to_trit(&self, bits: u32) -> u32 {
        // 0b101 -> 0b10_00_10
        let mut trits: u32 = 0;
        for j in 0..6 {
            let b: u32 = 2 << (2 * j);
            if bits & (1 << j) == (1 << j) {
                trits |= b;
            }
        }
        return trits;
    }

    fn mask_to_trit(&self, bits: u32) -> u32 {
        // 0b101 -> 0b01_00_01
        let mut trits: u32 = 0;
        for j in 0..6 {
            let b: u32 = 1 << (2 * j);
            if bits & (1 << j) == (1 << j) {
                trits |= b;
            }
        }
        return trits;
    }

    fn create_nets(&self, point_mask: u32, line_mask: u32, curve_mask: u32) -> Vec<Vec<u32>> {
        let mut nets: Vec<Vec<u32>> = Vec::new();

        // TODO: 0 based ?

        // Points
        for i in 0..6 {
            let mut net = Vec::new();
            if point_mask & (1 << i) == (1 << i) {
                net.push(i + 1);
                nets.push(net);
            }
        }

        // Lines, 2 lines never join together, rather they pass over each other
        if line_mask & 0b100001 == 0b100001 {
            nets.push(vec![1, 6]);
        }
        if line_mask & 0b010010 == 0b010010 {
            nets.push(vec![2, 5]);
        }
        if line_mask & 0b001100 == 0b001100 {
            nets.push(vec![3, 4]);
        }

        // Curves, sometimes we join 3 points together
        if curve_mask & 0b111000 == 0b111000 {
            nets.push(vec![4, 5, 6]);
        } else {
            if curve_mask & 0b011000 == 0b011000 {
                nets.push(vec![4, 5]);
            }
            if curve_mask & 0b110000 == 0b110000 {
                nets.push(vec![5, 6]);
            }
            if curve_mask & 0b101000 == 0b101000 {
                nets.push(vec![4, 6]);
            }
        }

        return nets;
    }

    fn create_tile_net_rust_file(&self) {
        let mut context = Context::new();
        let mut side_key_as_binary: HashMap<u32, String> = HashMap::new();

        let mut tiles: HashMap<Tile, Vec<Vec<u32>>> = HashMap::new();
        for (tile, nets) in self.tiles_by_value.iter() {
            tiles.insert(*tile, nets.clone());
            // 0b000000000000
            let v = tile.bits();
            let s = format!("{v:014b}").to_string();
            side_key_as_binary.entry(tile.bits()).or_insert_with(|| s);
        }

        let mut keys = self.tiles_by_value.keys().collect::<Vec<&Tile>>();
        keys.sort_unstable();
        // for key in keys.iter() {
        //     print!("{:?} {:?}\n", key.bits(), tiles[key])
        // }
        context.insert("tiles", &tiles);
        context.insert("keys", &keys);
        context.insert("side_key_as_binary", &side_key_as_binary);

        let template =
            fs::read_to_string("src/templates/tile_net.rs").expect("Unable to read file");
        let rendered = Tera::one_off(template.as_str(), &context, true);

        fs::write("../lib/src/maze/net.rs", rendered.unwrap()).expect("Unable to write file");
    }

    fn create_lookup_rust_files(&self) {
        //
        let mut side: HashMap<u32, Vec<Tile>> = HashMap::new();

        let mut side_key_as_binary: HashMap<u32, String> = HashMap::new();

        for n in 0..64 {
            let dont_care = self.dont_care_to_trit(n);
            //
            for (key, tiles) in self.tiles_by_index.iter() {
                let index = self.mask_to_trit(key & !n & 0b111_111) | dont_care;
                let mut sorted_tiles = tiles.clone();
                sorted_tiles.sort_unstable();
                side.entry(index)
                    .or_insert_with(Vec::new)
                    .extend(sorted_tiles);
                let s = format!("{index:012b}").to_string();
                side_key_as_binary.entry(index).or_insert_with(|| s);
            }
        }

        let mut context = Context::new();

        // Sort the Values
        for (_k, v) in side.iter_mut() {
            v.sort_unstable();
        }

        context.insert("side", &side);
        //
        let mut side_keys: Vec<u32> = side.into_keys().collect();
        //
        side_keys.sort_unstable();

        context.insert("side_keys", &side_keys);
        context.insert("side_key_as_binary", &side_key_as_binary);

        let template =
            fs::read_to_string("src/templates/tile_lookup.rs").expect("Unable to read file");
        let rendered = Tera::one_off(template.as_str(), &context, true);

        fs::write("../lib/src/renders/lookup.rs", rendered.unwrap()).expect("Unable to write file");
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

                        for curve_mask in CURVE_COMBINATION_GRAPH.iter() {
                            // Check they don't intersect
                            if curve_mask & point_mask != 0 || curve_mask & line_mask != 0 {
                                continue;
                            }

                            count += 1;
                            let key = point_mask | line_mask | curve_mask;

                            let value: Tile = Tile::from([point_mask, line_mask, *curve_mask]);

                            let mut factor = 1;

                            if curve_mask.count_ones() != 0 {
                                factor *= curve_mask.count_ones() * 5;
                            }

                            if line_mask.count_ones() != 0 {
                                factor *= line_mask.count_ones() * 4;
                            }

                            let nets = self.create_nets(point_mask, line_mask, *curve_mask);
                            for net in nets {
                                self.tiles_by_value
                                    .entry(value)
                                    .or_insert_with(Vec::new)
                                    .push(net);
                            }

                            // TODO: Push some items more that once so they have better odds of being selected
                            for _ in 0..factor {
                                self.tiles_by_index
                                    .entry(key)
                                    .or_insert_with(Vec::new)
                                    .push(value);
                            }
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
    combinations.create_lookup_rust_files();
    combinations.create_tile_net_rust_file();

    let elapsed_time = now.elapsed();
    println!(
        "Running slow_function() took {} ms.",
        elapsed_time.as_millis()
    );
}

use std::collections::HashMap;
use std::fs;
use std::time::Instant;

use itertools::Itertools;
use tera::{Context, Tera};

/*
       1
[1]  4  5   [2]
   2  6  3
     [3]
*/

const POINTS: [u32; 6] = [0b000001, 0b000010, 0b000100, 0b001000, 0b010000, 0b100000];
const LINES: [u32; 3] = [0b100001, 0b010010, 0b001100];
const CURVES_COMINATIONS: [u32; 5] = [0b000000, 0b011000, 0b110000, 0b101000, 0b111000];

struct Combinations {
    tiles_by_index: HashMap<u32, Vec<u32>>,
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

    fn create_tile_subset_rust_files(&self) {
        //
        let mut side_1: HashMap<u32, Vec<u32>> = HashMap::new();
        let mut side_2: HashMap<u32, Vec<u32>> = HashMap::new();
        let mut side_3: HashMap<u32, Vec<u32>> = HashMap::new();

        let mut side_1_2: HashMap<u32, Vec<u32>> = HashMap::new();
        let mut side_2_3: HashMap<u32, Vec<u32>> = HashMap::new();
        let mut side_3_1: HashMap<u32, Vec<u32>> = HashMap::new();

        let mut side_1_2_3: HashMap<u32, Vec<u32>> = HashMap::new();

        //
        for (key, tiles) in self.tiles_by_index.iter().sorted_by_key(|x| x.0) {
            side_1
                .entry(*key & 0b001_011)
                .or_insert_with(Vec::new)
                .extend(tiles); // 142
            side_2
                .entry(*key & 0b010_101)
                .or_insert_with(Vec::new)
                .extend(tiles); // 153
            side_3
                .entry(*key & 0b100_110)
                .or_insert_with(Vec::new)
                .extend(tiles); // 236
            side_1_2
                .entry(*key & 0b011_111)
                .or_insert_with(Vec::new)
                .extend(tiles); // 12345
            side_2_3
                .entry(*key & 0b110_111)
                .or_insert_with(Vec::new)
                .extend(tiles); // 123 5
            side_3_1
                .entry(*key & 0b101_111)
                .or_insert_with(Vec::new)
                .extend(tiles); // 1234 6
            side_1_2_3
                .entry(*key & 0b111_111)
                .or_insert_with(Vec::new)
                .extend(tiles);
        }

        let tera = match Tera::new("src/templates/*.rs") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        let mut context = Context::new();

        context.insert("side_1", &side_1);
        context.insert("side_2", &side_2);
        context.insert("side_3", &side_3);
        context.insert("side_1_2", &side_1_2);
        context.insert("side_2_3", &side_2_3);
        context.insert("side_3_1", &side_3_1);
        context.insert("side_1_2_3", &side_1_2_3);
        //
        let mut side_1_keys: Vec<u32> = side_1.into_keys().collect();
        let mut side_2_keys: Vec<u32> = side_2.into_keys().collect();
        let mut side_3_keys: Vec<u32> = side_3.into_keys().collect();
        let mut side_1_2_keys: Vec<u32> = side_1_2.into_keys().collect();
        let mut side_2_3_keys: Vec<u32> = side_2_3.into_keys().collect();
        let mut side_3_1_keys: Vec<u32> = side_3_1.into_keys().collect();
        let mut side_1_2_3_keys: Vec<u32> = side_1_2_3.into_keys().collect();
        //
        side_1_keys.sort_unstable();
        side_2_keys.sort_unstable();
        side_3_keys.sort_unstable();
        side_1_2_keys.sort_unstable();
        side_2_3_keys.sort_unstable();
        side_3_1_keys.sort_unstable();
        side_1_2_3_keys.sort_unstable();

        context.insert("side_1_keys", &side_1_keys);
        context.insert("side_2_keys", &side_2_keys);
        context.insert("side_3_keys", &side_3_keys);
        context.insert("side_1_2_keys", &side_1_2_keys);
        context.insert("side_2_3_keys", &side_2_3_keys);
        context.insert("side_3_1_keys", &side_3_1_keys);
        context.insert("side_1_2_3_keys", &side_1_2_3_keys);

        let rendered = tera.render("tile_subsets.rs", &context);

        fs::write("../lib/src/renders/tile_subsets.rs", rendered.unwrap())
            .expect("Unable to write file");

        println!("Created 'lib/src/renders/tile_subsets.rs'");
    }

    fn build_all_posible_tiles(&mut self) {
        let mut count = 0;

        for points_k in 0..=POINTS.len() {
            for perm_points in POINTS.iter().combinations(points_k).unique() {
                let point_mask = self.mask_from_list_of_masks(perm_points);

                for points_l in 0..=LINES.len() {
                    for perm_lines in LINES.iter().combinations(points_l).unique() {
                        let line_mask = self.mask_from_list_of_masks(perm_lines);

                        // Check they if they intersect, if so, skip
                        if point_mask & line_mask != 0 {
                            continue;
                        }

                        for curve_mask in CURVES_COMINATIONS.iter() {
                            // Check they don't intersect
                            if curve_mask & point_mask != 0 || curve_mask & line_mask != 0 {
                                continue;
                            }

                            count += 1;
                            let key = point_mask | line_mask | curve_mask;
                            self.tiles_by_index
                                .entry(key)
                                .or_insert_with(Vec::new)
                                .push(point_mask << 16 | line_mask << 8 | *curve_mask);
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

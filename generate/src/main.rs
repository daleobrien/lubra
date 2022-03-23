use std::collections::HashMap;

use itertools::Itertools;

/*
       1
[1]  4  5   [2]
   2  6  3
     [3]



   I don't really need enum for tile, it can be a bit field
*/

fn mask_from_list_of_masks(list_of_mask: Vec<&u8>) -> u8 {
    let mask: u8 = list_of_mask.iter().fold(0b0, |mut sum, &x| {
        sum = sum | x;
        sum
    });
    return mask;
}

const POINTS: [u8;6] = [0b000001, 0b000010, 0b000100, 0b001000, 0b010000, 0b100000];
const LINES:[u8;3] = [0b100001, 0b010010, 0b001100];
const CURVES: [u8;5] = [0b000000, 0b011000, 0b110000, 0b101000, 0b111000]; // All combinations

fn main() {
    
 
    let mut tiles_by_index: HashMap<u8, Vec<[u8; 3]>> = HashMap::new();

    let mut count = 0;

    for points_k in 0..=POINTS.len() {
        for perm_points in POINTS.iter().combinations(points_k).unique() {
            let point_mask = mask_from_list_of_masks(perm_points);

            for points_l in 0..=LINES.len() {
                for perm_lines in LINES.iter().combinations(points_l).unique() {
                    let line_mask = mask_from_list_of_masks(perm_lines);

                    // Check they don't intersect
                    if point_mask & line_mask != 0 {
                        continue;
                    }

                    for curve_mask in CURVES.iter() {
                        // Check they don't intersect
                        if point_mask & curve_mask != 0
                            || point_mask & line_mask != 0
                            || curve_mask & line_mask != 0
                        {
                            continue;
                        }
                        // Skip if last collection of curves had 2 or more curves, since it's a
                        // fully connected, in really the same way.

                        count += 1;
                        // println!(
                        //     "{count:#3}: points:{:#08b} lines:{:#08b} curves:{:#08b}",
                        //     point_mask, line_mask, curve_mask
                        // );
                        let key = point_mask | line_mask | curve_mask;
                        tiles_by_index.entry(key).or_insert_with(Vec::new).push([
                            point_mask,
                            line_mask,
                            *curve_mask,
                        ]);
                    }
                }
            }
        }
    }

    println!("Found {count} unique tiles");
    // Move around
    let mut side_1: HashMap<u8, Vec<[u8; 3]>> = HashMap::new();
    let mut side_2: HashMap<u8, Vec<[u8; 3]>> = HashMap::new();
    let mut side_3: HashMap<u8, Vec<[u8; 3]>> = HashMap::new();

    let mut side_1_2: HashMap<u8, Vec<[u8; 3]>> = HashMap::new();
    let mut side_2_3: HashMap<u8, Vec<[u8; 3]>> = HashMap::new();
    let mut side_3_1: HashMap<u8, Vec<[u8; 3]>> = HashMap::new();

    let mut side_1_2_3: HashMap<u8, Vec<[u8; 3]>> = HashMap::new();

    //
    for (key, tiles) in tiles_by_index.iter().sorted_by_key(|x| x.0) {
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

    for (key, tiles) in side_1_2_3.iter().sorted_by_key(|x| x.0) {
        println!("{key:#08b} {:?}", tiles);
    }
}

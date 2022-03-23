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

fn main() {
    let points: Vec<u8> = vec![0b000001, 0b000010, 0b000100, 0b001000, 0b010000, 0b100000];
    let lines: Vec<u8> = vec![0b100001, 0b010010, 0b001100];
    let curves: Vec<u8> = vec![0b000000, 0b011000, 0b110000, 0b101000, 0b111000]; // All combinations

    // The key is the binary represention of the vertex/edges that have connections.
    // For a given edge, we can then find compatible tiles
    let mut tiles_by_index: HashMap<u8, Vec<[u8; 3]>> = HashMap::new();

    let mut count = 0;

    for points_k in 0..=points.len() {
        for perm_points in points.iter().combinations(points_k).unique() {
            let point_mask = mask_from_list_of_masks(perm_points);

            for points_l in 0..=lines.len() {
                for perm_lines in lines.iter().combinations(points_l).unique() {
                    let line_mask = mask_from_list_of_masks(perm_lines);

                    // Check they don't intersect
                    if point_mask & line_mask != 0 {
                        continue;
                    }

                    for curve_mask in curves.iter() {
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
    //
    let mut side_1: HashMap<u8, Vec<[u8; 3]>> = HashMap::new();
    let mut side_2: HashMap<u8, Vec<[u8; 3]>> = HashMap::new();
    let mut side_3: HashMap<u8, Vec<[u8; 3]>> = HashMap::new();

    let mut side_1_2: HashMap<u8, Vec<[u8; 3]>> = HashMap::new();
    let mut side_2_3: HashMap<u8, Vec<[u8; 3]>> = HashMap::new();
    let mut side_3_1: HashMap<u8, Vec<[u8; 3]>> = HashMap::new();

    let mut side_1_2_3: HashMap<u8, Vec<[u8; 3]>> = HashMap::new();

    //
    for (key, tiles) in tiles_by_index.iter().sorted_by_key(|x| x.0) {
        let side_2_index = *key & 0b010101; // 153
        let side_3_index = *key & 0b100110; // 236

        let side_1_2_index = *key & 0b011111; // 12345
        let side_2_3_index = *key & 0b110111; // 123 5
        let side_3_1_index = *key & 0b101111; // 1234 6

        let side_1_2_3_index = *key & 0b111111; // 123456

        // 142
        side_1
            .entry(*key & 0b001011)
            .or_insert_with(Vec::new)
            .extend(tiles); // 142
        side_1
            .entry(*key & 0b010101)
            .or_insert_with(Vec::new)
            .extend(tiles); // 153
        side_3
            .entry(*key & 0b100110)
            .or_insert_with(Vec::new)
            .extend(tiles);

        side_1_2
            .entry(side_1_2_index)
            .or_insert_with(Vec::new)
            .extend(tiles);
        side_2_3
            .entry(side_2_3_index)
            .or_insert_with(Vec::new)
            .extend(tiles);
        side_3_1
            .entry(side_3_1_index)
            .or_insert_with(Vec::new)
            .extend(tiles);

        side_1_2_3
            .entry(side_1_2_3_index)
            .or_insert_with(Vec::new)
            .extend(tiles);
    }

    for (key, tiles) in side_1_2_3.iter().sorted_by_key(|x| x.0) {
        println!("{key:#08b} {:?}", tiles);
    }
}

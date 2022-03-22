use std::collections::HashMap;

use array_tool::vec::{Intersect, Union};
use itertools::Itertools;

use lib::renders::constants::TileType;

fn main() {
    let points: Vec<Vec<u8>> = vec![vec![1], vec![2], vec![3], vec![4], vec![5], vec![6]];
    let lines: Vec<Vec<u8>> = vec![vec![1, 6], vec![2, 5], vec![3, 4]];
    let curves: Vec<Vec<u8>> = vec![vec![4, 5], vec![5, 6], vec![6, 4]];

    // The key is the binary represention of the vertex/edges that have connections.
    // For a given edge, we can then find compatible tiles
    let mut tiles_by_index: HashMap<u8, Vec<Vec<TileType>>> = HashMap::new();

    let mut count = 0;

    for points_k in 0..=points.len() {
        for perm_points in points.iter().combinations(points_k).unique() {
            let flat_points = perm_points
                .into_iter()
                .flat_map(|s| s.clone())
                .collect::<Vec<u8>>();

            for points_l in 0..=lines.len() {
                for perm_lines in lines.iter().combinations(points_l).unique() {
                    // Check they don't intersect
                    let flat_lines = perm_lines
                        .into_iter()
                        .flat_map(|s| s.clone())
                        .collect::<Vec<u8>>();

                    if flat_points.intersect(flat_lines.clone()).len() > 0 {
                        continue;
                    }

                    let mut last_perm_curves_len = 0;
                    for curves_l in 0..curves.len() {
                        for perm_curves in curves.iter().combinations(curves_l).unique() {
                            // Check they don't intersect
                            let flat_curves = perm_curves
                                .into_iter()
                                .flat_map(|s| s.clone())
                                .collect::<Vec<u8>>();

                            if flat_points.intersect(flat_curves.clone()).len() > 0 {
                                continue;
                            }

                            if flat_lines.intersect(flat_curves.clone()).len() > 0 {
                                continue;
                            }
                            // Skip if last collection of curves had 2 or more curves, since it's a
                            // fully connected, in really the same way.
                            let perm_curves_len = flat_curves.len();
                            if perm_curves_len >= 4 && last_perm_curves_len >= 4 {
                                continue;
                            }

                            // Render
                            count += 1;
                            let mut primatives_in_tile: Vec<TileType> = vec![];
                            if flat_points.len() > 0 {
                                for x in flat_points.clone() {
                                    if [1, 2, 3].contains(&x) {
                                        match x {
                                            1 => primatives_in_tile.push(TileType::N1),
                                            2 => primatives_in_tile.push(TileType::N2),
                                            3 => primatives_in_tile.push(TileType::N3),
                                            _ => {
                                                print!("ERROR N{x}")
                                            }
                                        }
                                    } else {
                                        match x {
                                            4 => primatives_in_tile.push(TileType::T1),
                                            5 => primatives_in_tile.push(TileType::T2),
                                            6 => primatives_in_tile.push(TileType::T3),
                                            _ => {
                                                print!("ERROR T{x}")
                                            }
                                        }
                                    }
                                }
                            }
                            if flat_lines.len() > 0 {
                                if flat_lines.len() > 4 {
                                    primatives_in_tile.push(TileType::X2)
                                } else if flat_lines.len() > 2 {
                                    match (flat_lines[0], flat_lines[2]) {
                                        (1, 2) => primatives_in_tile.push(TileType::Y3),
                                        (1, 3) => primatives_in_tile.push(TileType::Y2),
                                        (2, 3) => primatives_in_tile.push(TileType::Y1),
                                        _ => {
                                            print!("ERROR Yx")
                                        }
                                    }
                                } else {
                                    match flat_lines[0] {
                                        1 => primatives_in_tile.push(TileType::L1),
                                        2 => primatives_in_tile.push(TileType::L2),
                                        3 => primatives_in_tile.push(TileType::L3),
                                        _ => {
                                            print!("ERROR Lx")
                                        }
                                    }
                                }
                            }
                            if flat_curves.len() > 0 {
                                if flat_curves.len() > 2 {
                                    primatives_in_tile.push(TileType::X1)
                                } else {
                                    match (flat_curves[0], flat_curves[1]) {
                                        (4, 5) => primatives_in_tile.push(TileType::A1),
                                        (5, 6) => primatives_in_tile.push(TileType::A3),
                                        (6, 4) => primatives_in_tile.push(TileType::A2),
                                        _ => {
                                            print!("ERROR Ax")
                                        }
                                    }
                                }
                            }
                            last_perm_curves_len = perm_curves_len;

                            // Convert the edge/vertex numbers to a binary numbr
                            let node_numbers = flat_points
                                .union(flat_curves.clone())
                                .union(flat_lines.clone());

                            let node_number: u8 =
                                node_numbers.iter().map(|&val| 1 << (val - 1)).sum();

                            // Should I pad out the vec?, or should I do the same in my other code,
                            // use Vec instead of fixed length Array
                            tiles_by_index
                                .entry(node_number)
                                .or_insert_with(Vec::new)
                                .push(primatives_in_tile);
                        }
                    }
                }
            }
        }
    }
    println!("Found {count} unique tiles");

    for (key, value) in tiles_by_index.iter().sorted_by_key(|x| x.0) {
        println!("{} <{}>: {:?}", key, value.len(), value);
        // Or do we use a match, for and mask off the bits, I think maybe that's the way
        // Can be an array at here, it's a known length
        let lowest_key = *key & 7u8;
        // first_side[lowest_key].push ... like above

        // And so on
    }
}

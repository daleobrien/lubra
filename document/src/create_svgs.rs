use std::fs;
use std::fs::File;
use std::io::Write;

use itertools::Itertools;

use common::consts::{
    A1, A2, A3, A4, A5, A6, B1, B2, B3, B4, B5, B6, IN, N1, N2, N3, N4, N5, N6, WA,
};
use common::types::point::{Point, TILE_HEIGHT, TILE_WIDTH};
use lib::renders::generated::fragment::MAP_PATHS;
use lib::renders::svg::Svg;

use crate::{add_nodes, draw_walls};

pub fn create_svgs() {
    // Clean folder first
    for entry in fs::read_dir("./../typst/svgs/").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() {
            fs::remove_file(path).unwrap();
        }
    }

    for (&key, value) in MAP_PATHS
        .into_iter()
        .sorted_by_key(|x| x.0.reverse_bits())
        .rev()
    {
        if value.is_empty() {
            continue;
        }

        if key & WA != 0usize {
            continue;
        }

        // Don't draw tiles with only 1 node
        // let lower = key & (N1 | N2 | N3 | N4 | N5 | N6);
        // if lower == N1 || lower == N2 || lower == N3 || lower == N4 || lower == N5 || lower == N6 {
        //     continue;
        // }

        let mut svg = Svg::new(4f64 * TILE_WIDTH, 4f64 * TILE_HEIGHT);

        svg.add_fragment(value, Point { x: 2f64, y: 1f64 }, 0);

        draw_walls::add_walls(key, &mut svg);
        add_nodes::add_indexes(key, &mut svg);

        let content = svg.to_string(false);

        let mut file_name_part_1: Vec<String> = vec![];
        let mut file_name_part_2: Vec<String> = vec![];
        let mut file_name_part_3: Vec<String> = vec![];
        let keys_to_check = [
            (N1, "1"),
            (N2, "2"),
            (N3, "3"),
            (N4, "4"),
            (N5, "5"),
            (N6, "6"),
        ];
        for &(key_value, name) in &keys_to_check {
            if key & key_value == key_value {
                file_name_part_1.push(name.to_string());
            }
        }
        let keys_to_check = [
            (A1, "1"),
            (A2, "2"),
            (A3, "3"),
            (A4, "4"),
            (A5, "5"),
            (A6, "6"),
        ];
        for &(key_value, name) in &keys_to_check {
            if key & key_value == key_value {
                file_name_part_2.push(name.to_string());
            }
        }
        let keys_to_check = [
            (B1, "1"),
            (B2, "2"),
            (B3, "3"),
            (B4, "4"),
            (B5, "5"),
            (B6, "6"),
        ];
        for &(key_value, name) in &keys_to_check {
            if key & key_value == key_value {
                file_name_part_3.push(name.to_string());
            }
        }

        let file_name_part_1 = file_name_part_1.join("");
        let file_name_part_2 = file_name_part_2.join("");
        let file_name_part_3 = file_name_part_3.join("");

        let mut filename: Vec<String> = vec![];
        if !file_name_part_1.is_empty() {
            let prefix = if key & IN == IN { "i" } else { "n" };
            filename.push(format!("{prefix}{file_name_part_1}"));
        }
        if !file_name_part_2.is_empty() {
            filename.push(format!("a{file_name_part_2}"));
        }
        if !file_name_part_3.is_empty() {
            filename.push(format!("b{file_name_part_3}"));
        }

        let filename = filename.join("-");

        let mut f =
            File::create(format!("./../typst/svgs/{filename}.svg")).expect("File Creation failed");
        let _ = f.write_all(content.as_bytes());
    }
}

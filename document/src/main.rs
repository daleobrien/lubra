use common::consts::{A1, A2, A3, A4, A5, A6, INVERT, N1, N2, N3, N4, N5, N6, WALL};
use common::types::point::{Point, TILE_HEIGHT, TILE_WIDTH};
use lib::renders::fragment::MAP_PATHS;
use lib::renders::svg::Svg;
use std::fs;
use std::fs::File;
use std::io::Write;

fn main() {
    println!("Loading fragments and generating, SVG");

    // Clean folder first
    for entry in fs::read_dir("./svgs").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() {
            fs::remove_file(path).unwrap();
        }
    }

    for (i, (&key, value)) in MAP_PATHS.into_iter().enumerate() {
        if key & WALL != 0u32 {
            continue;
        }

        let wall_key = (INVERT & key) | WALL;
        let wall = MAP_PATHS.get(&wall_key).unwrap();

        let mut svg = Svg::new(3f64 * TILE_WIDTH, 4f64 * TILE_HEIGHT);
        svg.add_fragment(
            value,
            Point {
                x: 0f64, //TILE_WIDTH / 2f64,
                y: 0f64, //TILE_HEIGHT / 2f64,
            },
            0,
        );
        svg.add_fragment(
            wall,
            Point {
                x: 0f64, //TILE_WIDTH / 2f64,
                y: 0f64, //TILE_HEIGHT / 2f64,
            },
            0,
        );
        let content = svg.to_string();

        let mut file_name_part_1: Vec<String> = vec![];
        let mut file_name_part_2: Vec<String> = vec![];
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
        let file_name_part_1 = file_name_part_1.join("");
        let file_name_part_2 = file_name_part_2.join("");

        let mut filename: Vec<String> = vec![];
        if key & INVERT == INVERT {
            filename.push("u".to_string());
        } else {
            filename.push("d".to_string());
        }

        if !file_name_part_1.is_empty() {
            filename.push(format!("n{file_name_part_1}"));
        }
        if !file_name_part_2.is_empty() {
            filename.push(format!("a{file_name_part_2}"));
        }

        let filename = filename.join("-");
        let mut f = File::create(format!("./svgs/{filename}.svg")).expect("File Creation failed");
        let _ = f.write_all(content.as_bytes());
    }
}

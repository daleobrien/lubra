use common::consts::{A1, A2, A3, A4, A5, A6, INVERT, N1, N2, N3, N4, N5, N6, WALL};
use common::types::point::{Point, TILE_HEIGHT, TILE_WIDTH};
use itertools::Itertools;
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

    let mut i = 0;
    for (&key, value) in MAP_PATHS.into_iter().sorted_by_key(|x| x.0.reverse_bits()) {
        if key & WALL != 0u32 {
            continue;
        }
        i += 1u32;

        let mut svg = Svg::new(4f64 * TILE_WIDTH, 4f64 * TILE_HEIGHT);

        svg.add_fragment(value, Point { x: 2f64, y: 1f64 }, 0);

        add_walls(key, &mut svg);
        add_indexes(key, &mut svg);

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
        if !file_name_part_1.is_empty() {
            let prefix = if key & INVERT == INVERT { "i" } else { "n" };
            filename.push(format!("{prefix}{file_name_part_1}"));
        }
        if !file_name_part_2.is_empty() {
            filename.push(format!("a{file_name_part_2}"));
        }

        let filename = filename.join("-");
        let mut f =
            File::create(format!("./svgs/{i:03}-{filename}.svg")).expect("File Creation failed");
        let _ = f.write_all(content.as_bytes());
    }
}

fn add_walls(key: u32, svg: &mut Svg) {
    let wall_key = (INVERT & key) | WALL;
    let wall = MAP_PATHS.get(&wall_key).unwrap();

    // Draw this a few times to highlight the walls of the centre tile.
    svg.add_fragment(wall, Point { x: 2f64, y: 1f64 }, 0);
    svg.add_fragment(wall, Point { x: 2f64, y: 1f64 }, 0);
    svg.add_fragment(wall, Point { x: 2f64, y: 1f64 }, 0);

    svg.add_fragment(wall, Point { x: 4f64, y: 1f64 }, 0);
    svg.add_fragment(wall, Point { x: 0f64, y: 1f64 }, 0);
    svg.add_fragment(wall, Point { x: 1f64, y: 2f64 }, 0);
    svg.add_fragment(wall, Point { x: 1f64, y: 0f64 }, 0);
    svg.add_fragment(wall, Point { x: 3f64, y: 2f64 }, 0);
    svg.add_fragment(wall, Point { x: 3f64, y: 0f64 }, 0);

    let invert = key & INVERT == INVERT;
    let y1 = 0f64;
    let y1 = if invert { 1f64 * TILE_HEIGHT - y1 } else { y1 };

    svg.add_fragment(
        &format!(
            "<path stroke-opacity=\"0.1\" stroke=\"red\" d=\"M{},{}l{},{}\"/>",
            0,
            y1,
            TILE_WIDTH / 2f64,
            if invert { TILE_HEIGHT } else { -TILE_HEIGHT }
        ),
        Point { x: 0f64, y: 1f64 },
        0,
    );
    svg.add_fragment(
        &format!(
            "<path stroke-opacity=\"0.1\" stroke=\"red\" d=\"M{},{}l{},{}\"/>",
            0,
            y1,
            -TILE_WIDTH / 2f64,
            if invert { TILE_HEIGHT } else { -TILE_HEIGHT }
        ),
        Point { x: 6f64, y: 1f64 },
        0,
    );

    svg.add_fragment(
        &format!(
            "<path stroke-opacity=\"0.1\" stroke=\"red\" d=\"M{},{}l{},{}\"/>",
            0, TILE_HEIGHT, TILE_WIDTH, 0
        ),
        Point {
            x: 2f64,
            y: if invert { -1f64 } else { 2f64 },
        },
        0,
    );
}

fn add_indexes(key: u32, svg: &mut Svg) {
    add_text(svg, "③".to_string(), 1f64, 0f64, key);
    add_text(svg, "⑤".to_string(), 2f64, 0f64, key);
    add_text(svg, "①".to_string(), 3f64, 0f64, key);
    add_text(svg, "④".to_string(), 4f64, 0f64, key);
    add_text(svg, "②".to_string(), 5f64, 0f64, key);

    add_text(svg, "⑤".to_string(), 0.5f64, 0.5f64, key);
    add_text(svg, "⑥".to_string(), 1.5f64, 0.5f64, key);
    add_text(svg, "④".to_string(), 2.5f64, 0.5f64, key);
    add_text(svg, "⑤".to_string(), 3.5f64, 0.5f64, key);
    add_text(svg, "⑥".to_string(), 4.5f64, 0.5f64, key);
    add_text(svg, "④".to_string(), 5.5f64, 0.5f64, key);

    add_text(svg, "①".to_string(), 0f64, 1f64, key);
    add_text(svg, "④".to_string(), 1f64, 1f64, key);
    add_text(svg, "②".to_string(), 2f64, 1f64, key);
    add_text(svg, "⑥".to_string(), 3f64, 1f64, key);
    add_text(svg, "③".to_string(), 4f64, 1f64, key);
    add_text(svg, "⑤".to_string(), 5f64, 1f64, key);
    add_text(svg, "①".to_string(), 6f64, 1f64, key);

    add_text(svg, "⑤".to_string(), 0.5f64, 1.5f64, key);
    add_text(svg, "⑥".to_string(), 1.5f64, 1.5f64, key);
    add_text(svg, "④".to_string(), 2.5f64, 1.5f64, key);
    add_text(svg, "⑤".to_string(), 3.5f64, 1.5f64, key);
    add_text(svg, "⑥".to_string(), 4.5f64, 1.5f64, key);
    add_text(svg, "④".to_string(), 5.5f64, 1.5f64, key);

    add_text(svg, "③".to_string(), 1f64, 2f64, key);
    add_text(svg, "⑤".to_string(), 2f64, 2f64, key);
    add_text(svg, "①".to_string(), 3f64, 2f64, key);
    add_text(svg, "④".to_string(), 4f64, 2f64, key);
    add_text(svg, "②".to_string(), 5f64, 2f64, key);

    add_text(svg, "⑥".to_string(), 1.5f64, 2.5f64, key);
    add_text(svg, "④".to_string(), 2.5f64, 2.5f64, key);
    add_text(svg, "⑤".to_string(), 3.5f64, 2.5f64, key);
    add_text(svg, "⑥".to_string(), 4.5f64, 2.5f64, key);

    add_text(svg, "②".to_string(), 2f64, 3f64, key);
    add_text(svg, "⑥".to_string(), 3f64, 3f64, key);
    add_text(svg, "③".to_string(), 4f64, 3f64, key);
}

fn add_text(svg: &mut Svg, n: String, x: f64, y: f64, key: u32) {
    let invert = key & INVERT == INVERT;
    let y = if invert { 3f64 - y } else { y };
    svg.add_fragment(
        &format!("<text font-size=\"1.7em\" fill=\"black\" x=\"-8\" y=\"5\" transform=\"scale(1, -1)\">{n}</text>"),
        Point { x, y },
        0,
    );
}

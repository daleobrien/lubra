use common::consts::{INVERT, WALL};
use common::types::point::{Point, TILE_HEIGHT, TILE_WIDTH};
use lib::renders::fragment::MAP_PATHS;
use lib::renders::svg::Svg;
use std::fs::File;
use std::io::Write;

fn main() {
    println!("Loading fragments and generating, SVG");
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
        let mut f = File::create(format!("./svgs/{:018b}.svg", key)).expect("File Creation failed");
        let _ = f.write_all(content.as_bytes());
    }
}

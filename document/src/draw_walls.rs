use common::consts::{IN, WA};
use common::types::point::{Point, TILE_HEIGHT, TILE_WIDTH};
use lib::renders::fragment::MAP_PATHS;
use lib::renders::svg::Svg;

pub fn add_walls(key: u32, svg: &mut Svg) {
    let invert = key & IN == IN;
    let wall_key = (IN & key) | WA;
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
            "<path stroke-opacity='0.1' stroke='red' d='M{},{}l{},{}'/>",
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
            "<path stroke-opacity='0.1' stroke='red' d='M{},{}l{},{}'/>",
            0, TILE_HEIGHT, TILE_WIDTH, 0
        ),
        Point {
            x: 2f64,
            y: if invert { -1f64 } else { 2f64 },
        },
        0,
    );
}

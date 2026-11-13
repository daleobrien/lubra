use crate::maze::tile::Tile;
use crate::renders::net_colours::net_colour;
use crate::renders::svg::Svg;
use array2d::Array2D;
use common::types::point::{Point, S2, TILE_HEIGHT, TILE_WIDTH};

const NUMBERS_0: [&str; 6] = ["①", "②", "③", "④", "⑤", "⑥"];
const NUMBERS_1: [&str; 6] = ["③", "①", "②", "⑤", "⑥", "④"];
const NUMBERS_2: [&str; 6] = ["②", "③", "①", "⑥", "④", "⑤"];
const NUMBERS: [[&str; 6]; 3] = [NUMBERS_0, NUMBERS_1, NUMBERS_2];
const POSITION_MARGIN_SQUARED: f64 = 4.0;

// Define node positions for a hexagonal tile (in local tile coordinates)
// These positions are based on a regular hexagon geometry
const NODE_POSITIONS: [(f64, f64); 6] = [
    (TILE_WIDTH / 2.0, TILE_HEIGHT),             // 1
    (0.0, 0.0),                                  // 2
    (TILE_WIDTH, 0.0),                           // 3
    (TILE_WIDTH / 4.0, TILE_HEIGHT / 2.0),       // 4
    (TILE_WIDTH * 3.0 / 4.0, TILE_HEIGHT / 2.0), // 5
    (TILE_WIDTH / 2.0, 0.0),                     // 6
];

/// Colour of the tile net labels.
const TILE_NET_COLOUR: &str = "blue";
/// How far a tile net label sits from the cell coordinates, towards the tile's point.
const TILE_NET_OFFSET: f64 = 14.0;

/// Where the cell coordinates `i,j` are written in tile `(i, j)`.
fn coordinate_position(i: usize, j: usize) -> Point {
    Point {
        x: i as f64 * TILE_WIDTH / 2.0 + TILE_WIDTH / 2.0,
        y: j as f64 * TILE_HEIGHT
            + if is_inverted(i, j) {
                TILE_HEIGHT * (1.0 - S2 / 4.0)
            } else {
                TILE_HEIGHT * S2 / 4.0
            },
    }
}

/// Whether tile `(i, j)` points down.
fn is_inverted(i: usize, j: usize) -> bool {
    (i + j) % 2 == 1
}

/// A net's node numbers, e.g. `12` for a net joining nodes 1 and 2.
fn net_label(net: usize) -> String {
    (0..6)
        .filter(|k| net & (1 << k) != 0)
        .map(|k| char::from(b'1' + k as u8))
        .collect()
}

/// The label for a tile's nets: each net's node numbers, nets separated by commas,
/// e.g. `12,5` for a net joining nodes 1 and 2 and a dead end at node 5.
pub fn tile_nets_label(tile_nets: &[usize]) -> String {
    tile_nets
        .iter()
        .map(|&net| net_label(net))
        .collect::<Vec<_>>()
        .join(",")
}

/// Writes each tile's nets (see `tile_nets_label`) beside the cell coordinates: above
/// them in a tile that points up, below in one that points down, so the label sits
/// towards the tile's point. Empty tiles get no label.
///
/// With `coloured`, each net is written in its track colour (see `net_colours`);
/// otherwise the whole label is blue.
pub fn add_tile_nets(svg: &mut Svg, tiles: &Array2D<Tile>, coloured: bool) {
    for j in 0..tiles.num_rows() {
        for i in 0..tiles.num_columns() {
            let tile_nets = &tiles[(j, i)].nodes;
            if tile_nets.is_empty() {
                continue;
            }
            let coordinates = coordinate_position(i, j);
            // The SVG's y axis points up, so "above" is a larger y.
            let offset = if is_inverted(i, j) {
                -TILE_NET_OFFSET
            } else {
                TILE_NET_OFFSET
            };
            let position = Point {
                x: coordinates.x,
                y: coordinates.y + offset,
            };
            if coloured {
                let mut parts = Vec::new();
                for (index, &net) in tile_nets.iter().enumerate() {
                    if index > 0 {
                        parts.push((",".to_string(), "#888"));
                    }
                    parts.push((net_label(net), net_colour(i, j, index)));
                }
                svg.add_coloured_parts(&parts, position, 2);
            } else {
                svg.add_coloured_text(&tile_nets_label(tile_nets), position, 2, TILE_NET_COLOUR);
            }
        }
    }
}

/// Add node numbers to the SVG for debugging/visualization purposes
///
/// This function adds numbered circles at each node position of the hexagonal tiles.
/// It prevents duplicate numbers at shared node positions between adjacent tiles.
///
/// # Arguments
/// * `svg` - Mutable reference to the Svg object to add text to
/// * `number_of_columns` - Number of columns in the maze
/// * `number_of_rows` - Number of rows in the maze
pub fn add_node_numbers(svg: &mut Svg, number_of_columns: usize, number_of_rows: usize) {
    // Add node numbers if requested

    // Track positions where text has been added to avoid duplicates
    let mut used_positions: Vec<Point> = Vec::new();

    for j in 0..number_of_rows {
        let tile_y = j as f64 * TILE_HEIGHT;
        for i in 0..number_of_columns {
            // Calculate base position for this tile
            let tile_x = i as f64 * TILE_WIDTH / 2.0;

            // Draw numbers for each active node
            for node_idx in 0..6 {
                let (local_x, local_y) = NODE_POSITIONS[node_idx];
                let position = Point {
                    x: tile_x + local_x,
                    y: tile_y
                        + if (i + j) % 2 == 0 {
                            local_y
                        } else {
                            TILE_HEIGHT - local_y
                        },
                };
                // Skip if we have already added text at this position
                let is_duplicate = used_positions.iter().any(|used_pos| {
                    let dx = position.x - used_pos.x;
                    let dy = position.y - used_pos.y;
                    (dx * dx + dy * dy) < POSITION_MARGIN_SQUARED
                });

                if !is_duplicate {
                    let text = NUMBERS[i % 3][node_idx];
                    svg.add_text(text, position, 2);
                    used_positions.push(position);
                }
            }
            svg.add_text(format!("{i},{j}").as_str(), coordinate_position(i, j), 2);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::consts::{N1, N2, N4, N5, N6};

    #[test]
    fn test_tile_nets_label() {
        assert_eq!(tile_nets_label(&[N1 | N2, N5]), "12,5");
        assert_eq!(tile_nets_label(&[N4 | N5 | N6]), "456");
        assert_eq!(tile_nets_label(&[]), "");
    }

    #[test]
    fn test_tile_net_label_sits_towards_the_point() {
        let mut maze = crate::maze::maze::Maze::new(2, 1, false, false, false, None);
        maze.tiles[(0, 0)] = Tile::new(vec![N5], true);
        maze.tiles[(0, 1)] = Tile::new(vec![N4], true);
        let mut svg = Svg::new(200.0, 200.0);
        add_tile_nets(&mut svg, &maze.tiles, false);
        let out = svg.to_string(false);
        // Tile (0,0) points up: its label is above the coordinates; (1,0) points down.
        let up = coordinate_position(0, 0).y + TILE_NET_OFFSET;
        let down = coordinate_position(1, 0).y - TILE_NET_OFFSET;
        assert!(
            out.contains(&format!("y='{up:.2}' font-size='12' fill='blue'")),
            "{out}"
        );
        assert!(
            out.contains(&format!("y='{down:.2}' font-size='12' fill='blue'")),
            "{out}"
        );
        assert!(
            out.contains(">5</text>") && out.contains(">4</text>"),
            "{out}"
        );
    }

    #[test]
    fn test_coloured_tile_net_labels_match_the_track_colours() {
        let mut maze = crate::maze::maze::Maze::new(1, 1, false, false, false, None);
        maze.tiles[(0, 0)] = Tile::new(vec![N1 | N2, N5], true);
        let mut svg = Svg::new(200.0, 200.0);
        add_tile_nets(&mut svg, &maze.tiles, true);
        let out = svg.to_string(false);
        let first = format!("<tspan fill='{}'>12</tspan>", net_colour(0, 0, 0));
        let second = format!("<tspan fill='{}'>5</tspan>", net_colour(0, 0, 1));
        assert!(out.contains(&first) && out.contains(&second), "{out}");
    }
}

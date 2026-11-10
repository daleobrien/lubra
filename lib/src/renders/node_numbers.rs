use crate::renders::svg::Svg;
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
            let position = Point {
                x: tile_x + TILE_WIDTH / 2.0,
                y: tile_y
                    + if (i + j) % 2 == 0 {
                        TILE_HEIGHT * S2 / 4.0
                    } else {
                        TILE_HEIGHT * (1.0 - S2 / 4.0)
                    },
            };
            svg.add_text(format!("{i},{j}").as_str(), position, 2);
        }
    }
}

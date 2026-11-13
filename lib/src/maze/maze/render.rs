use crate::maze::maze::FileType::{PNG, SVG};
use crate::maze::maze::{FileType, Maze};
use crate::renders::generated::fragment::MAP_PATHS;
use crate::renders::net_colours::{net_colour, net_index};
use crate::renders::node_numbers::{add_node_numbers, add_tile_nets};
use crate::renders::svg::Svg;
use crate::renders::terminal::terminal_fragment;
use common::consts::{NL, NX};
use common::types::bits;
use common::types::coordinate::{CataloguedNet, Coordinate, CoordinateAndNet};
use common::types::point::{Point, TILE_HEIGHT, TILE_WIDTH};
use std::error::Error;
use std::fs;
use std::time::Instant;
use usvg;
use usvg::fontdb;

/// Rendering methods for the Maze struct.
/// These methods handle SVG/PNG generation and file saving.
impl Maze {
    pub fn render(&mut self, file_name: String, file_type: FileType) -> Result<(), Box<dyn Error>> {
        self.render_with_options(file_name, file_type, false, false)
    }

    pub fn render_with_options(
        &mut self,
        file_name: String,
        file_type: FileType,
        background: bool,
        debug: bool,
    ) -> Result<(), Box<dyn Error>> {
        let start = Instant::now();

        let margin = 5.0;
        let w = (self.number_of_columns + 1) as f64 * TILE_WIDTH / 2.0 + 5.0 * margin;
        let h = self.number_of_rows as f64 * TILE_HEIGHT + 5.0 * margin;

        // Create header and so on
        let mut svg = Svg::new(w, h);
        svg.set_caption(self.caption.clone());

        let mut all_catorised_nets: CataloguedNet = CataloguedNet::new();

        for j in 0..self.number_of_rows {
            for i in 0..self.number_of_columns {
                let coord: Coordinate = Coordinate { i, j };

                let tile = self.tiles[(j, i)].clone();
                let surrounding_tiles = self.get_connected_titles_for_tile(i, j);

                // First, we build a list of nets (masks) and hidden nets at each coordinate
                // Then we use the still visible ones to build the SVG
                let cat_nets = tile.generate_hidden_and_visible_nets(
                    coord,
                    self.boundaries,
                    &surrounding_tiles,
                    debug, // Use debug parameter
                );
                all_catorised_nets.extend(cat_nets);
            }
        }

        if debug {
            println!("render took (1) {:?}", start.elapsed());
        }
        let start = Instant::now();

        for coordinate_and_net in all_catorised_nets.visible.iter() {
            // We don't want to draw the hidden nets
            let masked_net: CoordinateAndNet = CoordinateAndNet {
                // IN & WA
                net: coordinate_and_net.net & NX,
                coordinate: coordinate_and_net.coordinate,
            };

            if all_catorised_nets.hidden.contains(&masked_net) {
                continue;
            }

            let net = bits::transform_tile(coordinate_and_net.net, coordinate_and_net.coordinate);

            let point: Point = Point {
                x: coordinate_and_net.coordinate.i as f64,
                y: coordinate_and_net.coordinate.j as f64,
            };

            // The entrance and exit dead ends are drawn as markers.
            let Coordinate { i, j } = coordinate_and_net.coordinate;
            if let Some(marker) = terminal_fragment(net).filter(|_| self.is_terminal(i, j)) {
                svg.add_fragment(&marker, point, 1);
                continue;
            }

            let frag = MAP_PATHS.get(&net);

            if frag.is_none() {
                continue;
            }
            if frag.unwrap().is_empty() {
                continue;
            }

            if self.colour_tracks {
                // Colour the walls by the tile net this fragment draws.
                let nodes = coordinate_and_net.net & NL;
                if let Some(index) = net_index(&self.tiles[(j, i)].nodes, nodes) {
                    let stroke = format!("stroke='{}'", net_colour(i, j, index));
                    svg.add_fragment(&frag.unwrap().replace("stroke='black'", &stroke), point, 1);
                    continue;
                }
            }

            // Every fragment is drawn on one layer: where tracks cross, the one drawn
            // later passes over. Cross-tile fragments don't need a layer of their own
            // because PATH_WIDTH keeps an arc that dips into a neighbouring tile clear
            // of that tile's openings.
            svg.add_fragment(frag.unwrap(), point, 1);
        }

        if self.show_node_numbers {
            add_node_numbers(&mut svg, self.number_of_columns, self.number_of_rows);
        }
        if self.show_tile_nets {
            add_tile_nets(&mut svg, &self.tiles, self.colour_tracks);
        }

        if debug {
            println!("render took (2) {:?}", start.elapsed());
        }

        match file_type {
            SVG => self.save_as_svg(svg.to_string(background), file_name, debug),
            PNG => self.save_as_png(svg.to_string(background).as_str(), file_name, debug),
        }
    }

    pub fn save_as_svg(
        &mut self,
        content: String,
        file_name: String,
        debug: bool,
    ) -> Result<(), Box<dyn Error>> {
        if debug {
            let start = Instant::now();
            fs::write(format!("{}.svg", file_name), content).expect("Unable to write file");
            println!("SVG file write took: {:?}", start.elapsed());
        } else {
            fs::write(format!("{}.svg", file_name), content).expect("Unable to write file");
        }
        Ok(())
    }

    pub fn save_as_png(
        &mut self,
        svg_content: &str,
        file_name: String,
        debug: bool,
    ) -> Result<(), Box<dyn Error>> {
        let input = format!("{}.png", file_name);
        let mut fontdb = fontdb::Database::new();
        fontdb.load_system_fonts();

        let parse_start = Instant::now();
        let tree = usvg::Tree::from_str(svg_content, &usvg::Options::default(), &fontdb).unwrap();
        if debug {
            println!("SVG parsing took: {:?}", parse_start.elapsed());
        }

        let zoom = 1f32;
        let pixmap_size = tree.size().to_int_size().scale_by(zoom).unwrap();
        if debug {
            println!("Size w:{} h:{}", pixmap_size.width(), pixmap_size.height());
        }

        let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();

        let render_ts = tiny_skia::Transform::from_scale(zoom, zoom);

        // This is very slow when running on debug builds !!!
        let render_start = Instant::now();
        resvg::render(&tree, render_ts, &mut pixmap.as_mut());
        if debug {
            println!("Rendering to PNG took: {:?}", render_start.elapsed());
        }

        let write_start = Instant::now();
        pixmap.save_png(&input).unwrap();
        if debug {
            println!("PNG file write took: {:?}", write_start.elapsed());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::maze::maze::FileType::SVG;
    use crate::maze::maze::Maze;

    #[test]
    fn test_render_is_byte_identical_for_a_seed() {
        let dir = std::env::temp_dir();
        let render = |name: &str| {
            let mut maze = Maze::new(12, 8, true, true, false, Some(3));
            maze.reduce_nets(1);
            maze.remove_cycles(false);
            let path = dir.join(name);
            maze.render(path.to_string_lossy().into_owned(), SVG)
                .expect("render failed");
            let svg_path = path.with_extension("svg");
            let bytes = std::fs::read(&svg_path).expect("missing SVG");
            let _ = std::fs::remove_file(svg_path);
            bytes
        };
        assert!(render("lubra-determinism-a") == render("lubra-determinism-b"));
    }
}

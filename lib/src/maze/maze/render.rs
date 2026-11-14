use crate::maze::maze::FileType::{PNG, SVG};
use crate::maze::maze::{FileType, Maze};
use crate::maze::tile::Tile;
use crate::renders::generated::fragment::MAP_PATHS;
use crate::renders::kinks::{find_kinks, DrawnFragment, Kink, KinkKind};
use crate::renders::net_colours::{net_colour, net_index};
use crate::renders::node_numbers::{add_node_numbers, add_tile_nets};
use crate::renders::svg::Svg;
use crate::renders::terminal::terminal_fragment;
use common::consts::{NL, NX, WA};
use common::types::bits;
use common::types::coordinate::{CataloguedNet, Coordinate, CoordinateAndNet};
use common::types::point::{Point, TILE_HEIGHT, TILE_WIDTH};
use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
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

        let drawn = self.drawn_fragments(debug);
        if debug {
            println!("render took (1) {:?}", start.elapsed());
        }
        let start = Instant::now();

        for fragment in &drawn {
            let Coordinate { i, j } = fragment.coordinate;
            let point: Point = Point {
                x: i as f64,
                y: j as f64,
            };
            if self.colour_tracks && !fragment.marker {
                // Colour the walls by the tile net this fragment draws.
                if let Some(index) = net_index(&self.tiles[(j, i)].nodes, fragment.nodes) {
                    let stroke = format!("stroke='{}'", net_colour(i, j, index));
                    let coloured = fragment.svg.replace("stroke='black'", &stroke);
                    svg.add_fragment(&coloured, point, 1);
                    continue;
                }
            }

            // Every fragment is drawn on one layer: where tracks cross, the one drawn
            // later passes over. Cross-tile fragments don't need a layer of their own
            // because PATH_WIDTH keeps an arc that dips into a neighbouring tile clear
            // of that tile's openings.
            svg.add_fragment(&fragment.svg, point, 1);
        }

        if self.show_kinks {
            let kinks = find_kinks(&placed(&drawn));
            for kink in &kinks {
                let colour = match kink.kind {
                    KinkKind::Break => KINK_BREAK_COLOUR,
                    KinkKind::Wiggle => KINK_WIGGLE_COLOUR,
                };
                svg.add_ring(kink.x, kink.y, colour, 2);
            }
            if let Some(caption) = &self.caption {
                svg.set_caption(Some(format!("{caption}   kinks: {}", kinks.len())));
            }
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

    /// Every fragment the maze draws, in drawing order.
    fn drawn_fragments(&self, debug: bool) -> Vec<Drawn> {
        // First, we build a list of nets (masks) and hidden nets at each coordinate
        // Then we use the still visible ones to build the SVG
        let mut per_tile: Vec<(Coordinate, CataloguedNet)> = Vec::new();
        for j in 0..self.number_of_rows {
            for i in 0..self.number_of_columns {
                let coord = Coordinate { i, j };
                per_tile.push((coord, self.catalogue_tile(coord, debug, true)));
            }
        }

        // Some fragments replace a net in the tile across a vertex. Where that net is
        // already drawn by something else, draw those tiles the usual way.
        let conflicts = corner_hook_conflicts(
            &per_tile,
            |coord, key| self.vertex_s_blocked(coord.i, coord.j, key),
            |coord, key| {
                self.vertex_s_footprint(coord.i, coord.j, key)
                    .into_iter()
                    .map(|(i, j)| Coordinate { i, j })
                    .collect()
            },
        );
        let mut all_catorised_nets: CataloguedNet = CataloguedNet::new();
        for (coord, catalogue) in per_tile {
            if conflicts.contains(&coord) {
                all_catorised_nets.extend(self.catalogue_tile(coord, debug, false));
            } else {
                all_catorised_nets.extend(catalogue);
            }
        }

        let mut drawn = Vec::new();
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

            let key = bits::transform_tile(coordinate_and_net.net, coordinate_and_net.coordinate);
            let coordinate = coordinate_and_net.coordinate;
            let nodes = coordinate_and_net.net & NL;

            // The entrance and exit dead ends are drawn as markers.
            let terminal = self.is_terminal(coordinate.i, coordinate.j);
            if let Some(marker) = terminal_fragment(key).filter(|_| terminal) {
                drawn.push(Drawn {
                    coordinate,
                    key,
                    nodes,
                    svg: Cow::Owned(marker),
                    marker: true,
                });
                continue;
            }
            match MAP_PATHS.get(&key) {
                Some(fragment) if !fragment.is_empty() => drawn.push(Drawn {
                    coordinate,
                    key,
                    nodes,
                    svg: Cow::Borrowed(fragment),
                    marker: false,
                }),
                _ => {}
            }
        }
        drawn
    }

    /// The kinks in the maze as it would be drawn (see `renders::kinks`).
    pub fn kinks(&self) -> Vec<Kink> {
        find_kinks(&placed(&self.drawn_fragments(false)))
    }

    /// The fragments for tile `coord` and the neighbouring nets they replace.
    fn catalogue_tile(&self, coord: Coordinate, debug: bool, corner_hooks: bool) -> CataloguedNet {
        let Coordinate { i, j } = coord;
        let tile = self.tiles[(j, i)].clone();
        let surrounding_tiles = self.get_connected_titles_for_tile(i, j);
        tile.generate_hidden_and_visible_nets(
            coord,
            self.boundaries,
            &surrounding_tiles,
            debug,
            corner_hooks,
        )
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

/// Ring colours for the kink overlay.
const KINK_BREAK_COLOUR: &str = "#E02020";
const KINK_WIGGLE_COLOUR: &str = "#C020C0";

/// One fragment to draw.
struct Drawn {
    coordinate: Coordinate,
    /// The `MAP_PATHS` key (drawn orientation).
    key: usize,
    /// The tile net it draws, as stored in the tile.
    nodes: usize,
    svg: Cow<'static, str>,
    /// Whether it is an entrance or exit marker.
    marker: bool,
}

/// The drawn fragments with their positions in drawing units, for kink detection.
/// Entrance and exit markers and tile outlines are left out: they are not track.
fn placed(drawn: &[Drawn]) -> Vec<DrawnFragment<'_>> {
    drawn
        .iter()
        .filter(|fragment| !fragment.marker && fragment.key & WA == 0)
        .map(|fragment| DrawnFragment {
            key: fragment.key,
            fragment: &fragment.svg,
            x: fragment.coordinate.i as f64 * TILE_WIDTH / 2.0,
            y: fragment.coordinate.j as f64 * TILE_HEIGHT,
        })
        .collect()
}

/// The tiles to draw without fragments that reach across a vertex (see
/// `Tile::reaches_across_vertex`).
///
/// A tile is dropped when such a fragment claims a net that something else also draws
/// (another fragment replacing the same net, or a cross-tile fragment of that net's own
/// tile), or when `blocked(tile, key)` says its drawing would run into a neighbouring
/// track. Two vertex S fragments must not share any tile round their vertices
/// (`footprint`), as their half-circles would cross; the first in grid order is kept. Fragments across a vertex come in pairs (one draws, the tile opposite gets a
/// blank that hides the first tile's net), so when one tile of a pair is dropped, so is
/// the other; otherwise the blank would hide a net that is no longer drawn.
fn corner_hook_conflicts(
    per_tile: &[(Coordinate, CataloguedNet)],
    blocked: impl Fn(Coordinate, usize) -> bool,
    footprint: impl Fn(Coordinate, usize) -> Vec<Coordinate>,
) -> HashSet<Coordinate> {
    let across = |catalogue: &CataloguedNet| {
        catalogue
            .visible
            .iter()
            .any(|v| Tile::reaches_across_vertex(v.net & NX))
    };
    let mut claims: HashMap<CoordinateAndNet, usize> = HashMap::new();
    let mut drawn_cross_tile: HashSet<CoordinateAndNet> = HashSet::new();
    for (_, catalogue) in per_tile {
        for hidden in &catalogue.hidden {
            *claims.entry(*hidden).or_default() += 1;
        }
        for visible in &catalogue.visible {
            // Upper bits name nets in neighbouring tiles. Fragments across a vertex are
            // checked through their claims instead, as their partner draws a blank.
            let net = visible.net & NX;
            if net >> 6 != 0 && !Tile::reaches_across_vertex(net) {
                drawn_cross_tile.insert(CoordinateAndNet::new(visible.coordinate, net & NL));
            }
        }
    }
    let mut dropped: HashSet<Coordinate> = per_tile
        .iter()
        .filter(|(_, catalogue)| across(catalogue))
        .filter(|(coord, catalogue)| {
            let runs_into = catalogue
                .visible
                .iter()
                .any(|v| blocked(*coord, v.net & NX));
            runs_into
                || catalogue.hidden.iter().any(|hidden| {
                    claims[hidden] > 1
                        || drawn_cross_tile
                            .contains(&CoordinateAndNet::new(hidden.coordinate, hidden.net & NL))
                })
        })
        .map(|(coord, _)| *coord)
        .collect();

    // A vertex S swings into the tiles flanking its vertex. Their own fragments are
    // checked by `blocked`, but only in their single-tile shapes, so keep the S out of
    // flanking tiles that draw, or are drawn by, a cross-tile fragment.
    let cross_tile_tiles: HashSet<Coordinate> = per_tile
        .iter()
        .filter(|(_, c)| c.visible.iter().any(|v| (v.net & NX) >> 6 != 0))
        .map(|(coord, _)| *coord)
        .chain(
            per_tile
                .iter()
                .flat_map(|(_, c)| c.hidden.iter().map(|h| h.coordinate)),
        )
        .collect();
    for (coord, catalogue) in per_tile {
        for visible in &catalogue.visible {
            let tiles = footprint(*coord, visible.net & NX);
            // The first two are the S's own tile and its partner across the vertex.
            if tiles.iter().skip(2).any(|t| cross_tile_tiles.contains(t)) {
                dropped.insert(*coord);
            }
        }
    }

    // Keep vertex S fragments apart.
    let mut taken: HashSet<Coordinate> = HashSet::new();
    for (coord, catalogue) in per_tile {
        if dropped.contains(coord) {
            continue;
        }
        let tiles: Vec<Coordinate> = catalogue
            .visible
            .iter()
            .flat_map(|v| footprint(*coord, v.net & NX))
            .collect();
        if tiles.iter().any(|t| taken.contains(t)) {
            dropped.insert(*coord);
        } else {
            taken.extend(tiles);
        }
    }

    // Drop partners with them: tiles whose across-vertex fragment hides a net in a
    // dropped tile, or whose net a dropped tile's fragment hides.
    loop {
        let mut grew = false;
        for (coord, catalogue) in per_tile.iter().filter(|(_, c)| across(c)) {
            let hides_dropped = catalogue
                .hidden
                .iter()
                .any(|h| dropped.contains(&h.coordinate));
            let hidden_by_dropped = per_tile.iter().any(|(other, c)| {
                dropped.contains(other)
                    && across(c)
                    && c.hidden.iter().any(|h| h.coordinate == *coord)
            });
            if !dropped.contains(coord) && (hides_dropped || hidden_by_dropped) {
                dropped.insert(*coord);
                grew = true;
            }
        }
        if !grew {
            return dropped;
        }
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

    #[test]
    fn test_show_kinks_rings_every_kink_and_counts_them_in_the_caption() {
        let mut maze = Maze::new(12, 8, true, true, false, Some(3));
        maze.reduce_nets(1);
        maze.remove_cycles(false);
        // The maze draws tile outlines, whose corners must not count.
        let kinks = maze.kinks().len();
        assert!((1..40).contains(&kinks), "{kinks} kinks");

        maze.set_show_kinks(true);
        maze.set_caption(Some("caption".to_string()));
        let path = std::env::temp_dir().join("lubra-kinks");
        maze.render(path.to_string_lossy().into_owned(), SVG)
            .expect("render failed");
        let svg_path = path.with_extension("svg");
        let svg = std::fs::read_to_string(&svg_path).expect("missing SVG");
        let _ = std::fs::remove_file(svg_path);
        assert_eq!(svg.matches("stroke-width='3'").count(), kinks);
        assert!(svg.contains(&format!(">caption   kinks: {kinks}</text>")));
    }
}

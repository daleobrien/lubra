use super::draw_order::Piece;
use super::solution::RouteStep;
use super::touching::crossing_in_same_tile;
use crate::maze::maze::FileType::{PNG, SVG};
use crate::maze::maze::{FileType, Maze};
use crate::maze::tile::Tile;
use crate::renders::generated::fragment::MAP_PATHS;
use crate::renders::ink::{fragment_walls, ink_fragment, solution_markup, Pen};
use crate::renders::kinks::{find_kinks, DrawnFragment, Kink, KinkKind};
use crate::renders::net_colours::{net_colour, net_index};
use crate::renders::node_numbers::{add_node_numbers, add_tile_nets};
use crate::renders::svg::Svg;
use crate::renders::terminal::terminal_fragment;
use crate::renders::vertex_s::is_vertex_s;
use common::consts::{NL, NX, WA};
use common::nodes::get_connected_coords_at_coords;
use common::types::bits;
use common::types::bits::transform_tile;
use common::types::coordinate::{CataloguedNet, Coordinate, CoordinateAndNet};
use common::types::point::{Point, PATH_WIDTH, TILE_HEIGHT, TILE_WIDTH};
use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;
use std::time::Instant;

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
        let content = self.svg_markup(background, debug);
        match file_type {
            SVG => self.save_as_svg(content, file_name, debug),
            PNG => self.save_as_png(content.as_str(), file_name, debug),
        }
    }

    /// The maze as SVG text, drawn as the render settings say. With `debug`, timings
    /// are printed (not on WebAssembly, which has no clock to read).
    pub fn svg_markup(&self, background: bool, debug: bool) -> String {
        let debug = debug && cfg!(not(target_arch = "wasm32"));
        let start = debug.then(Instant::now);

        let margin = 5.0;
        let w = (self.number_of_columns + 1) as f64 * TILE_WIDTH / 2.0 + 5.0 * margin;
        let h = self.number_of_rows as f64 * TILE_HEIGHT + 5.0 * margin;

        // Create header and so on
        let mut svg = Svg::new(w, h);
        svg.set_caption(self.caption.clone());

        let drawn = self.drawn_fragments(debug);
        if let Some(start) = start {
            println!("render took (1) {:?}", start.elapsed());
        }
        let start = debug.then(Instant::now);

        let pen = Pen::new(self.seed.unwrap_or_default());
        for fragment in &drawn {
            let Coordinate { i, j } = fragment.coordinate;
            let point: Point = Point {
                x: i as f64,
                y: j as f64,
            };
            // Drawn by hand, every tile is different, so it is written out in place.
            // Tile outlines stay plain: they are a debugging aid, not track.
            if self.ink && fragment.key & WA == 0 {
                let offset = (i as f64 * TILE_WIDTH / 2.0, j as f64 * TILE_HEIGHT);
                svg.add_markup(&ink_fragment(&fragment.svg, offset, &pen), 1);
                continue;
            }
            if self.colour_tracks && !fragment.marker {
                // Colour the walls by the tile net this fragment draws.
                if let Some(index) = net_index(&self.tiles[(j, i)].nodes, fragment.nodes) {
                    let stroke = format!("stroke='{}'", net_colour(i, j, index));
                    let coloured = fragment.svg.replace("stroke='black'", &stroke);
                    svg.add_fragment(&coloured, point, 1);
                    continue;
                }
            }

            // Every fragment is drawn on one layer, in the order `drawn_fragments`
            // gives (see `draw_order`): where tracks cross, the one drawn later passes
            // over.
            svg.add_fragment(&fragment.svg, point, 1);
        }

        if self.show_solution {
            let (over, under) = self.solution_trace(&drawn);
            let ink_pen = self.ink.then_some(&pen);
            svg.add_markup(&solution_markup(&over, &under, ink_pen), 1);
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

        if let Some(start) = start {
            println!("render took (2) {:?}", start.elapsed());
        }
        svg.to_string(background)
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
                    logical: coordinate_and_net.net & NX,
                    key,
                    nodes,
                    svg: Cow::Owned(marker),
                    marker: true,
                });
                continue;
            }
            match MAP_PATHS.get(&(key as u64)) {
                Some(fragment) if !fragment.is_empty() => drawn.push(Drawn {
                    coordinate,
                    logical: coordinate_and_net.net & NX,
                    key,
                    nodes,
                    svg: Cow::Borrowed(fragment),
                    marker: false,
                }),
                _ => {}
            }
        }
        let pieces: Vec<Piece> = drawn
            .iter()
            .map(|fragment| Piece {
                i: fragment.coordinate.i,
                j: fragment.coordinate.j,
                nodes: fragment.nodes,
                track: !fragment.marker && fragment.key & WA == 0,
                svg: &fragment.svg,
            })
            .collect();
        let order = self.drawing_order(&pieces);
        let mut slots: Vec<Option<Drawn>> = drawn.into_iter().map(Some).collect();
        order
            .into_iter()
            .filter_map(|index| slots[index].take())
            .collect()
    }

    /// The route from the entrance to the exit as lines in drawing units, split into
    /// where it is on top and where it passes under another track.
    ///
    /// Each stretch of the route drawn by one fragment runs from an opening to an
    /// opening. Where a wall of that fragment joins the two openings, the trace is
    /// that wall moved half a track's width inwards, so it follows the track exactly.
    /// Otherwise (across the middle of a junction) it is a smooth curve leaving and
    /// arriving square to the openings.
    fn solution_trace(&self, drawn: &[Drawn]) -> (Vec<Vec<TracePoint>>, Vec<Vec<TracePoint>>) {
        self.trace_route(drawn, &self.solution_route())
    }

    /// Any route through the maze (as `solution_route` lists one) as lines in drawing
    /// units, split into where it is on top and where it passes under another track.
    /// A last step with no way out ends where it came in, unless it is the exit.
    pub fn route_lines(&self, route: &[RouteStep]) -> (Vec<Vec<TracePoint>>, Vec<Vec<TracePoint>>) {
        let drawn = self.drawn_fragments(false);
        let end = match route.last() {
            Some(last) if last.exit.is_none() && !self.is_terminal(last.i, last.j) => {
                route.len() - 1
            }
            _ => route.len(),
        };
        self.trace_route(&drawn, &route[..end])
    }

    /// Where logical node `node` of tile `(i, j)` opens, in drawing units; the middle of
    /// the tile when there is no node.
    pub fn opening_point(&self, i: usize, j: usize, node: Option<usize>) -> (f64, f64) {
        let step = RouteStep {
            i,
            j,
            net: 0,
            entry: node,
            exit: None,
        };
        endpoint(step, node).at
    }

    /// `route` as lines in drawing units (see `solution_trace`).
    fn trace_route(
        &self,
        drawn: &[Drawn],
        route: &[RouteStep],
    ) -> (Vec<Vec<TracePoint>>, Vec<Vec<TracePoint>>) {
        // Which drawn fragment draws each tile net.
        let mut drawer: HashMap<(Coordinate, usize), usize> = HashMap::new();
        for (index, fragment) in drawn.iter().enumerate() {
            if fragment.key & WA != 0 {
                continue;
            }
            let own = fragment.logical & NL;
            drawer.insert((fragment.coordinate, own), index);
            for group in 1..5 {
                let net = (fragment.logical >> (6 * group)) & NL;
                let shared = net & own;
                if net == 0 || shared == 0 {
                    continue;
                }
                let node = shared.trailing_zeros() as usize;
                if let Some(across) = get_connected_coords_at_coords(fragment.coordinate, node) {
                    drawer.insert((across, net), index);
                }
            }
        }

        let (mut over, mut under): (Vec<Vec<TracePoint>>, Vec<Vec<TracePoint>>) =
            Default::default();
        let mut previous_under: Option<bool> = None;
        let mut start = 0;
        while start < route.len() {
            let step = route[start];
            let coordinate = Coordinate {
                i: step.i,
                j: step.j,
            };
            let Some(&index) = drawer.get(&(coordinate, step.net)) else {
                start += 1;
                continue;
            };
            // The steps this fragment draws.
            let mut end = start;
            while end + 1 < route.len() {
                let next = route[end + 1];
                let next_coordinate = Coordinate {
                    i: next.i,
                    j: next.j,
                };
                if drawer.get(&(next_coordinate, next.net)) != Some(&index) {
                    break;
                }
                end += 1;
            }
            let last = route[end];
            let from = endpoint(step, step.entry);
            let to = endpoint(last, last.exit);
            let fragment = &drawn[index];
            let walls = fragment_walls(&fragment.svg, tile_origin(fragment.coordinate), TRACE_STEP);
            let line = along_wall(&walls, from, to).unwrap_or_else(|| curve(from, to));

            let beneath = (start..=end).any(|k| self.passes_under(route[k], index, drawn));
            let lines = if beneath { &mut under } else { &mut over };
            // Carry on the same line when the last stretch was drawn the same way.
            match lines.last_mut() {
                Some(current) if previous_under == Some(beneath) => {
                    current.extend(line.into_iter().skip(1))
                }
                _ => lines.push(line),
            }
            previous_under = Some(beneath);
            start = end + 1;
        }
        (over, under)
    }

    /// Whether the route's tile net `step` passes under another tile net of its tile:
    /// they cross, and the other is drawn after `index`, the fragment drawing the step.
    fn passes_under(&self, step: RouteStep, index: usize, drawn: &[Drawn]) -> bool {
        let coordinate = Coordinate {
            i: step.i,
            j: step.j,
        };
        drawn.iter().enumerate().skip(index + 1).any(|(_, other)| {
            other.coordinate == coordinate
                && other.key & WA == 0
                && other.nodes & step.net == 0
                && crossing_in_same_tile(step.net, other.nodes)
        })
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
        let terminals = [
            Coordinate { i: 0, j: 0 },
            Coordinate {
                i: self.number_of_columns - 1,
                j: self.number_of_rows - 1,
            },
        ];
        tile.generate_hidden_and_visible_nets(
            coord,
            self.boundaries,
            &surrounding_tiles,
            &terminals,
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

    #[cfg(feature = "png")]
    pub fn save_as_png(
        &mut self,
        svg_content: &str,
        file_name: String,
        debug: bool,
    ) -> Result<(), Box<dyn Error>> {
        use usvg::fontdb;
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

    /// PNG output needs the `png` feature (on by default).
    #[cfg(not(feature = "png"))]
    pub fn save_as_png(
        &mut self,
        _svg_content: &str,
        _file_name: String,
        _debug: bool,
    ) -> Result<(), Box<dyn Error>> {
        Err("PNG output needs the lib crate's `png` feature".into())
    }
}

/// Ring colours for the kink overlay.
const KINK_BREAK_COLOUR: &str = "#E02020";
const KINK_WIGGLE_COLOUR: &str = "#C020C0";

/// One fragment to draw.
struct Drawn {
    coordinate: Coordinate,
    /// The key as the tile sees it (its own net, then the neighbouring nets it draws).
    logical: usize,
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

/// A point of the solution trace, in drawing units.
type TracePoint = (f64, f64);

/// The trace is built from walls flattened to pieces this long.
const TRACE_STEP: f64 = 2.0;

/// Where tile `coordinate` is placed, in drawing units.
fn tile_origin(coordinate: Coordinate) -> TracePoint {
    (
        coordinate.i as f64 * TILE_WIDTH / 2.0,
        coordinate.j as f64 * TILE_HEIGHT,
    )
}

/// One end of a stretch of the route: a point and the direction into the tile.
#[derive(Clone, Copy)]
struct Opening {
    at: TracePoint,
    inwards: TracePoint,
}

fn unit((x, y): TracePoint) -> TracePoint {
    let length = x.hypot(y);
    (x / length, y / length)
}

/// The opening of logical node `node` of the step's tile, or the middle of the tile
/// (the entrance and exit, where the route starts and ends) when there is none.
fn endpoint(step: RouteStep, node: Option<usize>) -> Opening {
    let coordinate = Coordinate {
        i: step.i,
        j: step.j,
    };
    let (x0, y0) = tile_origin(coordinate);
    let inverted = (step.i + step.j) % 2 == 1;
    let y = |y: f64| if inverted { TILE_HEIGHT - y } else { y };
    let vertices = [
        (TILE_WIDTH / 2.0, y(TILE_HEIGHT)),
        (0.0, y(0.0)),
        (TILE_WIDTH, y(0.0)),
    ];
    let centre = (TILE_WIDTH / 2.0, y(TILE_HEIGHT / 3.0));
    let Some(node) = node else {
        return Opening {
            at: (x0 + centre.0, y0 + centre.1),
            inwards: (0.0, 0.0),
        };
    };
    // The node as drawn: tiles are drawn turned and flipped.
    let drawn = (transform_tile(1 << node, coordinate) & NL).trailing_zeros() as usize;
    let at = match drawn {
        0..=2 => vertices[drawn],
        _ => {
            let (a, b) = [(0, 1), (0, 2), (1, 2)][drawn - 3];
            (
                (vertices[a].0 + vertices[b].0) / 2.0,
                (vertices[a].1 + vertices[b].1) / 2.0,
            )
        }
    };
    Opening {
        at: (x0 + at.0, y0 + at.1),
        inwards: unit((centre.0 - at.0, centre.1 - at.1)),
    }
}

fn distance(a: TracePoint, b: TracePoint) -> f64 {
    (a.0 - b.0).hypot(a.1 - b.1)
}

/// A wall of the fragment that runs from beside opening `from` to beside opening
/// `to`, moved half a track's width towards the track: the track's middle line. The
/// walls of one fragment are joined end to end first.
fn along_wall(walls: &[Vec<TracePoint>], from: Opening, to: Opening) -> Option<Vec<TracePoint>> {
    let half_width = PATH_WIDTH * TILE_WIDTH;
    let beside = |opening: Opening, p: TracePoint| {
        (distance(opening.at, p) - half_width).abs() < 0.6
            && (opening.inwards == (0.0, 0.0)
                || ((p.0 - opening.at.0) * opening.inwards.0
                    + (p.1 - opening.at.1) * opening.inwards.1)
                    .abs()
                    < 0.6)
    };
    for wall in joined(walls) {
        let (first, last) = (wall[0], wall[wall.len() - 1]);
        let points: Vec<TracePoint> = if beside(from, first) && beside(to, last) {
            wall
        } else if beside(from, last) && beside(to, first) {
            wall.into_iter().rev().collect()
        } else {
            continue;
        };
        for side in [1.0, -1.0] {
            let moved = offset(&points, side * half_width);
            if distance(moved[0], from.at) < 1.0 && distance(moved[moved.len() - 1], to.at) < 1.0 {
                let mut line = moved;
                line[0] = from.at;
                let end = line.len() - 1;
                line[end] = to.at;
                return Some(line);
            }
        }
    }
    None
}

/// The walls with those that meet end to end joined into one.
fn joined(walls: &[Vec<TracePoint>]) -> Vec<Vec<TracePoint>> {
    let mut out: Vec<Vec<TracePoint>> = walls.to_vec();
    let close = |a: TracePoint, b: TracePoint| distance(a, b) < 0.3;
    let mut merged = true;
    while merged {
        merged = false;
        'search: for a in 0..out.len() {
            for b in 0..out.len() {
                if a == b {
                    continue;
                }
                let (a_end, b_start, b_end) = (
                    out[a][out[a].len() - 1],
                    out[b][0],
                    out[b][out[b].len() - 1],
                );
                let tail: Vec<TracePoint> = if close(a_end, b_start) {
                    out[b][1..].to_vec()
                } else if close(a_end, b_end) {
                    out[b].iter().rev().skip(1).copied().collect()
                } else {
                    continue;
                };
                out[a].extend(tail);
                out.remove(b);
                merged = true;
                break 'search;
            }
        }
    }
    out
}

/// `points` moved `by` to their left (negative for the right).
fn offset(points: &[TracePoint], by: f64) -> Vec<TracePoint> {
    (0..points.len())
        .map(|k| {
            let before = points[k.saturating_sub(1)];
            let after = points[(k + 1).min(points.len() - 1)];
            let (dx, dy) = unit((after.0 - before.0, after.1 - before.1));
            (points[k].0 - dy * by, points[k].1 + dx * by)
        })
        .collect()
}

/// A smooth curve from `from` to `to`, leaving and arriving square to the openings.
fn curve(from: Opening, to: Opening) -> Vec<TracePoint> {
    let reach = distance(from.at, to.at) * 0.4;
    let c1 = (
        from.at.0 + from.inwards.0 * reach,
        from.at.1 + from.inwards.1 * reach,
    );
    let c2 = (
        to.at.0 + to.inwards.0 * reach,
        to.at.1 + to.inwards.1 * reach,
    );
    let n = 24;
    (0..=n)
        .map(|k| {
            let t = k as f64 / n as f64;
            let u = 1.0 - t;
            let (a, b, c, d) = (u * u * u, 3.0 * u * u * t, 3.0 * u * t * t, t * t * t);
            (
                a * from.at.0 + b * c1.0 + c * c2.0 + d * to.at.0,
                a * from.at.1 + b * c1.1 + c * c2.1 + d * to.at.1,
            )
        })
        .collect()
}

/// The tiles to draw without fragments that reach across a vertex (see
/// `Tile::reaches_across_vertex`).
///
/// A tile is dropped when such a fragment claims a net that something else also draws
/// (another fragment replacing the same net, or a cross-tile fragment of that net's own
/// tile), or when `blocked(tile, key)` says its drawing would run into a neighbouring
/// track. Two vertex S or sweep fragments must not share any tile round their vertices
/// (`footprint`), as their curves would cross; the first in grid order is kept, every S
/// before the sweeps. Most fragments across a vertex come in pairs (one draws, the tile
/// opposite gets a blank that hides the first tile's net), so when one tile of a pair is
/// dropped, so is the other; otherwise the blank would hide a net that is no longer drawn.
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
    // flanking tiles that draw, or are drawn by, a cross-tile fragment. A fragment that
    // redraws a junction (`Tile::redraws_junction`) does not count: it stays inside the
    // space of the single-tile shapes it replaces.
    let reshapes = |catalogue: &CataloguedNet| {
        catalogue.visible.iter().any(|v| {
            let key = v.net & NX;
            key >> 6 != 0 && !Tile::redraws_junction(key)
        })
    };
    let cross_tile_tiles: HashSet<Coordinate> = per_tile
        .iter()
        .filter(|(_, catalogue)| reshapes(catalogue))
        .flat_map(|(coord, catalogue)| {
            std::iter::once(*coord).chain(catalogue.hidden.iter().map(|h| h.coordinate))
        })
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

    // Keep vertex S and vertex sweep fragments apart: the first in grid order is kept,
    // with every S ahead of the sweeps, as an S replaces a kink and a sweep a wiggle.
    let is_s =
        |catalogue: &CataloguedNet| catalogue.visible.iter().any(|v| is_vertex_s(v.net & NX));
    let s_first = per_tile
        .iter()
        .filter(|(_, c)| is_s(c))
        .chain(per_tile.iter().filter(|(_, c)| !is_s(c)));
    let mut taken: HashSet<Coordinate> = HashSet::new();
    for (coord, catalogue) in s_first {
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

    // Drop partners with them. A pair hides each other's net (one draws both tracks,
    // the other draws a blank), so when one is dropped the other must be too. A
    // fragment that only hides its neighbour's net, such as a vertex sweep, has no
    // partner: once it is dropped the neighbour's net is simply drawn again.
    loop {
        let mut grew = false;
        for (coord, catalogue) in per_tile.iter().filter(|(_, c)| across(c)) {
            let partner_dropped = per_tile.iter().any(|(other, c)| {
                dropped.contains(other)
                    && across(c)
                    && c.hidden.iter().any(|h| h.coordinate == *coord)
                    && catalogue.hidden.iter().any(|h| h.coordinate == *other)
            });
            if !dropped.contains(coord) && partner_dropped {
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
        // The maze draws tile outlines, whose corners must not count. (Detection
        // itself is tested in `renders::kinks`; this maze may have no kinks at all.)
        let kinks = maze.kinks().len();
        assert!(kinks < 40, "{kinks} kinks");

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

    /// The keys of the drawn fragments that redraw a junction in a neighbouring tile.
    fn junction_redraws(maze: &Maze) -> usize {
        maze.drawn_fragments(false)
            .iter()
            .filter(|fragment| crate::maze::tile::Tile::redraws_junction(fragment.key))
            .count()
    }

    #[test]
    fn test_hook_fork_is_drawn_only_when_both_tiles_are_clear() {
        use crate::maze::tile::Tile;
        use common::consts::{N1, N2, N3, N4, N5, N6};

        // An arc round vertex 2 (edges 4 and 6), and across edge 4 a junction that
        // hooks from vertex 2 to that edge.
        let (i, j) = (4, 4);
        let mut maze = Maze::new(9, 8, false, false, false, None);
        let (ti, tj) = maze.tile_across(i, j, 3).expect("a tile across edge 4");
        maze.tiles[(j, i)] = Tile::new(vec![N4 | N6], true);
        maze.tiles[(tj, ti)] = Tile::new(vec![N2 | N4 | N5], true);
        assert_eq!(junction_redraws(&maze), 1);
        // One fragment draws both nets.
        assert_eq!(maze.drawn_fragments(false).len(), 1);

        // A dead end in the arc's far corner is clear of the arc, so nothing changes.
        maze.tiles[(j, i)] = Tile::new(vec![N4 | N6, N3], true);
        assert_eq!(junction_redraws(&maze), 1);

        // A track crossing the arc is not, so both tiles are drawn the usual way.
        maze.tiles[(j, i)] = Tile::new(vec![N4 | N6, N1 | N5], true);
        let crosses = !crate::maze::maze::touching::clear_in_same_tile(N4 | N6, N1 | N5);
        assert_eq!(junction_redraws(&maze), usize::from(!crosses));
    }

    #[test]
    fn test_ink_draws_every_track_in_place_and_repeats_for_a_seed() {
        let render = |name: &str, seed: u64| {
            let mut maze = Maze::new(12, 8, true, false, false, Some(seed));
            maze.reduce_nets(1);
            maze.remove_cycles(false);
            maze.set_ink(true);
            let path = std::env::temp_dir().join(name);
            maze.render(path.to_string_lossy().into_owned(), SVG)
                .expect("render failed");
            let svg_path = path.with_extension("svg");
            let svg = std::fs::read_to_string(&svg_path).expect("missing SVG");
            let _ = std::fs::remove_file(svg_path);
            svg
        };
        let svg = render("lubra-ink-a", 3);
        // No fragment is reused: there is nothing in the defs and nothing placed.
        assert!(!svg.contains("<use"), "tracks were placed, not inked");
        assert!(svg.contains("stroke='#2b2118'"));
        assert_eq!(svg, render("lubra-ink-b", 3));
        assert_ne!(svg, render("lubra-ink-c", 4));
    }

    use common::types::point::TILE_HEIGHT;

    /// A neighbour's cross-tile fragment must never take over the entrance or exit
    /// dead end, which would leave it drawn as plain track with no dot.
    #[test]
    fn test_entrance_and_exit_are_always_drawn_as_markers() {
        for (columns, rows) in [(25, 7), (28, 6), (10, 6), (15, 16), (29, 17)] {
            for seed in 0..20 {
                let maze = Maze::generate(columns, rows, seed);
                let markers: Vec<(usize, usize)> = maze
                    .drawn_fragments(false)
                    .iter()
                    .filter(|fragment| fragment.marker)
                    .map(|fragment| (fragment.coordinate.i, fragment.coordinate.j))
                    .collect();
                assert_eq!(
                    markers,
                    [(0, 0), (columns - 1, rows - 1)],
                    "{columns}x{rows} seed {seed}"
                );
            }
        }
    }

    #[test]
    fn test_solution_runs_from_the_entrance_dot_to_the_exit_dot() {
        let mut maze = Maze::new(20, 12, true, false, false, Some(2));
        maze.reduce_nets(1);
        maze.remove_cycles(false);
        maze.trim_dead_ends(2);
        let drawn = maze.drawn_fragments(false);
        let (over, under) = maze.solution_trace(&drawn);
        let lines: Vec<_> = over.iter().chain(&under).collect();
        assert!(!lines.is_empty());
        let points: Vec<(f64, f64)> = lines.iter().flat_map(|l| l.iter().copied()).collect();
        let near = |p: (f64, f64), q: (f64, f64)| (p.0 - q.0).hypot(p.1 - q.1) < 0.5;
        // The entrance and exit tiles both point up here; their middles are a third of
        // the way up.
        let entrance = (60.0, TILE_HEIGHT / 3.0);
        let exit = (19.0 * 60.0 + 60.0, 11.0 * TILE_HEIGHT + TILE_HEIGHT / 3.0);
        assert!(
            points.iter().any(|&p| near(p, entrance)),
            "no start at the entrance"
        );
        assert!(points.iter().any(|&p| near(p, exit)), "no end at the exit");
    }

    #[test]
    fn test_show_solution_adds_a_red_trace() {
        let mut maze = Maze::new(12, 8, true, false, false, Some(3));
        maze.reduce_nets(1);
        maze.remove_cycles(false);
        maze.set_show_solution(true);
        let path = std::env::temp_dir().join("lubra-solution");
        maze.render(path.to_string_lossy().into_owned(), SVG)
            .expect("render failed");
        let svg_path = path.with_extension("svg");
        let svg = std::fs::read_to_string(&svg_path).expect("missing SVG");
        let _ = std::fs::remove_file(svg_path);
        assert!(svg.contains("stroke='#c0392b'"));
    }
}

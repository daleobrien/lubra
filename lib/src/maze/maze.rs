use crate::maze::net::Net;
use crate::maze::tile::Tile;
use array2d::Array2D;

// Import the implementation modules
mod dead_ends;
mod draw_order;
mod fill;
mod fill_clearance;
mod grid_generation;
mod net_operations;
mod net_reduction;
mod remove_cycles;
mod render;
mod solution;
pub use solution::RouteStep;
mod surrounding_coordinates;
mod tile_lookup;
pub(crate) mod touching;

#[derive(Eq, PartialEq)]
pub enum FileType {
    SVG,
    PNG,
}

pub struct Maze {
    pub style: Option<String>,
    pub(crate) nets: Net,
    width: f64,
    height: f64,
    pub(crate) seed: Option<u64>,
    pub(crate) number_of_columns: usize,
    pub(crate) number_of_rows: usize,
    pub(crate) tiles: Array2D<Tile>,
    pub(crate) boundaries: bool,
    pub(crate) show_node_numbers: bool,
    pub(crate) show_tile_nets: bool,
    pub(crate) caption: Option<String>,
    pub(crate) colour_tracks: bool,
    pub(crate) show_kinks: bool,
    pub(crate) ink: bool,
    pub(crate) show_solution: bool,
    /// Whether a track passing under another shows faintly through it.
    pub(crate) faint_under: bool,
    /// Whether tracks may cross, one passing under another.
    pub(crate) crossings: bool,
    /// How many crossings to make (see `CROSSING_LEVEL`).
    pub(crate) crossing_level: f64,
}

/// How many crossings a maze has, one track passing under another.
///
/// From 0 to 1 it is the share of crossings kept when touching tracks are separated
/// (`separate_touching_tracks`): at 0 a crossing goes whenever one of its tracks
/// touches another, which leaves very few; a kept crossing gives up its track only
/// when nothing else clears a touch. Above 1, every crossing is kept and tiles with
/// tracks crossing are also picked that many times as often when the grid is made.
///
/// Over 20 mazes of 24 by 12 tiles (0.12 wide tracks), the crossings per 100 tiles
/// (and touching pairs left over the 20): 0.3 at level 0 (137), 2 at 0.5 (123), 4 at
/// 1 (105), 7 at 2 (79), 10 at 4 (48) and 13.5 at 10 (33).
pub const CROSSING_LEVEL: f64 = 4.0;

/// The most times `Maze::generate_with` makes a maze again to be rid of a crossing.
const MAX_ATTEMPTS: u64 = 20;

impl Maze {
    pub fn new(
        number_of_columns: usize,
        number_of_rows: usize,
        randomise: bool,
        boundaries: bool,
        show_node_numbers: bool,
        seed: Option<u64>,
    ) -> Self {
        // User Maze object somehow

        let nets = Vec::new();

        let tiles: Array2D<Tile> =
            Array2D::filled_with(Tile::new(vec![], false), number_of_rows, number_of_columns);

        let mut obj = Self {
            style: None,
            width: 0f64,
            height: 0f64,
            nets,
            seed,
            number_of_columns,
            number_of_rows,
            tiles,
            boundaries,
            show_node_numbers,
            show_tile_nets: false,
            caption: None,
            colour_tracks: false,
            show_kinks: false,
            ink: false,
            show_solution: false,
            faint_under: true,
            crossings: true,
            crossing_level: CROSSING_LEVEL,
        };

        if randomise {
            Self::create_random_grid(&mut obj, number_of_columns, number_of_rows);
        }
        obj
    }

    /// How many crossings to make, from 0 (very few) up: see `CROSSING_LEVEL`, the
    /// default. Takes effect when the grid is made and tracks are separated.
    pub fn set_crossing_level(&mut self, level: f64) {
        self.crossing_level = level;
    }

    /// How much more often a tile with tracks crossing is picked: 1 up to level 1,
    /// then the level.
    pub(crate) fn crossing_weight(&self) -> f64 {
        self.crossing_level.max(1.0)
    }

    /// Whether to draw the route from the entrance to the exit, in red.
    pub fn set_show_solution(&mut self, show: bool) {
        self.show_solution = show;
    }

    /// Whether a track passing under another shows faintly through the one on top (on
    /// by default). Off, the hidden walls are simply not drawn.
    pub fn set_faint_under(&mut self, faint: bool) {
        self.faint_under = faint;
    }

    /// Whether to draw the maze by hand, in ink (see `renders::ink`).
    pub fn set_ink(&mut self, ink: bool) {
        self.ink = ink;
    }

    /// Whether to ring every kink in the rendered maze (see `renders::kinks`) and add
    /// their number to the caption.
    pub fn set_show_kinks(&mut self, show: bool) {
        self.show_kinks = show;
    }

    /// Whether to write each tile's nets (e.g. `12,5`) in blue in the rendered maze.
    pub fn set_show_tile_nets(&mut self, show: bool) {
        self.show_tile_nets = show;
    }

    /// A line of text drawn above the rendered maze, or `None` for no caption.
    pub fn set_caption(&mut self, caption: Option<String>) {
        self.caption = caption;
    }

    /// Whether to draw each net's track walls in its own colour, for debugging; the
    /// tile net labels then use the same colours.
    pub fn set_colour_tracks(&mut self, colour: bool) {
        self.colour_tracks = colour;
    }

    /// A finished maze of `columns` by `rows` tiles from `seed`, made the way the CLI
    /// makes one by default: nets reduced, loops removed, dead ends of up to two tiles
    /// trimmed, touching tracks separated and blank areas filled with side branches of
    /// two tiles or more.
    pub fn generate(columns: usize, rows: usize, seed: u64) -> Maze {
        Maze::generate_with(columns, rows, seed, true)
    }

    /// As `generate`, with or without `crossings`: without them no track passes
    /// under another, in a tile or where a gap is filled.
    ///
    /// Without crossings, the rare maze that still ends up with one (where every way
    /// to cut a loop crosses) is made again from a seed derived from `seed`, so the
    /// same seed still always gives the same maze.
    pub fn generate_with(columns: usize, rows: usize, seed: u64, crossings: bool) -> Maze {
        let build = |seed: u64| {
            let mut maze = Maze::new(columns, rows, false, false, false, Some(seed));
            maze.crossings = crossings;
            maze.create_random_grid(columns, rows);
            maze.reduce_nets(1);
            maze.remove_cycles(false);
            maze.trim_dead_ends(2);
            maze.separate_touching_tracks();
            maze.fill_blank_areas(2);
            maze
        };
        let mut maze = build(seed);
        let mut attempt = 0;
        while !crossings && maze.crossing_count() > 0 && attempt < MAX_ATTEMPTS {
            attempt += 1;
            maze = build(seed.wrapping_add(attempt.wrapping_mul(0x9E37_79B9_7F4A_7C15)));
        }
        maze
    }

    /// The number of columns of tiles.
    pub fn columns(&self) -> usize {
        self.number_of_columns
    }

    /// The number of rows of tiles.
    pub fn rows(&self) -> usize {
        self.number_of_rows
    }

    /// The tile nets of tile `(i, j)`: each is the logical nodes (bits 0-5) it joins.
    pub fn tile_nets(&self, i: usize, j: usize) -> &[usize] {
        &self.tiles[(j, i)].nodes
    }
}

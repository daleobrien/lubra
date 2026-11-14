use crate::maze::net::Net;
use crate::maze::tile::Tile;
use array2d::Array2D;

// Import the implementation modules
mod dead_ends;
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
}

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
        };

        if randomise {
            Self::create_random_grid(&mut obj, number_of_columns, number_of_rows);
        }
        obj
    }

    /// Whether to draw the route from the entrance to the exit, in red.
    pub fn set_show_solution(&mut self, show: bool) {
        self.show_solution = show;
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
        let mut maze = Maze::new(columns, rows, true, false, false, Some(seed));
        maze.reduce_nets(1);
        maze.remove_cycles(false);
        maze.trim_dead_ends(2);
        maze.separate_touching_tracks();
        maze.fill_blank_areas(2);
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

use crate::maze::net::Net;
use crate::maze::tile::Tile;
use array2d::Array2D;

// Import the implementation modules
mod grid_generation;
mod net_operations;
mod net_reduction;
mod remove_cycles;
mod render;
mod surrounding_coordinates;
mod tile_lookup;

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
        };

        if randomise {
            Self::create_random_grid(&mut obj, number_of_columns, number_of_rows);
        }
        obj
    }
}

/// Runs `test` on a thread with a 64 MiB stack.
///
/// Building a random maze in a debug build needs more than the 2 MiB stack a test
/// thread gets (the generated lookup functions have very large frames), so tests
/// that call `Maze::new(.., randomise: true, ..)` run through this.
#[cfg(test)]
pub(crate) fn with_large_stack<F: FnOnce() + Send + 'static>(test: F) {
    std::thread::Builder::new()
        .stack_size(64 << 20)
        .spawn(test)
        .expect("failed to spawn test thread")
        .join()
        .expect("test thread panicked");
}

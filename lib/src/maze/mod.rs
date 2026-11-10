pub mod adjacent;
pub mod generated;
// `maze::maze::Maze` is the established public path used by the CLI and document crates.
#[allow(clippy::module_inception)]
pub mod maze;
pub mod net;
pub mod random;
pub mod tile;

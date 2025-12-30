use crate::maze::maze::Maze;
use crate::maze::tile::Tile;
use common::consts::N5;
use common::types::point::{TILE_HEIGHT, TILE_WIDTH};
use rand::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand_chacha::ChaCha8Rng;

/// Grid generation methods for the Maze struct.
/// These methods handle random grid initialization and tile placement.
impl Maze {
    pub fn create_random_grid(&mut self, number_of_columns: usize, number_of_rows: usize) {
        // Seed some random tiles that don't touch each other or the boundaries

        let mut rng = match self.seed {
            Some(s) => ChaCha8Rng::seed_from_u64(s),
            None => ChaCha8Rng::from_rng(thread_rng()).unwrap(),
        };

       // Make the first Tile as a simple starting point
       // TODO: Determine orientations of first tile
        let tile = Tile::new(vec![N5], true);
        self.tiles[(0, 0)] = tile;
       // TODO: Determine which node to use as the last tile (i,j) => ?

        let mut indices: Vec<(usize, usize)> = (0..number_of_rows)
            .flat_map(|j| (0..number_of_columns).map(move |i| (j, i)))
            .collect();

        // Add the tiles in random order, otherwise the prevoius tile will influence the next
        indices.shuffle(&mut rng);

        for (j, i) in indices {
            // Don't need to create twice
            if self.tiles[(j, i)].initialised {
                continue;
            }

          self.tiles[(j, i)] = self.get_random_tile(i, j);

            self.seed = match self.seed {
                Some(s) => Some(s.wrapping_add(1)),
                _ => None,
            };
        }

        self.width = (number_of_columns + 1) as f64 * TILE_WIDTH / 2.0;
        self.height = number_of_rows as f64 * TILE_HEIGHT;
    }
}

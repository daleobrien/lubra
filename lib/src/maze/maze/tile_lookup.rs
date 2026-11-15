use crate::maze::adjacent::get_surrounding_tile_connections;
use crate::maze::maze::Maze;
use crate::maze::random::{random_tile, random_tile_without_crossings};
use crate::maze::tile::Tile;
use common::consts::NODE_TO_COORD;
use common::types::coordinate::add;

/// Tile lookup and positioning methods for the Maze struct.
/// These methods handle getting tiles by position and generating random tiles.
impl Maze {
    pub(crate) fn get_tile_relative_position(
        &self,
        pos: (usize, usize),
        rel: (i32, i32),
    ) -> Option<&Tile> {
        // Out of bounds, Array2D will return None if out of bounds.

        let x = add(pos.0, rel.0);
        let y = add(pos.1, rel.1);

        let result = match (x, y) {
            (Some(x), Some(y)) => self.tiles.get(y, x),
            _ => None, // Out of bounds, not possible tile
        };
        result
    }

    pub(crate) fn get_connected_titles_for_tile(&self, i: usize, j: usize) -> [Option<&Tile>; 12] {
        let k = i + (3 * (j % 2));
        /*
        We only have six different ways a tile can be rotated/flipped
        For each rotation/flip, we can then get the adjacent tile
        for each node index within the tile at {i,j}.

        So each k % 6 matches a rotation of the tile
        Additionally, each row is indexed:  1,2,3,4,5,6,1l,1r,2l,2r,3l,3r

        k % 6 ->
        {
         {0, 1, 2, 3, 4, 5},
         {3, 4, 5, 0, 1, 2},
         {0, 1, 2, 3, 4, 5},
         {3, 4, 5, 0, 1, 2},
         {0, 1, 2, 3, 4, 5},
         {3, 4, 5, 0, 1, 2}
        }

        This means that no matter what rotation the tile is in, we can always get the adjacent tile
        for an given node index without having to do any math,
        e.g.
           result[7] will always return the tile that is adjacent to node 6.
        */

        NODE_TO_COORD[k % 6].map(|coord| self.get_tile_relative_position((i, j), coord))
    }

    pub(crate) fn get_random_tile(&mut self, i: usize, j: usize) -> Tile {
        let surrounding_tiles = self.get_connected_titles_for_tile(i, j);
        let mask = get_surrounding_tile_connections(&surrounding_tiles);

        let pick = if self.crossings {
            random_tile
        } else {
            random_tile_without_crossings
        };
        // Try simple, then complex options
        let mut nets = pick(mask, self.seed, true);
        if mask != 0 && nets.is_empty() {
            // If we can't find a tile that fits, try again without the mask
            nets = pick(mask, self.seed, false);
        }

        Tile::new(nets, true)
    }
}

#[cfg(test)]
mod tests {
    use crate::maze::maze::Maze;

    #[test]
    fn test_surrounding_for_island() {
        let maze = Maze::new(1, 1, false, false, false, None);

        let surrounding_tiles = maze.get_connected_titles_for_tile(0, 0);

        let result: Vec<usize> = surrounding_tiles
            .iter()
            .map(|t| if t.is_some() { 1 } else { 0 })
            .collect::<Vec<usize>>();
        assert_eq!(result, vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_surrounding_for_x_0() {
        let maze = Maze::new(9, 9, false, false, false, None);

        let expected = [
            vec![1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0],
            vec![0, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 0],
            vec![0, 1, 0, 1, 0, 1, 1, 0, 1, 1, 0, 1],
            vec![0, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1],
            vec![0, 0, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1],
            vec![1, 0, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1],
        ];
        for (i, expected_row) in expected.iter().enumerate() {
            let surrounding_tiles = maze.get_connected_titles_for_tile(i, 0);

            let result: Vec<usize> = surrounding_tiles
                .iter()
                .map(|t| if t.is_some() { 1 } else { 0 })
                .collect::<Vec<usize>>();
            assert_eq!(&result, expected_row);
        }
    }
}

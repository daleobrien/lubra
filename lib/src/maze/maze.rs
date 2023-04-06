use std::cmp::max;
use std::collections::HashSet;
use std::error::Error;
use std::fs;

use array2d::Array2D;

use crate::maze::tile::{add, Tile};
use crate::renders::glyph::Glyph;
use crate::renders::primatives::Coordinate;
use crate::renders::strokes::{H, M};

use crate::renders::lookup::{find_tiles, random_side};
use crate::renders::svg::Path;

pub struct Maze {
    pub style: Option<String>,
    tiles: Vec<Vec<Glyph>>,
    walls: Vec<Vec<Glyph>>,
    nets: HashSet<Vec<[u32; 3]>>,
    last_free_net_id: u32,
    width: f64,
    height: f64,
}

/*

Problem,

Glyph has nets, strokes and so on,
while Tile just has a bit mask !


*/

impl Maze {
    pub fn new(number_of_columns: usize, number_of_rows: usize, randomise: bool) -> Self {
        // User Maze object somehow

        let tiles: Vec<Vec<Glyph>> = Vec::new();
        let walls: Vec<Vec<Glyph>> = Vec::new();
        let nets = HashSet::new();

        let mut obj = Self {
            style: None,
            tiles,
            walls,
            width: 0f64,
            height: 0f64,
            nets,
            last_free_net_id: 0u32,
        };

        if randomise {
            Self::create_random_grid(&mut obj, number_of_columns, number_of_rows);
        }
        return obj;
    }
    pub fn create_random_grid(&mut self, number_of_columns: usize, number_of_rows: usize) {
        let mut tiles = Array2D::filled_with(Tile::Z2, number_of_rows, number_of_columns);
        for j in 0..number_of_rows {
            let mut row_path: Vec<Glyph> = Vec::new();
            let mut row_wall: Vec<Glyph> = Vec::new();

            for i in 0..number_of_columns {
                let coord: Coordinate = Coordinate { i, j };

                let tile = self.get_random_tile(i, j, &tiles);

                tiles[(j, i)] = tile;

                let path: Glyph = Glyph::new(coord, tile, self.last_free_net_id);

                // ..
                //let this_net = tile.get_nets();

                // self.last_free_net_id = max(path.max(), self.last_free_net_id);

                // TODO: updated self.nets too, so I can find which tiles have a given net id.
                // Where to I move the net list too, maybe it should go with then maze itself
                // nets[net_id] = [{i,j,1}, {i,j,2}, ... ]
                //

                // println!(
                //     ":->  ({i},{j}) -> {} {:?}",
                //     self.last_free_net_id, path.nets
                // );

                row_path.push(path);

                let wall: Glyph = Glyph::new(coord, Tile::W1, 0);

                row_wall.push(wall);
            }
            self.tiles.push(row_path);
            self.walls.push(row_wall);
        }

        self.width = (number_of_columns + 1) as f64 * M;
        self.height = number_of_rows as f64 * H;
    }

    pub fn add_row(&mut self, row: Vec<Glyph>) {
        self.tiles.push(row);

        // Get the max number of rows within any one Row
        // NOTE: This assumes all items are continous from 0 to n with now skipped cells
        let number_of_columns = self
            .tiles
            .iter()
            .map(|row| row.len())
            .max()
            .unwrap_or_default();

        let number_of_rows = self.tiles.len();

        self.width = (number_of_columns + 1) as f64 * M;
        self.height = number_of_rows as f64 * H;
    }

    /*

    net_1 = [[i,j,k], [i,j,k], <n>] .. these are sets of tiles that are connected.
    net_2 = [[i,j,k], [i,j,k], <m>]

    Take the shortest net, and iterate over it looking for the first ajacent tile that is not part of the net.
    Then replace the matching tile such that the nets are connected and move the short net over to the larger net.

    Repeat until we only have one net.
     */

    // TODO:Added debug only method to create the grid outlines, aka the walls
    pub fn smoosh(&mut self) {
        for row in self.tiles.iter() {
            for tile in row {
                // TODO: get neighbours and so on ...
                //println!("{:?}", tile.nets);
            }
        }
    }

    pub fn create_one_tile_grid(&mut self) {
        let mut row_path: Vec<Glyph> = Vec::new();
        let mut row_wall: Vec<Glyph> = Vec::new();

        for i in [0, 3, 6, 9, 12, 15] {
            for j in [0, 2, 4, 6, 8, 10] {
                row_path.push(Glyph::new(Coordinate { i, j }, Tile::C3, 0));
                row_wall.push(Glyph::new(Coordinate { i, j }, Tile::W1, 0));
            }
        }

        self.tiles.push(row_path);
        self.walls.push(row_wall);

        self.width = 17f64 * M;
        self.height = 12f64 * H;
    }

    fn get_tile_relative_position(
        &self,
        pos: (usize, usize),
        rel: (i32, i32),
        tiles: &Array2D<Tile>,
    ) -> Tile {
        if pos.1 as i32 + rel.1 < 0 || pos.0 as i32 + rel.0 < 0 {
            return Tile::Z1;
        }

        let x: usize = add(pos.0, rel.0);
        let y: usize = add(pos.1, rel.1);

        let tile = tiles.get(y, x);
        return match tile {
            Some(t) => *t,
            None => Tile::Z1,
        };
    }

    fn calc_vertex(
        &self,
        gathered_tiles: [[Tile; 5]; 3],
        tile: Tile,
        pos: usize, // 0..2
        is_top: bool,
    ) -> (bool, bool) {
        /*
                             ____ ____ ____
                                 /\   /\
               this row -> .    /  \ /  \
                                ---- ----

        The other row depends on where the tile is pointing up or down
        */

        let this_row = gathered_tiles[1];

        let other_row = if is_top {
            gathered_tiles[2]
        } else {
            gathered_tiles[0]
        };

        let opposite_tile = other_row[4 - 2 * pos]; // index 4, 2, 0

        if opposite_tile.contains(Tile::Z2) {
            // is not routed
            if pos == 0
                && !this_row[3].contains(tile)
                && !this_row[4].contains(tile)
                && !other_row[2].contains(tile)
                && !other_row[3].contains(tile)
            {
                return (false, true);
            }
            if pos == 1
                && !this_row[1].contains(tile)
                && !this_row[3].contains(tile)
                && !other_row[1].contains(tile)
                && !other_row[3].contains(tile)
            {
                return (false, true);
            }
            if pos == 2
                && !this_row[0].contains(tile)
                && !this_row[1].contains(tile)
                && !other_row[1].contains(tile)
                && !other_row[2].contains(tile)
            {
                return (false, true);
            }
        } else if opposite_tile.contains(tile) {
            // Must be 1
            return (true, false);
        }
        // Must be 0
        (false, false)
    }

    fn calc_side(
        &self,
        gathered_tiles: [[Tile; 5]; 3],
        tile: Tile,
        pos: usize,
        full_pos: usize,
    ) -> (bool, bool) {
        let j = if pos == 1 {
            if full_pos % 2 == 1 {
                0
            } else {
                2
            }
        } else {
            1
        };

        let adjacent = gathered_tiles[j][pos + 1];

        if adjacent.contains(Tile::Z2) {
            // Don't care
            return (false, true);
        }
        if adjacent.contains(tile) {
            // Must match, on a one
            return (true, false);
        }
        // Must match, on a zero
        (false, false)
    }

    fn get_surrounding_tiles(&self, i: usize, j: usize, tiles: &Array2D<Tile>) -> [[Tile; 5]; 3] {
        let gathered_tiles = [
            [
                self.get_tile_relative_position((i, j), (-2, -1), tiles),
                self.get_tile_relative_position((i, j), (-1, -1), tiles),
                self.get_tile_relative_position((i, j), (0, -1), tiles),
                self.get_tile_relative_position((i, j), (1, -1), tiles),
                self.get_tile_relative_position((i, j), (2, -1), tiles),
            ],
            [
                self.get_tile_relative_position((i, j), (-2, 0), tiles),
                self.get_tile_relative_position((i, j), (-1, 0), tiles),
                Tile::Z2,
                self.get_tile_relative_position((i, j), (1, 0), tiles),
                self.get_tile_relative_position((i, j), (2, 0), tiles),
            ],
            [
                self.get_tile_relative_position((i, j), (-2, 1), tiles),
                self.get_tile_relative_position((i, j), (-1, 1), tiles),
                self.get_tile_relative_position((i, j), (0, 1), tiles),
                self.get_tile_relative_position((i, j), (1, 1), tiles),
                self.get_tile_relative_position((i, j), (2, 1), tiles),
            ],
        ];
        return gathered_tiles;
    }

    pub fn get_surrounding_tile_connections(
        &self,
        i: usize,
        j: usize,
        tiles: &Array2D<Tile>,
    ) -> (u32, u32) {
        let surrounding_tiles = self.get_surrounding_tiles(i, j, tiles);

        let mut dont_care = 0u32;
        let mut matches = 0u32;

        let pos1 = (4 + i + (3 * (j % 2))) % 6;
        let pos2 = (2 + i + (3 * (j % 2))) % 6;
        let pos3 = (0 + i + (3 * (j % 2))) % 6; // Alternates on each row , 1st row 0,1,2,3,4,5,0, ... 2nd row, 3,4,5,0,1,2,3,...

        // Node 1
        let (m, d) = self.calc_vertex(surrounding_tiles, Tile::G1, (1 + pos3) % 3, pos1 < 3);
        if m {
            matches |= 0b000_001;
        }
        if d {
            dont_care |= 0b000_001;
        }

        // Node 2
        let (m, d) = self.calc_vertex(surrounding_tiles, Tile::G2, (2 + pos3) % 3, pos2 < 3);
        if m {
            matches |= 0b000_010;
        }
        if d {
            dont_care |= 0b000_010;
        }

        // Node3
        let (m, d) = self.calc_vertex(surrounding_tiles, Tile::G3, (3 + pos3) % 3, pos3 < 3);
        if m {
            matches |= 0b000_100;
        }
        if d {
            dont_care |= 0b000_100;
        }

        let (m, d) = self.calc_side(surrounding_tiles, Tile::G4, (3 + pos3) % 3, pos3);
        if m {
            matches |= 0b001_000;
        }
        if d {
            dont_care |= 0b001_000;
        }

        let (m, d) = self.calc_side(surrounding_tiles, Tile::G5, (3 + pos2) % 3, pos3);
        if m {
            matches |= 0b010_000;
        }
        if d {
            dont_care |= 0b010_000;
        }

        let (m, d) = self.calc_side(surrounding_tiles, Tile::G6, (3 + pos1) % 3, pos3);
        if m {
            matches |= 0b100_000;
        }
        if d {
            dont_care |= 0b100_000;
        }
        return (dont_care, matches);
    }

    fn get_random_tile(&self, i: usize, j: usize, tiles: &Array2D<Tile>) -> Tile {
        if i == 0 && j == 0 {
            return Tile::L3;
        }

        let (dont_care, matches) = self.get_surrounding_tile_connections(i, j, tiles);

        // TODO: strip out cells that only have Nx and Tx
        for _ in 0..45 {
            let first_tile = random_side(matches, dont_care);
            if !first_tile.contains(Tile::N1)
                && !first_tile.contains(Tile::N2)
                && !first_tile.contains(Tile::N3)
            {
                return first_tile;
            }
        }
        return random_side(matches, dont_care);
    }

    pub fn create_all_title_in_grid(&mut self) {
        let possible_tiles = find_tiles(0b101010101010);

        let mut row_path: Vec<Glyph> = Vec::new();
        let mut row_wall: Vec<Glyph> = Vec::new();

        let i = 0;
        let mut j: usize = 0;

        //println!("");
        for tile in possible_tiles.iter() {
            //println!("{j} {i} {:?}", Tile::from_bits_truncate(*tile));

            let coord1: Coordinate = Coordinate { i: i * 2, j: j * 2 };
            let coord2: Coordinate = Coordinate {
                i: i * 2 + 3,
                j: j * 2,
            };

            let path1: Glyph = Glyph::new(coord1, Tile::from_bits_truncate(*tile), 0);
            let wall1: Glyph = Glyph::new(coord1, Tile::W1, 0);

            let path2: Glyph = Glyph::new(coord2, Tile::from_bits_truncate(*tile), 0);
            let wall2: Glyph = Glyph::new(coord2, Tile::W1, 0);

            row_path.push(path1);
            row_wall.push(wall1);
            row_path.push(path2);
            row_wall.push(wall2);

            let coord1: Coordinate = Coordinate {
                i: i * 2 + 1,
                j: j * 2 + 1,
            };
            let coord2: Coordinate = Coordinate {
                i: i * 2 + 3 + 1,
                j: j * 2 + 1,
            };

            let path1: Glyph = Glyph::new(coord1, Tile::from_bits_truncate(*tile), 0);
            let wall1: Glyph = Glyph::new(coord1, Tile::W1, 0);

            let path2: Glyph = Glyph::new(coord2, Tile::from_bits_truncate(*tile), 0);
            let wall2: Glyph = Glyph::new(coord2, Tile::W1, 0);

            row_path.push(path1);
            row_wall.push(wall1);
            row_path.push(path2);
            row_wall.push(wall2);

            j += 2;
        }
        self.tiles.push(row_path);
        self.walls.push(row_wall);

        self.width = (10 * 1) as f64 * (M + 50.0);
        self.height = (possible_tiles.len() * 3) as f64 * (H + 00.0);
    }

    // Path commands
    // https://github.com/bodoni/svg/blob/master/src/node/element/path/data.rs
    pub fn render(self, file_name: String) -> Result<(), Box<dyn Error>> {
        let mut svg = String::new();

        let margin = 5.0;
        let w = self.width;
        let h = self.height;

        let view_box = format!(
            "viewBox=\"-{m} -{m} {w} {h}\" ",
            w = w + 2.0 * margin,
            h = h + 2.0 * margin,
            m = margin
        );

        svg.push_str(&format!(
            "<svg id=\"maze\" xmlns=\"http://www.w3.org/2000/svg\" {view_box} width=\"{w}px\" height=\"{h}px\" fill-opacity=\"0\" stroke-linecap=\"round\" stroke=\"black\" >",
            w = w + 2.0 * margin, h = h + 2.0 * margin, view_box = view_box));

        let mut path = Path::new("black".to_string(), 3.0, 1.0);
        let mut faint_path = Path::new("gray".to_string(), 3.0, 0.2);
        let mut wall = Path::new("brown".to_string(), 3.0, 0.3);

        for row in &self.tiles {
            for tile in row {
                tile.add_to_path(&mut path, &mut faint_path);
            }
        }
        for row in &self.walls {
            for tile in row {
                tile.add_to_path(&mut wall, &mut faint_path)
            }
        }

        svg.push_str(&faint_path.to_string());
        svg.push_str(&path.to_string());
        svg.push_str(&wall.to_string());
        svg.push_str("</svg>");

        fs::write(file_name, svg.as_bytes())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::maze::maze::Maze;
    use crate::maze::tile::Tile;
    use crate::renders::glyph::Glyph;
    use crate::renders::primatives::Coordinate;

    #[test]
    fn very_small_net() {
        let mut maze = Maze::new(2, 1, false);
        let mut row_path_1: Vec<Glyph> = Vec::new();
        let mut row_path_2: Vec<Glyph> = Vec::new();

        let title1 = Glyph::new(Coordinate { i: 0, j: 0 }, Tile::C2, 0);
        let title2 = Glyph::new(Coordinate { i: 1, j: 0 }, Tile::L1, 0);
        row_path_1.push(title1);
        row_path_1.push(title2);

        let title3 = Glyph::new(Coordinate { i: 0, j: 1 }, Tile::C2, 0);
        let title4 = Glyph::new(Coordinate { i: 1, j: 1 }, Tile::L1, 0);
        row_path_2.push(title3);
        row_path_2.push(title4);

        maze.add_row(row_path_1);
        maze.add_row(row_path_2);

        maze.smoosh();

        let result_f = maze.render("../test-smoosh.svg".to_string());

        println!("Finished {:?}", result_f.unwrap());
    }
}

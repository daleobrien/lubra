use std::collections::HashSet;
use std::error::Error;
use std::fs;

use crate::maze::net::calculate_nets;

use array2d::Array2D;
use rand_distr::num_traits::ToPrimitive;

use crate::maze::tile::{add, Tile};
use crate::renders::glyph::Glyph;
use crate::renders::lookup::{find_tiles, random_side};
use crate::renders::primatives::Coordinate;
use crate::renders::strokes::{H, M};
use crate::renders::svg::Path;

pub struct Maze {
    pub style: Option<String>,
    glyphs: Vec<Vec<Glyph>>,
    walls: Vec<Vec<Glyph>>,
    nets: HashSet<Vec<[usize; 3]>>,
    width: f64,
    height: f64,
}

impl Maze {
    pub fn new(number_of_columns: usize, number_of_rows: usize, randomise: bool) -> Self {
        // User Maze object somehow

        let glyphs: Vec<Vec<Glyph>> = Vec::new();
        let walls: Vec<Vec<Glyph>> = Vec::new();
        let nets = HashSet::new();

        let mut obj = Self {
            style: None,
            glyphs,
            walls,
            width: 0f64,
            height: 0f64,
            nets,
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

                let path: Glyph = Glyph::new(coord, tile);

                row_path.push(path);

                let wall: Glyph = Glyph::new(coord, Tile::W1);

                row_wall.push(wall);
            }
            self.glyphs.push(row_path);
            self.walls.push(row_wall);
        }

        self.width = (number_of_columns + 1) as f64 * M;
        self.height = number_of_rows as f64 * H;
    }

    pub fn add_row(&mut self, row: Vec<Glyph>) {
        self.glyphs.push(row);

        // Get the max number of rows within any one Row
        // NOTE: This assumes all items are continuous from 0 to n with now skipped cells
        let number_of_columns = self
            .glyphs
            .iter()
            .map(|row| row.len())
            .max()
            .unwrap_or_default();

        let number_of_rows = self.glyphs.len();

        self.width = (number_of_columns + 1) as f64 * M;
        self.height = number_of_rows as f64 * H;
    }

    /*

        net_1 = [[i,j,k], [i,j,k], <n>] .. these are sets of tiles that are connected.
        net_2 = [[i,j,k], [i,j,k], <m>]

        Take the shortest net, and iterate over it looking for the first adjacent tile that is not part of the net.
        Then replace the matching tile such that the nets are connected and move the short net over to the larger net.

        Repeat until we only have one net.
    F
         The surrounding_tiles are of the form:

                1
               ╱ ╲
              4   5
             ╱     ╲
            2 ━ 6 ━ 3

           ▽△▽△▽      △▽△▽△
           △▽▲▽△  or  ▽△▼△▽
           ▽△▽△▽      △▽△▽△

        */
    fn get_adjacent_tile(
        &self,
        surrounding_tiles: [[Tile; 5]; 3],
        i: usize,
        j: usize,
        k: usize, // the node within a tile
    ) -> Tile {
        let z = i + (3 * (j % 2));

        /*

         🜂🜄🜂🜄🜂  <- 1st row
         🜄🜂🜄🜂🜄  <- 2nd row
         🜂🜄🜂🜄🜂  <- 3rd row

         Thus the tile points if the position is (i+j) % 2 == 0

        */

        match k {
            1 => {
                // Find the triange vertex the lines up with the top vertex
                let pos = (4 + z) % 6 < 3;
                let other_row = surrounding_tiles[if pos { 2 } else { 0 }];
                return self.adjacent_vertex(other_row, (1 + i) % 3);
            }
            2 => {
                // Find the triange vertex the lines up with the bottom left vertex
                let pos = (2 + z) % 6 < 3;
                let other_row = surrounding_tiles[if pos { 2 } else { 0 }];
                return self.adjacent_vertex(other_row, (2 + i) % 3);
            }
            3 => {
                // Find the triange vertex the lines up with the bottom right vertex
                let pos = (0 + z) % 6 < 3;
                let other_row = surrounding_tiles[if pos { 2 } else { 0 }];
                return self.adjacent_vertex(other_row, (0 + i) % 3);
            }
            4 => {
                // The tile to the left
                return surrounding_tiles[1][2];
            }
            5 => {
                // The tile to the right
                return surrounding_tiles[1][4];
            }
            6 => {
                let pos = (i + j) % 2;
                return surrounding_tiles[pos][3];
            }
            _ => return Tile::Z1,
        }
    }

    // TODO: Added debug only method to create the grid outlines, aka the walls
    pub fn smoosh(&mut self) {
        println!("Smooshing ...\n");
        let number_of_rows = self.glyphs.len();
        let number_of_columns = self.glyphs[0].len();

        // Convert to 2D array of tiles
        let mut tiles = Array2D::filled_with(Tile::Z2, number_of_rows, number_of_columns);
        for (j, row) in self.glyphs.iter().enumerate() {
            for (i, glyph) in row.iter().enumerate() {
                tiles[(j, i)] = glyph.tile;
            }
        }

        for j in 0..number_of_rows {
            for i in 0..number_of_columns {
                let tile = tiles[(j, i)];

                // And build up a list of matching nets ??? of the surrounding nets
                let surrounding_tiles = self.get_surrounding_tiles(i, j, &tiles);

                // Get the nets for the current tile
                let nets = calculate_nets(tile.bits());

                // Get tiles for surrounding tiles
                let adjancent_tile_1 = self.get_adjacent_tile(surrounding_tiles, i, j, 1);
                let adjancent_tile_2 = self.get_adjacent_tile(surrounding_tiles, i, j, 2);
                let adjancent_tile_3 = self.get_adjacent_tile(surrounding_tiles, i, j, 3);
                let adjancent_tile_4 = self.get_adjacent_tile(surrounding_tiles, i, j, 4);
                let adjancent_tile_5 = self.get_adjacent_tile(surrounding_tiles, i, j, 5);
                let adjancent_tile_6 = self.get_adjacent_tile(surrounding_tiles, i, j, 6);
                println!(
                    "\n<{i},{j}>: {:?} {:?} {:?} {:?} {:?} {:?}",
                    adjancent_tile_1,
                    adjancent_tile_2,
                    adjancent_tile_3,
                    adjancent_tile_4,
                    adjancent_tile_5,
                    adjancent_tile_6
                );

                for net in nets {
                    println!("({i},{j}): {:?}", net);
                    let key: [usize; 3] = [i, j, net[0].to_usize().unwrap_or_default()];

                    //
                    for existing_nets in self.nets.iter() {
                        if existing_nets.contains(&key) {
                            println!("Found {i},{j},{:}", net[0]);
                        }
                    }
                    // if !self.nets.contains(&key) {
                    //     println!("Adding {i},{j},{:}", net[0]);
                    //     self.nets.insert(key);
                    // }
                }
            }
        }

        println!("\nDone Smooshing.\n");
    }

    pub fn create_one_tile_grid(&mut self) {
        let mut row_path: Vec<Glyph> = Vec::new();
        let mut row_wall: Vec<Glyph> = Vec::new();

        for i in [0, 3, 6, 9, 12, 15] {
            for j in [0, 2, 4, 6, 8, 10] {
                row_path.push(Glyph::new(Coordinate { i, j }, Tile::C3));
                row_wall.push(Glyph::new(Coordinate { i, j }, Tile::W1));
            }
        }

        self.glyphs.push(row_path);
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
        // Out of bounds
        if pos.0 as i32 + rel.0 < 0 || pos.1 as i32 + rel.1 < 0 {
            return Tile::Z1;
        }

        let x: usize = add(pos.0, rel.0);
        let y: usize = add(pos.1, rel.1);

        return *tiles.get(y, x).unwrap_or(&Tile::Z1);
    }

    fn adjacent_vertex(
        &self,
        other_row: [Tile; 5],
        pos: usize, // 0..2
    ) -> Tile {
        return other_row[4 - 2 * pos]; // index 4, 2, 0
    }

    fn calc_vertex(
        &self,
        gathered_tiles: [[Tile; 5]; 3],
        tile: Tile,
        pos: usize, // 0..2
        is_top: bool,
    ) -> (bool, bool) {
        /*
        The other row depends on where the tile is pointing up or down
        */
        let other_row = gathered_tiles[if is_top { 2 } else { 0 }];

        let opposite_tile = self.adjacent_vertex(other_row, pos);

        // is not routed
        if opposite_tile.contains(Tile::Z2) {
            let this_row = gathered_tiles[1];

            if pos == 0
                && !this_row[3].intersects(tile)
                && !this_row[4].intersects(tile)
                && !other_row[2].intersects(tile)
                && !other_row[3].intersects(tile)
            {
                return (false, true);
            }
            if pos == 1
                && !this_row[1].intersects(tile)
                && !this_row[3].intersects(tile)
                && !other_row[1].intersects(tile)
                && !other_row[3].intersects(tile)
            {
                return (false, true);
            }
            if pos == 2
                && !this_row[0].intersects(tile)
                && !this_row[1].intersects(tile)
                && !other_row[1].intersects(tile)
                && !other_row[2].intersects(tile)
            {
                return (false, true);
            }
        } else if opposite_tile.intersects(tile) {
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
        if adjacent.intersects(tile) {
            // Must match, on a one
            return (true, false);
        }
        // Must match, on a zero
        (false, false)
    }

    fn get_surrounding_tiles(&self, i: usize, j: usize, tiles: &Array2D<Tile>) -> [[Tile; 5]; 3] {
        // TODO: I can do better here, only get the tiles I need
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
        // Get the surrounding tiles
        let surrounding_tiles = self.get_surrounding_tiles(i, j, tiles);

        let mut dont_care = 0u32;
        let mut matches = 0u32;

        // Calculate the relative positions of the surrounding tiles
        let k = i + (3 * (j % 2));
        let pos = [(4 + k) % 6, (2 + k) % 6, (0 + k) % 6];

        // Node 1
        let (m, d) = self.calc_vertex(surrounding_tiles, Tile::G1, (1 + i) % 3, pos[0] < 3);

        if m {
            matches |= 0b000_001;
        } else if d {
            dont_care |= 0b000_001;
        }

        // Node 2
        let (m, d) = self.calc_vertex(surrounding_tiles, Tile::G2, (2 + i) % 3, pos[1] < 3);
        if m {
            matches |= 0b000_010;
        } else if d {
            dont_care |= 0b000_010;
        }

        // Node3
        let (m, d) = self.calc_vertex(surrounding_tiles, Tile::G3, (0 + i) % 3, pos[2] < 3);
        if m {
            matches |= 0b000_100;
        } else if d {
            dont_care |= 0b000_100;
        }

        let (m, d) = self.calc_side(surrounding_tiles, Tile::G4, (0 + i) % 3, pos[2]);
        if m {
            matches |= 0b001_000;
        } else if d {
            dont_care |= 0b001_000;
        }

        let (m, d) = self.calc_side(surrounding_tiles, Tile::G5, (2 + i) % 3, pos[2]);
        if m {
            matches |= 0b010_000;
        } else if d {
            dont_care |= 0b010_000;
        }

        let (m, d) = self.calc_side(surrounding_tiles, Tile::G6, (1 + i) % 3, pos[2]);
        if m {
            matches |= 0b100_000;
        } else if d {
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
            if !first_tile.intersects(Tile::N1)
                && !first_tile.intersects(Tile::N2)
                && !first_tile.intersects(Tile::N3)
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

            let path1: Glyph = Glyph::new(coord1, Tile::from_bits_truncate(*tile));
            let wall1: Glyph = Glyph::new(coord1, Tile::W1);

            let path2: Glyph = Glyph::new(coord2, Tile::from_bits_truncate(*tile));
            let wall2: Glyph = Glyph::new(coord2, Tile::W1);

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

            let path1: Glyph = Glyph::new(coord1, Tile::from_bits_truncate(*tile));
            let wall1: Glyph = Glyph::new(coord1, Tile::W1);

            let path2: Glyph = Glyph::new(coord2, Tile::from_bits_truncate(*tile));
            let wall2: Glyph = Glyph::new(coord2, Tile::W1);

            row_path.push(path1);
            row_wall.push(wall1);
            row_path.push(path2);
            row_wall.push(wall2);

            j += 2;
        }
        self.glyphs.push(row_path);
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
        let mut wall = Path::new("brown".to_string(), 3.0, 0.1);

        for row in &self.glyphs {
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

        let title1 = Glyph::new(Coordinate { i: 0, j: 0 }, Tile::C2);
        let title2 = Glyph::new(Coordinate { i: 1, j: 0 }, Tile::L1);
        row_path_1.push(title1);
        row_path_1.push(title2);

        let title3 = Glyph::new(Coordinate { i: 0, j: 1 }, Tile::C2);
        let title4 = Glyph::new(Coordinate { i: 1, j: 1 }, Tile::L1);
        row_path_2.push(title3);
        row_path_2.push(title4);

        maze.add_row(row_path_1);
        maze.add_row(row_path_2);

        maze.smoosh();

        let result_f = maze.render("../test-smoosh.svg".to_string());

        println!("Finished {:?}", result_f.unwrap());
    }
}

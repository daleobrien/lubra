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
    //number_of_columns: usize,
    //number_of_rows: usize,
    seed: Option<u64>,
}

impl Maze {
    pub fn new(
        number_of_columns: usize,
        number_of_rows: usize,
        randomise: bool,
        seed: Option<u64>,
    ) -> Self {
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
            //number_of_columns,
            //number_of_rows,
            seed,
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

                //println!("Surrounding tiles: {:?}", surrounding_tiles);

                let tile = self.get_random_tile(i, j, &tiles);

                // TODO: At this stage, only need to get the tiles that are above and or to the left
                // TODO: Have tile, add to network!
                // let nets = calculate_nets(tile.bits());
                // println!("Tile ({i},{j}) -> {:?}", nets);
                // for k in 1..=6 {
                //     self.update_network(surrounding_tiles, i, j, k, tile);
                // }

                tiles[(j, i)] = tile;

                let glyph: Glyph = Glyph::new(coord, tile);

                row_path.push(glyph);

                let glyph: Glyph = Glyph::new(coord, Tile::W1);

                row_wall.push(glyph);
            }
            self.glyphs.push(row_path);
            self.walls.push(row_wall);
        }

        self.width = (number_of_columns + 1) as f64 * M;
        self.height = number_of_rows as f64 * H;

        println!("\nNets are now:");
        for (n, nets) in self.nets.iter().enumerate() {
            println!("{n}-> {:?}", nets);
        }
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
    fn update_network(
        &self,
        surrounding_tiles: [[Tile; 5]; 3],
        i: usize,
        j: usize,
        k: usize,
        tile: Tile,
    ) {
        let z = i + (3 * (j % 2));

        /*

         🜂🜄🜂🜄🜂  <- 1st row
         🜄🜂🜄🜂🜄  <- 2nd row
         🜂🜄🜂🜄🜂  <- 3rd row

         Thus the tile points if the position is (i+j) % 2 == 0

        */

        let connected_tile = match k {
            1 => {
                // Find the triangle vertex the lines up with the top vertex
                // TODO: Abstrack this relative position out into a function,
                // see get_surrounding_tile_connections()
                let pos = (4 + z) % 6 < 3;
                let y = if pos { 2 } else { 0 };
                let x = (1 + i) % 3;
                let other_row = surrounding_tiles[y];
                print!("({}, {y}) ", 4 - 2 * x);
                self.adjacent_vertex(other_row, x).bits()
            }
            2 => {
                // Find the triangle vertex the lines up with the bottom left vertex
                let pos = (2 + z) % 6 < 3;
                let y = if pos { 2 } else { 0 };
                let x = (2 + i) % 3;
                let other_row = surrounding_tiles[y];
                print!("({}, {y}) ", 4 - 2 * x);
                self.adjacent_vertex(other_row, x).bits()
            }
            3 => {
                // Find the triangle vertex the lines up with the bottom right vertex
                let pos = (0 + z) % 6 < 3;
                let y = if pos { 2 } else { 0 };
                let x = (0 + i) % 3;
                let other_row = surrounding_tiles[y];
                print!("({}, {y}) ", 4 - 2 * x);
                self.adjacent_vertex(other_row, x).bits()
            }
            4 => {
                let pos = (0 + z) % 6 < 3;
                let y = if pos { 1 } else { 2 };
                let x = (0 + i) % 3 + 1;
                print!("({x}, {y}) ");
                surrounding_tiles[y][x].bits()
            }
            5 => {
                let pos = (2 + z) % 6 < 3;
                let y = if pos { 1 } else { 2 };
                let x = (2 + i) % 3 + 1;
                print!("({x}, {y}) ");
                surrounding_tiles[y][x].bits()
            }
            6 => {
                let pos = (4 + z) % 6 < 3;
                let y = if pos { 0 } else { 2 };
                let x = (1 + i) % 3 + 1;
                print!("({x}, {y}) ");
                surrounding_tiles[y][x].bits()
            }
            _ => Tile::Z1.bits(),
        };

        let tile_nets = calculate_nets(tile.bits());
        if tile_nets.is_empty() {
            return;
        }

        // TODO: Do I even care about the other nets ?
        let connected_nets = calculate_nets(connected_tile);
        println!("{k} >> {:?}", connected_nets);

        // for net in tile_nets {
        //     println!("  > ({i},{j}): {:?}", net);
        //
        //     for node in net {
        //         let key: [usize; 3] = [i, j, node.to_usize().unwrap_or_default()];
        //
        //         //
        //         let mut found = false;
        //         for existing_nets in self.nets.iter() {
        //             if existing_nets.contains(&key) {
        //                 println!("Found {i},{j},{:}", node);
        //                 found = true;
        //             }
        //         }
        //         if !found {
        //             // println!("Adding new net {i},{j},{:}", node);
        //             //self.nets.insert(Vec::from([key]));
        //         }
        //     }
        // }
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

        // It must be a valid tile.
        return *tiles.get(y, x).unwrap_or(&Tile::Z1);
    }

    fn adjacent_vertex(
        &self,
        other_row: [Tile; 5],
        pos: usize, // 0..2
    ) -> Tile {
        return other_row[4 - 2 * pos]; // index 4, 2, 0
    }

    fn get_connected_titles_for_tile(
        &self,
        i: usize,
        j: usize,
        tiles: &Array2D<Tile>,
    ) -> [Tile; 12] {
        let k = i + (3 * (j % 2));
        /*
        We only have six different ways a tile can be rotated
        For each rotation, we can then get the adjacent tile
        for each node index within the
        tile(i,j).

        So each k % 6 matches a rotation of the tile

        Each row is indexed in a strange way:  _,1,_,5,_,3,_,6,_,2,_,4
        where the _ is too the left and right of the vertex, (vertex are number 1,2 & 3).

        k % 6 ->
        {
         {0, 1, 2, 3, 4, 5},
         {3, 4, 5, 0, 1, 2},
         {0, 1, 2, 3, 4, 5},
         {3, 4, 5, 0, 1, 2},
         {0, 1, 2, 3, 4, 5},
         {3, 4, 5, 0, 1, 2}
        }

        */
        return match k % 6 {
            0 => [
                self.get_tile_relative_position((i, j), (-1, -1), tiles),
                self.get_tile_relative_position((i, j), (0, -1), tiles),
                self.get_tile_relative_position((i, j), (1, -1), tiles),
                self.get_tile_relative_position((i, j), (1, 0), tiles),
                self.get_tile_relative_position((i, j), (2, 0), tiles),
                self.get_tile_relative_position((i, j), (2, 1), tiles),
                self.get_tile_relative_position((i, j), (1, 1), tiles),
                self.get_tile_relative_position((i, j), (0, 1), tiles),
                self.get_tile_relative_position((i, j), (-1, 1), tiles),
                self.get_tile_relative_position((i, j), (-2, 1), tiles),
                self.get_tile_relative_position((i, j), (-2, 0), tiles),
                self.get_tile_relative_position((i, j), (-1, 0), tiles),
            ],
            1 => [
                self.get_tile_relative_position((i, j), (-1, -1), tiles),
                self.get_tile_relative_position((i, j), (-2, -1), tiles),
                self.get_tile_relative_position((i, j), (-2, 0), tiles),
                self.get_tile_relative_position((i, j), (-1, 0), tiles),
                self.get_tile_relative_position((i, j), (-1, 1), tiles),
                self.get_tile_relative_position((i, j), (0, 1), tiles),
                self.get_tile_relative_position((i, j), (1, 1), tiles),
                self.get_tile_relative_position((i, j), (1, 0), tiles),
                self.get_tile_relative_position((i, j), (2, 0), tiles),
                self.get_tile_relative_position((i, j), (2, -1), tiles),
                self.get_tile_relative_position((i, j), (1, -1), tiles),
                self.get_tile_relative_position((i, j), (0, -1), tiles),
            ],
            2 => [
                self.get_tile_relative_position((i, j), (2, 0), tiles),
                self.get_tile_relative_position((i, j), (2, 1), tiles),
                self.get_tile_relative_position((i, j), (1, 1), tiles),
                self.get_tile_relative_position((i, j), (0, 1), tiles),
                self.get_tile_relative_position((i, j), (-1, 1), tiles),
                self.get_tile_relative_position((i, j), (-2, 1), tiles),
                self.get_tile_relative_position((i, j), (-2, 0), tiles),
                self.get_tile_relative_position((i, j), (-1, 0), tiles),
                self.get_tile_relative_position((i, j), (-1, -1), tiles),
                self.get_tile_relative_position((i, j), (0, -1), tiles),
                self.get_tile_relative_position((i, j), (1, -1), tiles),
                self.get_tile_relative_position((i, j), (1, 0), tiles),
            ],
            3 => [
                self.get_tile_relative_position((i, j), (-1, 1), tiles),
                self.get_tile_relative_position((i, j), (0, 1), tiles),
                self.get_tile_relative_position((i, j), (1, 1), tiles),
                self.get_tile_relative_position((i, j), (1, 0), tiles),
                self.get_tile_relative_position((i, j), (2, 0), tiles),
                self.get_tile_relative_position((i, j), (2, -1), tiles),
                self.get_tile_relative_position((i, j), (1, -1), tiles),
                self.get_tile_relative_position((i, j), (0, -1), tiles),
                self.get_tile_relative_position((i, j), (-1, -1), tiles),
                self.get_tile_relative_position((i, j), (-2, -1), tiles),
                self.get_tile_relative_position((i, j), (-2, 0), tiles),
                self.get_tile_relative_position((i, j), (-1, 0), tiles),
            ],
            4 => [
                self.get_tile_relative_position((i, j), (-1, 1), tiles),
                self.get_tile_relative_position((i, j), (-2, 1), tiles),
                self.get_tile_relative_position((i, j), (-2, 0), tiles),
                self.get_tile_relative_position((i, j), (-1, 0), tiles),
                self.get_tile_relative_position((i, j), (-1, -1), tiles),
                self.get_tile_relative_position((i, j), (0, -1), tiles),
                self.get_tile_relative_position((i, j), (1, -1), tiles),
                self.get_tile_relative_position((i, j), (1, 0), tiles),
                self.get_tile_relative_position((i, j), (2, 0), tiles),
                self.get_tile_relative_position((i, j), (2, 1), tiles),
                self.get_tile_relative_position((i, j), (1, 1), tiles),
                self.get_tile_relative_position((i, j), (0, 1), tiles),
            ],
            5 => [
                self.get_tile_relative_position((i, j), (2, 0), tiles),
                self.get_tile_relative_position((i, j), (2, -1), tiles),
                self.get_tile_relative_position((i, j), (1, -1), tiles),
                self.get_tile_relative_position((i, j), (0, -1), tiles),
                self.get_tile_relative_position((i, j), (-1, -1), tiles),
                self.get_tile_relative_position((i, j), (-2, -1), tiles),
                self.get_tile_relative_position((i, j), (-2, 0), tiles),
                self.get_tile_relative_position((i, j), (-1, 0), tiles),
                self.get_tile_relative_position((i, j), (-1, 1), tiles),
                self.get_tile_relative_position((i, j), (0, 1), tiles),
                self.get_tile_relative_position((i, j), (1, 1), tiles),
                self.get_tile_relative_position((i, j), (1, 0), tiles),
            ],
            _ => panic!(), // Maths say I can't get here
        };
    }

    fn mask_for_tile_vertex(
        &self,
        opposite_idx: usize,
        lower_left_idx: usize,
        lower_right_idx: usize,
        tile: Tile,
        relative_tiles: [Tile; 12],
    ) -> (bool, bool) {
        let opposite = relative_tiles[opposite_idx];

        let left = relative_tiles[opposite_idx - 1];
        let right = relative_tiles[opposite_idx + 1];

        let l_adjacent = relative_tiles[lower_left_idx];
        let r_adjacent = relative_tiles[lower_right_idx];

        if opposite.contains(Tile::Z2) {
            if !left.intersects(tile)
                && !right.intersects(tile)
                && !l_adjacent.intersects(tile)
                && !r_adjacent.intersects(tile)
            {
                // Can be anything
                return (false, true);
            }
        } else if opposite.intersects(tile) {
            // Must be connected
            return (true, false);
        }
        // Must be unconnected
        return (false, false);
    }

    fn mask_for_tile_side(
        &self,
        opposite_idx: usize,
        tile: Tile,
        relative_tiles: [Tile; 12],
    ) -> (bool, bool) {
        let adjacent = relative_tiles[opposite_idx];

        if adjacent.contains(Tile::Z2) {
            // Don't care
            return (false, true);
        }
        if adjacent.intersects(tile) {
            // Must match, on a one
            return (true, false);
        }
        // Must match, on a zero
        return (false, false);
    }

    pub fn get_surrounding_tile_connections(
        &self,
        i: usize,
        j: usize,
        tiles: &Array2D<Tile>,
    ) -> (u32, u32) {
        // Get the surrounding tiles

        let mut dont_care = 0u32;
        let mut matches = 0u32;

        // Calculate the relative positions of the surrounding tiles
        let k = i + (3 * (j % 2));

        let pos = [(4 + k) % 6, (2 + k) % 6, (0 + k) % 6];

        let relative_tiles = self.get_connected_titles_for_tile(i, j, tiles);

        // Node 1
        let (m, d) = self.mask_for_tile_vertex(1, 3, 11, Tile::G1, relative_tiles);
        if m {
            matches |= 0b000_001;
        } else if d {
            dont_care |= 0b000_001;
        }

        // Node 2
        let (m, d) = self.mask_for_tile_vertex(9, 11, 7, Tile::G2, relative_tiles);

        if m {
            matches |= 0b000_010;
        } else if d {
            dont_care |= 0b000_010;
        }

        // Node3
        let (m, d) = self.mask_for_tile_vertex(5, 3, 7, Tile::G3, relative_tiles);
        if m {
            matches |= 0b000_100;
        } else if d {
            dont_care |= 0b000_100;
        }

        let (m, d) = self.mask_for_tile_side(11, Tile::G4, relative_tiles);
        if m {
            matches |= 0b001_000;
        } else if d {
            dont_care |= 0b001_000;
        }

        let (m, d) = self.mask_for_tile_side(3, Tile::G5, relative_tiles);
        if m {
            matches |= 0b010_000;
        } else if d {
            dont_care |= 0b010_000;
        }

        let (m, d) = self.mask_for_tile_side(7, Tile::G6, relative_tiles);
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
            let first_tile = random_side(matches, dont_care, self.seed);
            if !first_tile.intersects(Tile::N1)
                && !first_tile.intersects(Tile::N2)
                && !first_tile.intersects(Tile::N3)
            {
                return first_tile;
            }
        }
        return random_side(matches, dont_care, self.seed);
    }

    pub fn create_all_title_in_grid(&mut self) {
        let possible_tiles = find_tiles(0b101010101010);

        let mut row_path: Vec<Glyph> = Vec::new();
        let mut row_wall: Vec<Glyph> = Vec::new();

        let i = 0;
        let mut j: usize = 0;

        for tile in possible_tiles.iter() {
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
        let mut wall = Path::new("red".to_string(), 3.0, 0.1);
        let mut green_wall = Path::new("green".to_string(), 3.0, 0.3);
        let mut red_wall = Path::new("red".to_string(), 3.0, 0.3);

        for row in &self.glyphs {
            for tile in row {
                tile.add_to_path(&mut path, &mut faint_path);
            }
        }
        let mut i = 0;
        let mut j = 0;
        for row in &self.walls {
            i = 0;
            for tile in row {
                if false && i == 4 && j == 1 {
                    tile.add_to_path(&mut red_wall, &mut faint_path)
                } else {
                    let k = i + (3 * (j % 2));
                    if k % 6 == 0 {
                        tile.add_to_path(&mut green_wall, &mut faint_path)
                    } else {
                        tile.add_to_path(&mut wall, &mut faint_path)
                    }
                }
                i += 1;
            }
            j += 1;
        }

        svg.push_str(&faint_path.to_string());
        svg.push_str(&path.to_string());
        svg.push_str(&wall.to_string());
        svg.push_str(&green_wall.to_string());
        svg.push_str(&red_wall.to_string());
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
    use std::println;

    // // TODO: Add tests for getting networks
    // #[test]
    // fn very_small_net() {
    //     let maze = Maze::new(3, 2, true, Some(37u64));
    //     // let mut row_path_1: Vec<Glyph> = Vec::new();
    //     // let mut row_path_2: Vec<Glyph> = Vec::new();
    //     //
    //     // let title1 = Glyph::new(Coordinate { i: 0, j: 0 }, Tile::C2);
    //     // let title2 = Glyph::new(Coordinate { i: 1, j: 0 }, Tile::C1);
    //     // row_path_1.push(title1);
    //     // row_path_1.push(title2);
    //     //
    //     // let title3 = Glyph::new(Coordinate { i: 0, j: 1 }, Tile::A2);
    //     // let title4 = Glyph::new(Coordinate { i: 1, j: 1 }, Tile::L1);
    //     // row_path_2.push(title3);
    //     // row_path_2.push(title4);
    //     //
    //     // maze.add_row(row_path_1);
    //     // maze.add_row(row_path_2);
    //     //
    //     // for i in [0, 1] {
    //     //     let mut row_wall: Vec<Glyph> = Vec::new();
    //     //     for j in [0, 1] {
    //     //         row_wall.push(Glyph::new(Coordinate { i, j }, Tile::W1));
    //     //     }
    //     //     maze.walls.push(row_wall);
    //     // }
    //
    //     // maze.smoosh();
    //
    //     let result_f = maze.render("../test-smoosh.svg".to_string());
    //
    //     println!("Finished {:?}", result_f.unwrap());
    // }
}

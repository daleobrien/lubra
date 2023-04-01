use std::cmp::max;
use std::collections::HashSet;
use std::error::Error;
use std::fs;

use array2d::Array2D;

use crate::maze::tile::{get_random_tile, Tile, H, M};
use crate::renders::glyph::Glyph;
use crate::renders::primatives::Coordinate;
use crate::renders::svg::Path;
use crate::renders::tile_subsets::find_tiles;

pub struct Maze {
    pub style: Option<String>,
    tiles: Vec<Vec<Glyph>>,
    walls: Vec<Vec<Glyph>>,
    nets: HashSet<Vec<[u32; 3]>>,
    last_free_net_id: u32,
    width: f64,
    height: f64,
}

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

                let tile = get_random_tile(i, j, &tiles);

                tiles[(j, i)] = tile;
                let path: Glyph = Glyph::new(coord, tile, self.last_free_net_id);
                self.last_free_net_id = max(path.max(), self.last_free_net_id);
                // TODO: updated self.nets too, so I can find which tiles have a given net id.

                println!(
                    ":->  ({i},{j}) -> {} {:?}",
                    self.last_free_net_id, path.nets
                );

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
        // TODO: Titles don't really need i.j postions
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

    pub fn get_surrounding_tiles(&mut self, i: usize, j: usize) {}

    // TODO:Added debug only method to create the grid outlines, aka the walls
    pub fn smoosh(&mut self) {
        for row in self.tiles.iter() {
            for tile in row {
                // TODO: get neighbours and so on ...
                println!("{:?}", tile.nets);
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

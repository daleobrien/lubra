use std::error::Error;
use std::fs;

use rand::seq::SliceRandom;

use crate::renders::constants::{TileFlags, H, M};
use crate::renders::primatives::Coordinate;
use crate::renders::render_tile::Tile;
use crate::renders::svg_path::Path;
use crate::renders::tile_subsets::{match_side_1, match_side_1_2_3, match_side_3};

pub struct RenderSvg {
    pub style: Option<String>,
    tiles: Vec<Vec<Tile>>,
    walls: Vec<Vec<Tile>>,
    width: f64,
    height: f64,
}

impl RenderSvg {
    pub fn new() -> Self {
        let tiles: Vec<Vec<Tile>> = Vec::new();
        let walls: Vec<Vec<Tile>> = Vec::new();

        let obj = Self {
            style: None,
            tiles,
            walls,
            width: 0f64,
            height: 0f64,
        };

        return obj;
    }

    pub fn display_one_tile(&mut self) {
        let mut row_path: Vec<Tile> = Vec::new();
        let mut row_wall: Vec<Tile> = Vec::new();

        let path1: Tile = Tile::new(Coordinate { i: 0, j: 0 }, TileFlags::Y2 | TileFlags::T2);
        let path2: Tile = Tile::new(Coordinate { i: 1, j: 0 }, TileFlags::Y2 | TileFlags::T2);

        row_path.push(path1);
        row_path.push(path2);

        let wall1: Tile = Tile::new(Coordinate { i: 0, j: 0 }, TileFlags::W1);
        let wall2: Tile = Tile::new(Coordinate { i: 1, j: 0 }, TileFlags::W1);
        //
        row_wall.push(wall1);
        row_wall.push(wall2);

        self.tiles.push(row_path);
        self.walls.push(row_wall);

        self.width = 2.0 * (M + 50.0);
        self.height = H + 0.0;
    }

    pub fn display_all_tiles(&mut self) {
        for j in 0..63 {
            let mut row_path: Vec<Tile> = Vec::new();
            let mut row_wall: Vec<Tile> = Vec::new();

            let possible_tiles = match_side_1_2_3(j);

            let mut i = 0;
            //println!("");
            for tile in possible_tiles.iter() {
                //println!("{j} {i} {:?}", TileFlags::from_bits_truncate(*tile));

                let coord1: Coordinate = Coordinate { i: i * 2, j: j * 2 };
                let coord2: Coordinate = Coordinate {
                    i: i * 2 + 1,
                    j: j * 2,
                };

                let path1: Tile = Tile::new(coord1, TileFlags::from_bits_truncate(*tile));
                let wall1: Tile = Tile::new(coord1, TileFlags::W1);

                let path2: Tile = Tile::new(coord2, TileFlags::from_bits_truncate(*tile));
                let wall2: Tile = Tile::new(coord2, TileFlags::W1);

                row_path.push(path1);
                row_wall.push(wall1);
                row_path.push(path2);
                row_wall.push(wall2);
                i += 1;
            }
            self.tiles.push(row_path);
            self.walls.push(row_wall);
        }

        self.width = (10 * 2) as f64 * (M + 50.0);
        self.height = (63 * 2) as f64 * (H + 00.0);
    }

    pub fn display_random_grid(&mut self) {
        let m = 9;
        let n = 6;
        for j in 0..n {
            let mut row_path: Vec<Tile> = Vec::new();
            let mut row_wall: Vec<Tile> = Vec::new();

            //println!("");
            for i in 0..m {
                //println!("{j} {i} {:?}", TileFlags::from_bits_truncate(*tile));

                let coord: Coordinate = Coordinate { i, j };

                let mut tile: Option<&u32> = Some(&TileFlags::W1.bits());

                if i == 0 && j == 0 {
                    let possible_tiles = match_side_1(0);
                    tile = possible_tiles.choose(&mut rand::thread_rng());
                    if tile.is_some() {
                        let path: Tile = Tile::new(
                            coord,
                            TileFlags::from_bits_truncate(*tile.unwrap_or(&TileFlags::W1.bits())),
                        );
                        row_path.push(path);
                    }
                } else if j == 0 && i % 2 == 1 {
                    println!("{i}");
                    let possible_tiles = match_side_3(0);
                    tile = possible_tiles.choose(&mut rand::thread_rng());
                    if tile.is_some() {
                        let path: Tile = Tile::new(
                            coord,
                            TileFlags::from_bits_truncate(*tile.unwrap_or(&TileFlags::W1.bits())),
                        );
                        row_path.push(path);
                    }
                }

                let wall: Tile = Tile::new(coord, TileFlags::W1);

                row_wall.push(wall);
            }
            self.tiles.push(row_path);
            self.walls.push(row_wall);
        }

        self.width = (m + 1) as f64 * M;
        self.height = (n) as f64 * H;
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

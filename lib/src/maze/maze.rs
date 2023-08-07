use std::collections::HashSet;
use std::error::Error;
use std::fs;

use crate::maze::consts::NODE_TO_COORD;
use crate::maze::net::calculate_nets;

use array2d::Array2D;
use rand_distr::num_traits::ToPrimitive;

use crate::maze::tile::{add, Tile};
use crate::renders::glyph::Glyph;
use crate::renders::lookup::random_side;
use crate::renders::primatives::Coordinate;
use crate::renders::strokes::{H, M};
use crate::renders::svg::Path;

pub struct Maze {
    pub style: Option<String>,
    glyphs: Vec<Vec<Glyph>>,
    walls: Vec<Vec<Glyph>>,
    nets: Vec<HashSet<[usize; 3]>>,
    width: f64,
    height: f64,
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
        let nets = Vec::new();

        let mut obj = Self {
            style: None,
            glyphs,
            walls,
            width: 0f64,
            height: 0f64,
            nets,
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

                let surrounding_tiles = self.get_connected_titles_for_tile(i, j, &tiles);

                let tile = self.get_random_tile(i, j, &surrounding_tiles);

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

        // Sort such that the last net is the shortest, we will be deleting them as we go
        self.nets.sort_by(|a, b| (&b.len()).cmp(&a.len()));

        // Join the nets together such that we have a comeplete maze!

        // TODO: At the point we can remove nets that have a length of 2, since they are just mini-circle
        // It does seem that it can be very hard to join some nets!!!
        println!("\nWe have disjoint {:?} nets:", self.nets.len());
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
    fn update_network(&mut self, tile: &Tile, i: usize, j: usize) {
        let tile_nets = calculate_nets(tile.bits());

        let mapping = NODE_TO_COORD[(i + (3 * (j % 2))) % 6];

        for net in tile_nets {
            let tile_sub_nets: HashSet<[usize; 3]> = net
                .iter()
                .map(|node| [i, j, node.to_usize().unwrap_or_default()])
                .collect();

            // For this sub net of the current tile, find which nets they are part of
            let mut net_indexes: Vec<usize> = Vec::new();

            for (n, nets) in self.nets.iter().enumerate() {
                for tile_sub_net in tile_sub_nets.iter() {
                    // Find adjacent tile so we can check if it is part of an existing net
                    let node_idx = tile_sub_net[2];
                    let coord = mapping[node_idx - 1];
                    let x = add(i, coord.0);
                    let y = add(j, coord.1);

                    if nets.contains(&[x, y, node_idx]) && !net_indexes.contains(&n) {
                        net_indexes.push(n);
                        //break;
                    }
                }
            }
            if net_indexes.len() == 1 {
                // Append to an existing net
                let n = net_indexes[0];
                self.nets[n].extend(tile_sub_nets);
            } else if net_indexes.len() == 0 {
                // Add a new net
                self.nets.push(tile_sub_nets);
            } else if net_indexes.len() > 1 {
                // Since this tile's subnet was part of 2 or more nets, we can say they are really
                // the same net. So we need to merge them.

                let n = net_indexes[0];
                // Keep the net[n] and merge the other nets into it
                let r = net_indexes[1..].to_vec();

                let mut new_net = HashSet::new();
                // Add in the bridging net
                new_net.extend(tile_sub_nets);
                // Add the rest of the nets
                for i in net_indexes {
                    new_net.extend(self.nets[i].clone());
                }

                self.nets[n] = new_net; // The new merged net

                // Remove the rest of the nets since they are now merged.
                // Note: This won't have very good performance. It might be the other nets are always towards the end of the list though
                // which will help.
                for i in r.iter().rev() {
                    self.nets.remove(*i);
                }
            }
        }
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

    fn get_connected_titles_for_tile(
        &self,
        i: usize,
        j: usize,
        tiles: &Array2D<Tile>,
    ) -> [Tile; 12] {
        let k = i + (3 * (j % 2));
        /*
        We only have six different ways a tile can be rotated/flipped
        For each rotation/flip, we can then get the adjacent tile
        for each node index within the tile at {i,j}.

        So each k % 6 matches a rotation of the tile
        Each row is indexed:  1,2,3,4,5,6,1l,1r,2l,2r,3l,3r

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
        return NODE_TO_COORD[k % 6]
            .map(|coord| self.get_tile_relative_position((i, j), coord, tiles));
    }

    fn mask_for_tile_vertex(
        &self,
        opposite_idx: usize,
        idxs: [usize; 4],
        tile: Tile,
        relative_tiles: &[Tile; 12],
        mask: u32,
        dont_care_mask: &mut u32,
        match_mask: &mut u32,
    ) {
        let opposite = relative_tiles[opposite_idx];
        let left = relative_tiles[idxs[0]];
        let right = relative_tiles[idxs[1]];
        let l_adjacent = relative_tiles[idxs[2]];
        let r_adjacent = relative_tiles[idxs[3]];

        if opposite.contains(Tile::Z2) {
            if !left.intersects(tile)
                && !right.intersects(tile)
                && !l_adjacent.intersects(tile)
                && !r_adjacent.intersects(tile)
            {
                *dont_care_mask |= mask;
                // Can be anything
                return;
            }
        } else if opposite.intersects(tile) {
            // Must be connected
            *match_mask |= mask;
        }
    }

    fn mask_for_tile_side(
        &self,
        opposite_idx: usize,
        tile: Tile,
        relative_tiles: &[Tile; 12],
        mask: u32,
        dont_care_mask: &mut u32,
        match_mask: &mut u32,
    ) {
        let adjacent = relative_tiles[opposite_idx];

        if adjacent.contains(Tile::Z2) {
            *dont_care_mask |= mask;
            return;
        }
        if adjacent.intersects(tile) {
            *match_mask |= mask;
        }
        return;
    }

    pub fn get_surrounding_tile_connections(&self, surrounding_tiles: &[Tile; 12]) -> (u32, u32) {
        // Get the surrounding tiles

        let mut dont_care = 0u32;
        let mut matches = 0u32;

        // 1,2,3,4,5,6, 1a,1b,2a,2b,3a,3b
        // 0 1 2 3 4 5  6  7  8  9  10 11

        // Node 1
        self.mask_for_tile_vertex(
            0, // 1
            [3, 4, 6, 7],
            Tile::G1,
            surrounding_tiles,
            0b000_001,
            &mut dont_care,
            &mut matches,
        );
        self.mask_for_tile_vertex(
            1, // 2
            [3, 5, 8, 9],
            Tile::G2,
            surrounding_tiles,
            0b000_010,
            &mut dont_care,
            &mut matches,
        );
        self.mask_for_tile_vertex(
            2, // 3
            [4, 5, 10, 11],
            Tile::G3,
            surrounding_tiles,
            0b000_100,
            &mut dont_care,
            &mut matches,
        );
        self.mask_for_tile_side(
            3,
            Tile::G4,
            surrounding_tiles,
            0b001_000,
            &mut dont_care,
            &mut matches,
        );
        self.mask_for_tile_side(
            4,
            Tile::G5,
            surrounding_tiles,
            0b010_000,
            &mut dont_care,
            &mut matches,
        );
        self.mask_for_tile_side(
            5,
            Tile::G6,
            surrounding_tiles,
            0b100_000,
            &mut dont_care,
            &mut matches,
        );

        return (dont_care, matches);
    }

    fn get_random_tile(&mut self, i: usize, j: usize, surrounding_tiles: &[Tile; 12]) -> Tile {
        // TODO: Need to also put in the last tile too
        if i == 0 && j == 0 {
            return Tile::L3; // Always start with a left tile
        }

        let (dont_care, matches) = self.get_surrounding_tile_connections(surrounding_tiles);

        let tile = random_side(matches, dont_care, self.seed);

        self.seed = match self.seed {
            Some(s) => Some(s.wrapping_add(1)),
            _ => None,
        };

        // Update the network the describes the maze
        self.update_network(&tile, i, j);

        return tile;
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
        //let mut green_wall = Path::new("green".to_string(), 3.0, 0.3);
        //let mut red_wall = Path::new("red".to_string(), 3.0, 0.3);

        for row in &self.glyphs {
            for tile in row {
                tile.add_to_path(&mut path, &mut faint_path);
            }
        }
        //let mut j = 0;
        for row in &self.walls {
            //let mut i = 0;
            for tile in row {
                tile.add_to_path(&mut wall, &mut faint_path);
                // if false && i == 4 && j == 1 {
                //     tile.add_to_path(&mut red_wall, &mut faint_path)
                // } else {
                //     let k = i + (3 * (j % 2));
                //     if k % 6 == 0 {
                //         tile.add_to_path(&mut green_wall, &mut faint_path)
                //     } else {
                //         tile.add_to_path(&mut wall, &mut faint_path)
                //     }
                // }
                //i += 1;
            }
            //j += 1;
        }

        svg.push_str(&faint_path.to_string());
        svg.push_str(&path.to_string());
        svg.push_str(&wall.to_string());
        //svg.push_str(&green_wall.to_string());
        //svg.push_str(&red_wall.to_string());
        svg.push_str("</svg>");

        fs::write(file_name, svg.as_bytes())?;
        Ok(())
    }
}

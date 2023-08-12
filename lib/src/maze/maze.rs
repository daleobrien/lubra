use std::collections::HashSet;
use std::error::Error;
use std::fs;

use crate::maze::consts::NODE_TO_COORD;

use crate::maze::random::random_tile;
use array2d::Array2D;
use rand_distr::num_traits::ToPrimitive;

use crate::maze::tile::{add, Tile};
use crate::renders::glyph::Glyph;
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
        let mut tiles = Array2D::filled_with(Tile::new(vec![]), number_of_rows, number_of_columns);
        for j in 0..number_of_rows {
            let mut row_path: Vec<Glyph> = Vec::new();
            let mut row_wall: Vec<Glyph> = Vec::new();

            for i in 0..number_of_columns {
                let coord: Coordinate = Coordinate { i, j };

                let surrounding_tiles = self.get_connected_titles_for_tile(i, j, &tiles);

                let tile = self.get_random_tile(i, j, &surrounding_tiles);

                tiles[(j, i)] = tile;

                // let glyph: Glyph = Glyph::new(coord, tile);
                // row_path.push(glyph);
                // let glyph: Glyph = Glyph::new(coord, Tile::W1);
                // row_wall.push(glyph);
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

           ▽△▽△▽      △▽△▽△:
           △▽▲▽△  or  ▽△▼△▽
           ▽△▽△▽      △▽△▽△

        */
    fn update_network(&mut self, tile: &Tile, i: usize, j: usize) {
        let tile_nets = tile.nodes.clone();

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

        // for i in [0, 3, 6, 9, 12, 15] {
        //     for j in [0, 2, 4, 6, 8, 10] {
        //         row_path.push(Glyph::new(Coordinate { i, j }, Tile::C3));
        //         row_wall.push(Glyph::new(Coordinate { i, j }, Tile::W1));
        //     }
        // }

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
    ) -> Option<Tile> {
        // Out of bounds, Array2D will return None if out of bounds.
        // if pos.0 as i32 + rel.0 < 0 || pos.1 as i32 + rel.1 < 0 {
        //     return Tile::new(vec![]);
        // }

        let x: usize = add(pos.0, rel.0);
        let y: usize = add(pos.1, rel.1);

        return tiles.get(y, x).cloned();
    }

    fn get_connected_titles_for_tile(
        &self,
        i: usize,
        j: usize,
        tiles: &Array2D<Tile>,
    ) -> [Option<Tile>; 12] {
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

    fn tile_is_empty(&self, tile: &Option<Tile>) -> bool {
        match tile {
            Some(t) => return t.is_empty(),
            None => return true,
        }
    }

    fn tile_contains(&self, tile: &Option<Tile>, node_idx: u8) -> bool {
        match tile {
            Some(t) => return t.contains(node_idx),
            None => return false,
        }
    }

    fn mask_for_tile_vertex(
        &self,
        node_idx: u8,
        idxs: [usize; 4],
        relative_tiles: &[Option<Tile>; 12],
    ) -> u32 {
        let opposite = &relative_tiles[(node_idx - 1).to_usize().unwrap_or_default()];

        let left = &relative_tiles[idxs[0]];
        let right = &relative_tiles[idxs[1]];
        let l_adjacent = &relative_tiles[idxs[2]];
        let r_adjacent = &relative_tiles[idxs[3]];
        //
        if self.tile_is_empty(opposite) {
            if !self.tile_contains(left, node_idx)
                && !self.tile_contains(right, node_idx)
                && !self.tile_contains(l_adjacent, node_idx)
                && !self.tile_contains(r_adjacent, node_idx)
            {
                // Can be anything
                return match node_idx {
                    1 => 2,
                    2 => 20,
                    3 => 200,
                    _ => 0,
                };
            }
        } else if self.tile_contains(opposite, node_idx) {
            // Must be connected
            return match node_idx {
                1 => 1,
                2 => 10,
                3 => 100,
                _ => 0,
            };
        }
        return 0;
    }

    fn mask_for_tile_side(&self, node_idx: u8, relative_tiles: &[Option<Tile>; 12]) -> u32 {
        let adjacent = &relative_tiles[(node_idx - 1).to_usize().unwrap_or_default()];

        if self.tile_is_empty(adjacent) {
            return match node_idx {
                4 => 2000,
                5 => 20000,
                6 => 200000,
                _ => 0,
            };
        }

        if self.tile_contains(adjacent, node_idx) {
            return match node_idx {
                4 => 1000,
                5 => 10000,
                6 => 100000,
                _ => 0,
            };
        }

        return 0;
    }

    pub fn get_surrounding_tile_connections(&self, surrounding_tiles: &[Option<Tile>; 12]) -> u32 {
        // Get the surrounding tiles

        let mut mask = 0u32;

        // 1,2,3,4,5,6, 1a,1b,2a,2b,3a,3b
        // 0 1 2 3 4 5  6  7  8  9  10 11

        // Node 1
        mask |= self.mask_for_tile_vertex(1, [3, 4, 6, 7], surrounding_tiles);
        mask |= self.mask_for_tile_vertex(2, [3, 5, 8, 9], surrounding_tiles);
        mask |= self.mask_for_tile_vertex(3, [4, 5, 10, 11], surrounding_tiles);
        mask |= self.mask_for_tile_side(4, surrounding_tiles);
        mask |= self.mask_for_tile_side(5, surrounding_tiles);
        mask |= self.mask_for_tile_side(6, surrounding_tiles);

        return mask;
    }

    fn get_random_tile(
        &mut self,
        i: usize,
        j: usize,
        surrounding_tiles: &[Option<Tile>; 12],
    ) -> Tile {
        // TODO: Need to also put in the last tile too
        if i == 0 && j == 0 {
            return Tile::new(vec![vec![4, 5]]);
        }

        let mask = self.get_surrounding_tile_connections(surrounding_tiles);

        let nets = random_tile(mask, self.seed);

        self.seed = match self.seed {
            Some(s) => Some(s.wrapping_add(1)),
            _ => None,
        };

        let tile = Tile::new(nets);

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

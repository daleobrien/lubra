use std::collections::HashSet;
use std::error::Error;
use std::fs;

use crate::maze::consts::NODE_TO_COORD;

use crate::maze::random::random_tile;
use array2d::Array2D;
use rand_distr::num_traits::ToPrimitive;

use crate::maze::tile::{add, Tile};
use crate::renders::_point::Coordinate;
use crate::renders::svg::Svg;
use common::types::point::{Point, TILE_HEIGHT, TILE_WIDTH};

const NODE_1: usize = 0usize;
const NODE_2: usize = 1usize;
const NODE_3: usize = 2usize;
const NODE_4: usize = 3usize;
const NODE_5: usize = 4usize;
const NODE_6: usize = 5usize;

const NODE_1L: usize = 6usize;
const NODE_1R: usize = 7usize;
const NODE_2L: usize = 8usize;
const NODE_2R: usize = 9usize;
const NODE_3L: usize = 10usize;
const NODE_3R: usize = 11usize;

#[cfg(debug_assertions)]
fn pretty_print_int(b: u32) {
    let mut s_iter = format!("{:b}", b);
    let mut s = String::new();
    for (idx, val) in s_iter.chars().rev().enumerate() {
        if idx != 0 && idx % 2 == 0 {
            s.insert(0, '_');
        }
        s.insert(0, val);
    }
    println!("{}", s);
}

pub struct Maze {
    pub style: Option<String>,
    nets: Vec<HashSet<[usize; 3]>>,
    width: f64,
    height: f64,
    seed: Option<u64>,
    number_of_columns: usize,
    number_of_rows: usize,
    tiles: Array2D<Tile>,
}

impl Maze {
    pub fn new(
        number_of_columns: usize,
        number_of_rows: usize,
        randomise: bool,
        seed: Option<u64>,
    ) -> Self {
        // User Maze object somehow

        let nets = Vec::new();

        let tiles: Array2D<Tile> =
            Array2D::filled_with(Tile::new(vec![], false), number_of_rows, number_of_columns);

        let mut obj = Self {
            style: None,
            width: 0f64,
            height: 0f64,
            nets,
            seed,
            number_of_columns,
            number_of_rows,
            tiles,
        };

        if randomise {
            Self::create_random_grid(&mut obj, number_of_columns, number_of_rows);
        }
        return obj;
    }

    pub fn create_random_grid(&mut self, number_of_columns: usize, number_of_rows: usize) {
        // Empty Array2D of Option<Tile>
        for j in 0..number_of_rows {
            for i in 0..number_of_columns {
                let coord: Coordinate = Coordinate { i, j };

                let surrounding_tiles = self.get_connected_titles_for_tile(i, j);

                let tile = self.get_random_tile(i, j, &surrounding_tiles);

                println!("Tile({i},{j}): {:?}", tile.nodes);
                self.tiles[(j, i)] = tile;
            }
        }

        self.width = (number_of_columns + 1) as f64 * TILE_WIDTH / 2.0;
        self.height = number_of_rows as f64 * TILE_HEIGHT;

        // Sort such that the last net is the shortest, we will be deleting them as we go
        self.nets.sort_by(|a, b| (&b.len()).cmp(&a.len()));

        // Join the nets together such that we have a complete maze!

        // TODO: At the point we can remove nets that have a length of 2, since they are just mini-circle
        // It does seem that it can be very hard to join some nets!!!
        // println!("\nWe have disjoint {:?} nets:", self.nets.len());
        // for (n, nets) in self.nets.iter().enumerate() {
        //     println!("{n}-> {:?}", nets);
        // }
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
                    //
                    let x = add(i, coord.0);
                    let y = add(j, coord.1);

                    match (x, y) {
                        (Some(x), Some(y)) => {
                            if nets.contains(&[x, y, node_idx]) && !net_indexes.contains(&n) {
                                net_indexes.push(n);
                                //break;
                            }
                        }
                        _ => {}
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

    fn get_tile_relative_position(&self, pos: (usize, usize), rel: (i32, i32)) -> Option<Tile> {
        // Out of bounds, Array2D will return None if out of bounds.
        // if pos.0 as i32 + rel.0 < 0 || pos.1 as i32 + rel.1 < 0 {
        //     return Tile::new(vec![]);
        // }

        let x = add(pos.0, rel.0);
        let y = add(pos.1, rel.1);

        // println!(
        //     "pos ({},{}) + rel ({},{}) -> ({x:?},{y:?})",
        //     pos.0, pos.1, rel.0, rel.1
        // );
        return match (x, y) {
            (Some(x), Some(y)) => self.tiles.get(y, x).cloned(),
            _ => None, // Out of bounds, not possible tile
        };
    }

    fn get_connected_titles_for_tile(&self, i: usize, j: usize) -> [Option<Tile>; 12] {
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

        return NODE_TO_COORD[k % 6].map(|coord| self.get_tile_relative_position((i, j), coord));
    }

    fn tile_is_empty(&self, tile: &Option<Tile>) -> bool {
        //
        match tile {
            Some(t) => return !t.initialised, // A tile, may or may not be free
            None => return false,             // It's not really empty, but it's not a tile either.
        }
    }

    fn tile_contains(&self, tile: &Option<Tile>, node_idx: u32) -> bool {
        match tile {
            Some(t) => return t.contains(node_idx),
            None => return false,
        }
    }

    fn mask_for_tile_vertex(
        &self,
        relative_idx: usize,
        idxs: [usize; 4],
        relative_tiles: &[Option<Tile>; 12],
    ) -> u32 {
        let opposite = &relative_tiles[relative_idx];

        let left = &relative_tiles[idxs[0]];
        let right = &relative_tiles[idxs[1]];
        let l_adjacent = &relative_tiles[idxs[2]];
        let r_adjacent = &relative_tiles[idxs[3]];
        let node_value = 1 + relative_idx as u32;

        //
        if self.tile_is_empty(opposite) {
            if !self.tile_contains(left, node_value)
                && !self.tile_contains(right, node_value)
                && !self.tile_contains(l_adjacent, node_value)
                && !self.tile_contains(r_adjacent, node_value)
            {
                // Can be anything
                return match node_value {
                    1 => 2,
                    2 => 20,
                    3 => 200,
                    _ => 0,
                };
            }
        } else if self.tile_contains(opposite, node_value) {
            // Must be connected
            return match node_value {
                1 => 1,
                2 => 10,
                3 => 100,
                _ => 0,
            };
        }
        return 0;
    }

    fn mask_for_tile_side(&self, relative_idx: usize, relative_tiles: &[Option<Tile>; 12]) -> u32 {
        let adjacent = &relative_tiles[relative_idx];

        let node_value = 1 + relative_idx as u32;

        if self.tile_is_empty(adjacent) {
            return match node_value {
                4 => 2000,
                5 => 20000,
                6 => 200000,
                _ => 0,
            };
        }

        if self.tile_contains(adjacent, node_value) {
            return match node_value {
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

        // 1,2,3, 4,5,6, 1a,1b, 2a,2b, 3a,3b
        // 0 1 2  3 4 5  6  7   8  9   10 11

        // Node 1
        return self.mask_for_tile_vertex(NODE_1, [NODE_4, NODE_5, NODE_1L, NODE_1R], surrounding_tiles)
            // Node 2
            + self.mask_for_tile_vertex(NODE_2, [NODE_4, NODE_6, NODE_2L, NODE_2R], surrounding_tiles)
            // Node 3
            + self.mask_for_tile_vertex(NODE_3, [NODE_3, NODE_5, NODE_3L, NODE_3R], surrounding_tiles)
            + self.mask_for_tile_side(NODE_4, surrounding_tiles)
            + self.mask_for_tile_side(NODE_5, surrounding_tiles)
            + self.mask_for_tile_side(NODE_6, surrounding_tiles);
    }
    #[cfg(debug_assertions)]
    fn string_for_tile(&self, tile: &Option<Tile>) -> String {
        match tile {
            Some(t) => return t.to_string(),
            None => return "⌀".to_string(),
        }
    }

    #[cfg(debug_assertions)]
    fn print_surrounding(&self, surrounding_tiles: &[Option<Tile>; 12]) {
        // 1,2,3,4,5,6, 1a,1b,2a,2b,3a,3b
        // 0 1 2 3 4 5  6  7  8  9  10 11
        println!(
            "Surround tiles: {:?}",
            &(surrounding_tiles
                .iter()
                .map(|t| self.string_for_tile(t))
                .collect::<Vec<String>>())
        );
    }

    fn get_random_tile(
        &mut self,
        i: usize,
        j: usize,
        surrounding_tiles: &[Option<Tile>; 12],
    ) -> Tile {
        //return Tile::new(vec![vec![2]], true);

        // println!("\n get random ({i},{j})");
        // TODO: Need to also put in the last tile too
        if i == 0 && j == 0 {
            return Tile::new(vec![vec![4, 5]], true);
        }
        // if i == 0 && j == 1 {
        //     return Tile::new(vec![vec![3, 4]], true);
        // }

        //#[cfg(debug_assertions)]
        //self.print_surrounding(surrounding_tiles);

        let mask = self.get_surrounding_tile_connections(surrounding_tiles);
        let nets = random_tile(mask, self.seed);
        // DEBUG
        let num = mask
            .to_string()
            .as_bytes()
            .rchunks(3)
            .rev()
            .map(std::str::from_utf8)
            .collect::<Result<Vec<&str>, _>>()
            .unwrap()
            .join("_"); // separator

        self.seed = match self.seed {
            Some(s) => Some(s.wrapping_add(1)),
            _ => None,
        };

        let tile = Tile::new(nets, true);

        self.update_network(&tile, i, j);

        return tile;
    }

    // Path commands
    // https://github.com/bodoni/svg/blob/master/src/node/element/path/data.rs
    pub fn render(self, file_name: String) -> Result<(), Box<dyn Error>> {
        let margin = 5.0;
        let w = (self.number_of_columns + 1) as f64 * TILE_WIDTH / 2.0 + 5.0 * margin;
        let h = self.number_of_rows as f64 * TILE_HEIGHT + 5.0 * margin;

        // println!(
        //     "Render {w},{h}\n",
        //     w = self.number_of_columns,
        //     h = self.number_of_rows
        // );

        // Create header and so on
        let mut svg = Svg::new(w, h);

        for j in 0..self.number_of_rows {
            for i in 0..self.number_of_columns {
                let coord: Coordinate = Coordinate { i, j };
                let point: Point = Point {
                    x: i as f64,
                    y: j as f64,
                };

                let tile = self.tiles[(j, i)].clone();

                // println!("-> Tile({i},{j}): {:?}", tile.nodes, i = i, j = j);

                for (g, &frag) in tile.to_svg_groups(coord).iter().enumerate() {
                    svg.add_fragment(frag, point, g);
                }
            }
        }

        println!("svg: {}", file_name);
        fs::write(file_name, svg.to_string())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::maze::maze::Maze;

    #[test]
    fn test_surrounding_for_island() {
        let maze = Maze::new(1, 1, false, None);

        let surrounding_tiles = maze.get_connected_titles_for_tile(0, 0);

        let result: Vec<usize> = surrounding_tiles
            .iter()
            .map(|t| if t.is_some() { 1 } else { 0 })
            .collect::<Vec<usize>>();
        assert_eq!(result, vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_surrounding_for_x_0() {
        let maze = Maze::new(9, 9, false, None);

        let expected = vec![
            vec![1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0],
            vec![0, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 0],
            vec![0, 1, 0, 1, 0, 1, 1, 0, 1, 1, 0, 1],
            vec![0, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1],
            vec![0, 0, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1],
            vec![1, 0, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1],
        ];
        for i in 0..6 {
            println!("=> ({}, 0)", i);
            let surrounding_tiles = maze.get_connected_titles_for_tile(i, 0);

            let result: Vec<usize> = surrounding_tiles
                .iter()
                .map(|t| if t.is_some() { 1 } else { 0 })
                .collect::<Vec<usize>>();
            assert_eq!(result, expected[i]);
        }
    }
}

use std::collections::HashSet;
use std::error::Error;
use std::fs;

use common::consts::NODE_TO_COORD;

use crate::maze::random::random_tile;
use array2d::Array2D;
use common::types::coordinate::{add, CataloguedNet, Coordinate};
use rand_distr::num_traits::ToPrimitive;

use crate::maze::adjacent::get_surrounding_tile_connections;
use crate::maze::tile::Tile;
use crate::renders::fragment::MAP_PATHS;
use crate::renders::svg::Svg;
use common::types::point::{Point, TILE_HEIGHT, TILE_WIDTH};

pub struct Maze {
    pub style: Option<String>,
    nets: Vec<HashSet<[usize; 3]>>,
    width: f64,
    height: f64,
    seed: Option<u64>,
    number_of_columns: usize,
    number_of_rows: usize,
    tiles: Array2D<Tile>,
    boundaries: bool,
}

impl Maze {
    pub fn new(
        number_of_columns: usize,
        number_of_rows: usize,
        randomise: bool,
        boundaries: bool,
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
            boundaries,
        };

        if randomise {
            Self::create_random_grid(&mut obj, number_of_columns, number_of_rows);
        }
        return obj;
    }

    pub fn create_random_grid(&mut self, number_of_columns: usize, number_of_rows: usize) {
        // Seed some random tiles that don't touch each other or the boundaries

        for j in 0..number_of_rows {
            for i in 0..number_of_columns {
                // Don't need to create twice
                if self.tiles[(j, i)].initialised {
                    continue;
                }

                let tile = self.get_random_tile(i, j);
                //println!("Tile({i},{j}): {:?}", tile.nodes);
                self.tiles[(j, i)] = tile;

                self.seed = match self.seed {
                    Some(s) => Some(s.wrapping_add(1)),
                    _ => None,
                };
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
        //     if nets.len() == 2 {
        //         // We can remove this net, since it is just a mini-circle
        //         // self.nets.remove(n);
        //     }
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
    // fn update_network(&mut self, tile: &Tile, i: usize, j: usize) {
    //     let tile_nets = tile.nodes.clone();
    //
    //     let mapping = NODE_TO_COORD[(i + (3 * (j % 2))) % 6];
    //
    //     for net in tile_nets {
    //         let tile_sub_nets: HashSet<[usize; 3]> = net
    //             .iter()
    //             .map(|node| [i, j, node.to_usize().unwrap_or_default()])
    //             .collect();
    //
    //         // For this sub net of the current tile, find which nets they are part of
    //         let mut net_indexes: Vec<usize> = Vec::new();
    //
    //         for (n, nets) in self.nets.iter().enumerate() {
    //             for tile_sub_net in tile_sub_nets.iter() {
    //                 // Find adjacent tile so we can check if it is part of an existing net
    //                 let node_idx = tile_sub_net[2];
    //                 let coord = mapping[node_idx - 1];
    //                 //
    //                 let x = add(i, coord.0);
    //                 let y = add(j, coord.1);
    //
    //                 match (x, y) {
    //                     (Some(x), Some(y)) => {
    //                         if nets.contains(&[x, y, node_idx]) && !net_indexes.contains(&n) {
    //                             net_indexes.push(n);
    //                             //break;
    //                         }
    //                     }
    //                     _ => {}
    //                 }
    //             }
    //         }
    //         if net_indexes.len() == 1 {
    //             // Append to an existing net
    //             let n = net_indexes[0];
    //             self.nets[n].extend(tile_sub_nets);
    //         } else if net_indexes.len() == 0 {
    //             // Add a new net
    //             self.nets.push(tile_sub_nets);
    //         } else if net_indexes.len() > 1 {
    //             // Since this tile's subnet was part of 2 or more nets, we can say they are really
    //             // the same net. So we need to merge them.
    //
    //             let n = net_indexes[0];
    //             // Keep the net[n] and merge the other nets into it
    //             let r = net_indexes[1..].to_vec();
    //
    //             let mut new_net = HashSet::new();
    //             // Add in the bridging net
    //             new_net.extend(tile_sub_nets);
    //             // Add the rest of the nets
    //             for i in net_indexes {
    //                 new_net.extend(self.nets[i].clone());
    //             }
    //
    //             self.nets[n] = new_net; // The new merged net
    //
    //             // Remove the rest of the nets since they are now merged.
    //             // Note: This won't have very good performance. It might be the other nets are always towards the end of the list though
    //             // which will help.
    //             for i in r.iter().rev() {
    //                 self.nets.remove(*i);
    //             }
    //         }
    //     }
    // }

    fn get_tile_relative_position(&self, pos: (usize, usize), rel: (i32, i32)) -> Option<&Tile> {
        // Out of bounds, Array2D will return None if out of bounds.

        let x = add(pos.0, rel.0);
        let y = add(pos.1, rel.1);

        let result = match (x, y) {
            (Some(x), Some(y)) => self.tiles.get(y, x),
            _ => None, // Out of bounds, not possible tile
        };
        return result;
    }

    fn get_connected_titles_for_tile(&self, i: usize, j: usize) -> [Option<&Tile>; 12] {
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

        return NODE_TO_COORD[k % 6].map(|coord| self.get_tile_relative_position((i, j), coord));
    }

    fn get_random_tile(&mut self, i: usize, j: usize) -> Tile {
        // Test the new way to draw spanning two cell

        //println!("\n>> ({i},{j})");
        let surrounding_tiles = self.get_connected_titles_for_tile(i, j);
        //print_surrounding(&surrounding_tiles);

        let mask = get_surrounding_tile_connections(&surrounding_tiles);
        //println!("MASK ({i},{j}) {:06}", mask);
        let nets = random_tile(mask, self.seed, false);

        let tile = Tile::new(nets, true);

        //self.update_network(&tile, i, j);

        return tile;
    }

    fn get_any_random_tile(&mut self) -> Tile {
        let nets = random_tile(222222, self.seed, true);

        self.seed = match self.seed {
            Some(s) => Some(s.wrapping_add(1)),
            _ => None,
        };

        let tile = Tile::new(nets, true);
        // self.update_network(&tile, i, j);
        return tile;
    }

    pub fn render(self, file_name: String) -> Result<(), Box<dyn Error>> {
        let margin = 5.0;
        let w = (self.number_of_columns + 1) as f64 * TILE_WIDTH / 2.0 + 5.0 * margin;
        let h = self.number_of_rows as f64 * TILE_HEIGHT + 5.0 * margin;

        // Create header and so on
        let mut svg = Svg::new(w, h);

        let mut all_catorised_nets: CataloguedNet = CataloguedNet::new();

        for j in 0..self.number_of_rows {
            for i in 0..self.number_of_columns {
                let coord: Coordinate = Coordinate { i, j };

                let tile = self.tiles[(j, i)].clone();
                let surrounding_tiles = self.get_connected_titles_for_tile(i, j);

                // First, we build a list of nets (masks) and hidden nets at each coordinate
                // Then we use the still visible ones to build the SVG
                let cat_nets = tile.generate_hidden_and_visible_nets(
                    coord,
                    self.boundaries,
                    &surrounding_tiles,
                );
                all_catorised_nets.extend(cat_nets);
            }
        }

        for coordinate_and_net in all_catorised_nets.visible.iter() {
            // We don't want to draw the hidden nets
            if all_catorised_nets.hidden.contains(coordinate_and_net) {
                continue;
            }
            let frag = MAP_PATHS.get(&coordinate_and_net.net);
            if frag.is_none() {
                continue;
            }

            let point: Point = Point {
                x: coordinate_and_net.coordinate.i as f64,
                y: coordinate_and_net.coordinate.j as f64,
            };

            svg.add_fragment(frag.unwrap(), point, 1);
        }

        let _ = fs::write(file_name, svg.to_string()).expect("Unable to write file");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::maze::maze::Maze;

    #[test]
    fn test_surrounding_for_island() {
        let maze = Maze::new(1, 1, false, false, None);

        let surrounding_tiles = maze.get_connected_titles_for_tile(0, 0);

        let result: Vec<usize> = surrounding_tiles
            .iter()
            .map(|t| if t.is_some() { 1 } else { 0 })
            .collect::<Vec<usize>>();
        assert_eq!(result, vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_surrounding_for_x_0() {
        let maze = Maze::new(9, 9, false, false, None);

        let expected = vec![
            vec![1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0],
            vec![0, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 0],
            vec![0, 1, 0, 1, 0, 1, 1, 0, 1, 1, 0, 1],
            vec![0, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1],
            vec![0, 0, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1],
            vec![1, 0, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1],
        ];
        for i in 0..6 {
            let surrounding_tiles = maze.get_connected_titles_for_tile(i, 0);

            let result: Vec<usize> = surrounding_tiles
                .iter()
                .map(|t| if t.is_some() { 1 } else { 0 })
                .collect::<Vec<usize>>();
            assert_eq!(result, expected[i]);
        }
    }
}

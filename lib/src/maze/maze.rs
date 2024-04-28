use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;

use array2d::Array2D;
use rand::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand_chacha::ChaCha8Rng;
use rand_distr::num_traits::ToPrimitive;
use usvg;
use usvg::fontdb;

#[cfg(build = "debug")]
use common::bit_utils::print_nets;
use common::consts::{N1, N2, N3, N4, N5, N6, NODE_TO_COORD, NX};
use common::types::bits;
use common::types::coordinate::{add, CataloguedNet, Coordinate, CoordinateAndNet};
use common::types::point::{Point, TILE_HEIGHT, TILE_WIDTH};

use crate::maze::adjacent::get_surrounding_tile_connections;
use crate::maze::maze::FileType::{PNG, SVG};
use crate::maze::random::random_tile;
use crate::maze::tile::Tile;
use crate::renders::fragment::MAP_PATHS;
use crate::renders::svg::Svg;

#[derive(Eq, PartialEq)]
pub enum FileType {
    SVG,
    PNG,
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

        let mut rng = match self.seed {
            Some(s) => ChaCha8Rng::seed_from_u64(s),
            None => ChaCha8Rng::from_rng(thread_rng()).unwrap(),
        };

        let mut indices: Vec<(usize, usize)> = (0..number_of_rows)
            .flat_map(|j| (0..number_of_columns).map(move |i| (j, i)))
            .collect();

        // Add the tiles in random order, otherwise the prevoius tile will influence the next
        indices.shuffle(&mut rng);

        for (j, i) in indices {
            // Don't need to create twice
            if self.tiles[(j, i)].initialised {
                continue;
            }

            self.tiles[(j, i)] = self.get_random_tile(i, j);

            self.seed = match self.seed {
                Some(s) => Some(s.wrapping_add(1)),
                _ => None,
            };
        }

        self.width = (number_of_columns + 1) as f64 * TILE_WIDTH / 2.0;
        self.height = number_of_rows as f64 * TILE_HEIGHT;
    }

    pub fn remove_cycles(&mut self) {
        println!("Removing cycles ...");
        // We can do this by removing any tile that is part of a cycle
        let mut visited: HashSet<(usize, usize, usize)> = HashSet::new();

        for net in self.nets.iter() {
            let mut cycle = false;
            for [i, j, k] in net.iter() {
                //println!("@ {}, {}, {}", i, j, k);
                if visited.contains(&(*j, *i, *k)) {
                    cycle = true;
                    //println!("->: {}, {}, {}", i, j, k);
                    break;
                }
                visited.insert((*j, *i, *k));
            }

            if cycle {
                // Break the cycle
            }
        }
    }

    pub fn reduce_nets(&mut self, lower_limit: usize) {
        self.sort_nets_by_length();
        self.remove_short_nets(lower_limit);

        // Create a hashSet of the nets, but used just i,j coordinates as the keys
        let coord_maps = self.create_coord_maps();

        // NOTE: Do I only need get intersections one at a time, since we are merging them?
        // But faster to get them all at once, and then not use the later ones we no longer
        // need

        // We can do the below until there is only none list
        // Alter node and track which nets can be merged, since we need to do it on one pass?
        let shared_coords = Self::get_intersections(coord_maps);

        for ((n, m), intersections) in shared_coords.iter() {
            // Must always have at least one element

            // We could loop through, looking for any intersections that are exactly 2
            let mut found = false;
            for (i, j) in intersections {
                let tiles_at_intersections = self.tiles.get(*j, *i).unwrap();
                if tiles_at_intersections.nodes.len() == 2 {
                    //println!(" {n} \\[UndirectedEdge] {m} ,");
                    // We can merge these two nets in a simple way
                    self.tiles
                        .set(*j, *i, Tile::new(vec![tiles_at_intersections.bits], true));
                    found = true;

                    // TODO: Joins nets! // But this will change all the other join indexes
                    // Need a way to do it all at once!
                    break;
                }
            }
            if !found {
                // We can't merge these nets in a simple way
                println!("Tile: ({m},{n}){:?}", intersections);
                // TODO: Implement more complex joins ...
            }

            // If I merge one, then all other mergers will be need to be updated
            // Example:
            // If 1/2 merged then 1/3 & 2/3 will effectively merged too, and so on. But we don't need to scan, so that's ok
            // thus, skip all 2/n provided joins if there is a 1/n, and I will iterate over a 1/n before a 2/n
        }

        //println!("nets  {:?}", self.nets);
        //println!("tiles  {:?}", self.tiles);
        // println!("Shared coords: {:?}", shared_coords);

        println!("\nWe have disjoint {:?} nets:", self.nets.len());

        // for (n, nets) in self.nets.iter_mut().enumerate() {
        //     println!("{n} -> {}, {:?}", nets.len(), nets);
        // }

        /*

         net_1 = [[i,j,k], [i,j,k], <n>] .. these are sets of tiles that are connected.
         net_2 = [[i,j,k], [i,j,k], <m>]

         Take the shortest net, and iterate over it looking for the first adjacent tile that is not part of the net.
         Then replace the matching tile such that the nets are connected and move the short net over to the larger net.

         Repeat until we only have one net.

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
    }

    fn get_intersections(
        coord_maps: Vec<HashMap<(usize, usize), Vec<usize>>>,
    ) -> HashMap<(usize, usize), HashSet<(usize, usize)>> {
        let mut shared_coords: HashMap<(usize, usize), HashSet<(usize, usize)>> = HashMap::new();

        // <set #, set connected nodes >
        let mut node_to_set: Vec<HashSet<usize>> = vec![];

        for (n, coords_n) in coord_maps.iter().enumerate() {
            let n_keys: HashSet<(usize, usize)> = coords_n.keys().cloned().collect();
            for (m, coords_m) in coord_maps.iter().enumerate() {
                if m <= n {
                    continue; // It is symmetric!
                }

                // If n and m are in the same set, then we don't need to check them since they have
                // already have an intersection via some intermediary set.
                // For example: If I have some joins on say, 1/2 and 2/3 then I don't need to check 2/3
                let index_n = node_to_set.iter().position(|x| x.contains(&n));
                let index_m = node_to_set.iter().position(|x| x.contains(&m));
                if index_n.is_some() && index_n == index_m {
                    continue;
                }

                // Find any intersections between the two sets, i.e. where different nets share the same tile
                let m_keys: HashSet<(usize, usize)> = coords_m.keys().cloned().collect();
                let intersecting: HashSet<(usize, usize)> =
                    n_keys.intersection(&m_keys).cloned().collect();

                if !intersecting.is_empty() {
                    // Some intersections have been found!
                    match (index_n, index_m) {
                        (Some(index_n), Some(index_m)) => {
                            // These two sets are connected, so we can merge them
                            // None: `m > n` is always true.
                            let (left, right) = node_to_set.split_at_mut(index_m);
                            let set_n = &mut left[index_n];
                            let set_m = &right[0];
                            set_n.extend(set_m.iter());
                            node_to_set.remove(index_m);

                            shared_coords.insert((n, m), intersecting);
                        }
                        (Some(index_n), None) => {
                            // Add m to n's set
                            let set_n = node_to_set.get_mut(index_n).unwrap();
                            set_n.insert(m);
                            shared_coords.insert((n, m), intersecting);
                        }
                        (None, Some(index_m)) => {
                            // Add n to m's set
                            let set_m = node_to_set.get_mut(index_m).unwrap();
                            set_m.insert(n);
                            shared_coords.insert((n, m), intersecting);
                        }
                        _ => {
                            // Create a new set
                            node_to_set.push(vec![n, m].into_iter().collect());
                            shared_coords.insert((n, m), intersecting);
                        }
                    }
                }
            }
        }
        return shared_coords;
    }

    fn create_coord_maps(&mut self) -> Vec<HashMap<(usize, usize), Vec<usize>>> {
        let mut coord_maps: Vec<HashMap<(usize, usize), Vec<usize>>> = vec![]; //HashMap::new();
        for (n, nets) in self.nets.iter().enumerate() {
            for [i, j, k] in nets.iter() {
                let map = coord_maps.get_mut(n);
                if map.is_none() {
                    coord_maps.push(HashMap::new());
                }
                let map = coord_maps.get_mut(n).unwrap();
                let entry = map.entry((*i, *j)).or_insert(vec![]);
                entry.push(*k);
            }
        }
        // for set in coord_maps.iter() {
        //     //println!("SET");
        //     for (coord, nets) in set.iter() {
        //         //  println!("{:?} -> {:?}", coord, nets);
        //     }
        // }
        return coord_maps;
    }

    fn sort_nets_by_length(&mut self) {
        // Sort such that the last net is the shortest, we will be deleting them as we go
        self.nets.sort_by(|a, b| (&b.len()).cmp(&a.len()));
    }

    fn remove_short_nets(&mut self, lower_limit: usize) {
        // Strip out small nets
        self.nets.retain(|nets| {
            // Remove small nets
            if nets.len() > lower_limit {
                return true;
            }
            // Of the form [(i,j,4), (...), ... ]
            for [i, j, n] in nets.iter() {
                //
                let tile = self.tiles.get(*j, *i);

                if tile.is_some() {
                    let reduced_tile = tile
                        .unwrap()
                        .nodes
                        .iter()
                        .filter(|&b| {
                            let mask = 1u32 << n;
                            return mask & *b != mask;
                        })
                        .map(|b| *b)
                        .collect::<Vec<u32>>();

                    let _ = self.tiles[(*j, *i)] = Tile::new(reduced_tile, true);
                }
            }
            return false;
        });
    }

    fn update_network(&mut self, tile: &Tile, i: usize, j: usize) {
        let tile_nets = tile.nodes.clone();

        let mapping = NODE_TO_COORD[(i + (3 * (j % 2))) % 6];

        for net in tile_nets {
            // TODO: Need to transform this into bits rather than use node indexes
            let tile_sub_nets: HashSet<[usize; 3]> = vec![N1, N2, N3, N4, N5, N6]
                .iter()
                .filter(|&node| net & *node == *node)
                .map(|&node| [i, j, node.trailing_zeros().to_usize().unwrap()])
                .collect();

            // For this sub net of the current tile, find which nets they are part of
            let mut net_indexes: Vec<usize> = Vec::new();

            for (n, nets) in self.nets.iter().enumerate() {
                for tile_sub_net in tile_sub_nets.iter() {
                    // Find adjacent tile so we can check if it is part of an existing net
                    let node_idx = tile_sub_net[2];
                    let coord = mapping[node_idx];
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
        let surrounding_tiles = self.get_connected_titles_for_tile(i, j);
        let mask = get_surrounding_tile_connections(&surrounding_tiles);

        // Try simple, then complex options?
        let mut nets = random_tile(mask, self.seed, true);
        if mask != 0 && nets.len() == 0 {
            // If we can't find a tile that fits, try again without the mask
            nets = random_tile(mask, self.seed, false);
        }

        let tile = Tile::new(nets, true);
        self.update_network(&tile, i, j);
        return tile;
    }

    #[cfg(build = "debug")]
    fn print_out_networks(&self) {
        println!("");
        let rows: Vec<_> = self.tiles.rows_iter().collect();

        let text = rows
            .into_iter()
            .rev() // Reverse the rows
            .map(|row| {
                row.map(|t| format!("{:^10}", print_nets(&t.nodes)))
                    .collect::<Vec<_>>()
                    .join("|")
            })
            .collect::<Vec<_>>()
            .join("\n");
        println!("{}\n", text);
    }

    pub fn render(&mut self, file_name: String, file_type: FileType) -> Result<(), Box<dyn Error>> {
        //let start = Instant::now();
        #[cfg(build = "debug")]
        self.print_out_networks();

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

        // println!("render took (1) {:?}", start.elapsed());
        // let start = Instant::now();

        for coordinate_and_net in all_catorised_nets.visible.iter() {
            // We don't want to draw the hidden nets
            let masked_net: CoordinateAndNet = CoordinateAndNet {
                // IN & WA
                net: coordinate_and_net.net & NX,
                coordinate: coordinate_and_net.coordinate,
            };

            if all_catorised_nets.hidden.contains(&masked_net) {
                continue;
            }

            let net = bits::transform_tile(coordinate_and_net.net, coordinate_and_net.coordinate);

            let frag = MAP_PATHS.get(&net);
            if frag.is_none() {
                continue;
            }
            if frag.unwrap().is_empty() {
                continue;
            }

            let point: Point = Point {
                x: coordinate_and_net.coordinate.i as f64,
                y: coordinate_and_net.coordinate.j as f64,
            };
            svg.add_fragment(frag.unwrap(), point, 1);
        }

        //println!("render took (2) {:?}", start.elapsed());
        //let start = Instant::now();

        match file_type {
            SVG => return self.save_as_svg(svg.to_string(false), file_name),
            PNG => return self.save_as_png(svg.to_string(false).as_str(), file_name),
        }
    }

    pub fn save_as_svg(
        &mut self,
        content: String,
        file_name: String,
    ) -> Result<(), Box<dyn Error>> {
        let _ = fs::write(format!("{}.svg", file_name), content).expect("Unable to write file");
        return Ok(());
    }

    pub fn save_as_png(
        &mut self,
        svg_content: &str,
        file_name: String,
    ) -> Result<(), Box<dyn Error>> {
        let input = format!("{}.png", file_name);
        let mut fontdb = fontdb::Database::new();
        fontdb.load_system_fonts();
        let tree = usvg::Tree::from_str(svg_content, &usvg::Options::default(), &fontdb).unwrap();

        let zoom = 1f32;
        let pixmap_size = tree.size().to_int_size().scale_by(zoom).unwrap();
        println!("Size w:{} h:{}", pixmap_size.width(), pixmap_size.height());

        let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();

        let render_ts = tiny_skia::Transform::from_scale(zoom, zoom);

        // This is very slow when running on debug builds !!!
        resvg::render(&tree, render_ts, &mut pixmap.as_mut());

        pixmap.save_png(&input).unwrap();

        return Ok(());
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

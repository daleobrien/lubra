use std::fmt;

use rand_distr::num_traits::ToPrimitive;

use common::bit_utils::{sort_upper_bits, to_6bit_binary};
use common::consts::{IN, NL, WA};
use common::nodes::get_connected_coords_at_coords;
use common::types::bits;
use common::types::coordinate::{CataloguedNet, ComplexNet, Coordinate, CoordinateAndNet};

use crate::maze::generated::nets_adjacent::ADJ_CHECKS;
use crate::maze::maze::touching::clear_in_same_tile;
use crate::renders::generated::fragment::MAP_PATHS;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Tile {
    pub nodes: Vec<usize>,
    pub initialised: bool,
    pub bits: usize,
}

impl Tile {
    pub fn new(nodes: Vec<usize>, initialised: bool) -> Tile {
        let bits = nodes.iter().fold(0, |a, b| a | b);
        Tile {
            nodes,
            initialised,
            bits,
        }
    }

    pub fn get_first_nodes(&self) -> Vec<usize> {
        // Get the first set bit from each node
        self.nodes
            .iter()
            .map(|n| n.trailing_zeros().to_usize().unwrap())
            .collect()
    }

    pub fn to_string_vec(self) -> String {
        if self.nodes.is_empty() {
            return "-".to_string();
        }
        let s: String = self
            .nodes
            .iter()
            .map(|n| to_6bit_binary(n, true))
            .collect::<Vec<String>>()
            .join(", ");

        format!("[{s}]", s = s.replace("\"", ""))
    }

    /// The fragments to draw for this tile, and the neighbouring nets they replace.
    ///
    /// With `corner_hooks` false, cross-tile fragments that reach a tile across a vertex
    /// (see `is_corner_hook`) are not used; the renderer turns them off for a tile when
    /// they would claim a net that another fragment already draws.
    pub fn generate_hidden_and_visible_nets(
        self,
        coordinate: Coordinate,
        boundaries: bool,
        surrounding_tiles: &[Option<&Tile>; 12],
        debug: bool,
        corner_hooks: bool,
    ) -> CataloguedNet {
        //

        let mut render_nets = CataloguedNet::new();

        // if debug {
        //     println!("NETS: {coordinate} -> {:?}", print_nets(&self.nodes));
        // }
        let tile_nets = self.nodes.clone();
        for net in self.nodes {
            // Now we need to look for related tiles ...

            // if debug {
            //     println!("\nNET_: {coordinate} -> {:?}", print_bits(net));
            // }
            // This needs to return a net to use, and a list of adjacent nets to remove.
            let complex_net = Self::find_best_path(
                coordinate,
                &surrounding_tiles,
                &net,
                &tile_nets,
                debug,
                corner_hooks,
            );

            if let Some(complex_net) = complex_net {
                render_nets.add(complex_net);
            } else {
                // Simple path
                let path = MAP_PATHS.get(&(net as u64));

                // A simple tile with no adjacent tiles to worry about?
                if path.is_some() {
                    render_nets
                        .visible
                        .push(CoordinateAndNet::new(coordinate, net));
                }
            }
        }

        if boundaries {
            let flipped = (coordinate.j + coordinate.i) % 2 == 1;
            let bits = if flipped { WA | IN } else { WA };
            let wall = MAP_PATHS.get(&(bits as u64));
            if wall.is_some() {
                render_nets
                    .visible
                    .push(CoordinateAndNet::new(coordinate, bits));
            }
        }

        render_nets
    }

    fn find_best_path(
        coordinate: Coordinate,
        surrounding_tiles: &&[Option<&Tile>; 12],
        net: &usize,
        tile_nets: &[usize],
        debug: bool,
        corner_hooks: bool,
    ) -> Option<ComplexNet> {
        let list_of_adj_and_nets = ADJ_CHECKS.get(&(*net as u64));

        let mut complex_net: ComplexNet = ComplexNet::new();

        if let Some(list_of_adj_and_nets) = list_of_adj_and_nets {
            // let x = list_of_adj_and_nets
            //     .unwrap()
            //     .iter()
            //     .map(|&x| (*x)[0].2)
            //     .collect::<Vec<_>>();
            // if debug {
            //     println!(
            //         "find_best_path: {}  -> \"{}\" [{:?}]",
            //         coordinate,
            //         print_bits(*net),
            //         x.iter().map(|n| print_bits(*n)).collect::<Vec<String>>()
            //     );
            // }
            for &adj_and_nets in list_of_adj_and_nets.iter() {
                // Neighbour indices 0-2 are the tiles across the vertices.
                if !corner_hooks && adj_and_nets.iter().any(|&(index, _, _)| index < 3) {
                    continue;
                }
                // A fragment that redraws a neighbouring junction (frag_hook_fork) reshapes
                // both nets inside the space they already take up. That is only known
                // to be clear of the other nets on the two tiles when those were drawn
                // clear of the nets it replaces, not crossing them.
                let crowded = adj_and_nets
                    .iter()
                    .filter(|&&(_, _, junction)| junction.count_ones() > 2)
                    .any(|&(index, _, junction)| {
                        let beside = |nets: &[usize], replaced: usize| {
                            nets.iter().any(|&other| {
                                other != replaced && !clear_in_same_tile(replaced, other)
                            })
                        };
                        beside(tile_nets, *net)
                            || surrounding_tiles[index]
                                .is_some_and(|tile| beside(&tile.nodes, junction))
                    });
                if crowded {
                    continue;
                }
                if Self::all_match(surrounding_tiles, adj_and_nets, debug) {
                    // For each subnet, merge them into one net
                    let adj_net = adj_and_nets
                        .iter()
                        .map(|(_, _m, n)| n)
                        .fold(0, |a, b| a << 6 | b);

                    let mut new_bits = (adj_net << 6) | net;

                    new_bits = sort_upper_bits(new_bits);

                    // From here on, the bits are in the correct orientation rather then logical.
                    let bits = new_bits;

                    if MAP_PATHS.contains_key(&(bits as u64)) {
                        // if debug {
                        //     println!(
                        //         "__D : {c}  -> \"{b}\"",
                        //         c = coordinate,
                        //         b = to_6bit_binary(&bits, false),
                        //     );
                        // }

                        // Adjacent tiles masks
                        let adj_nets_1 = (bits >> 6) & NL;
                        let adj_nets_2 = (bits >> 12) & NL;
                        let adj_nets_3 = (bits >> 18) & NL;
                        let adj_nets_4 = (bits >> 24) & NL;

                        for adj_net in [adj_nets_1, adj_nets_2, adj_nets_3, adj_nets_4] {
                            if adj_net != 0 {
                                let node_number =
                                    ((net & NL) & adj_net).trailing_zeros().to_usize().unwrap();

                                // -------
                                // if debug {
                                //     println!(
                                //         "ADJ : {c}  -> {b} {adj_nets_1}",
                                //         c = coordinate,
                                //         b = print_bits(adj_net),
                                //         adj_nets_1 = print_bits(adj_nets_1),
                                //     );
                                // }
                                Self::add_node_to_hidden(
                                    coordinate,
                                    &mut complex_net,
                                    adj_net,
                                    node_number,
                                    debug,
                                );
                            }
                        }

                        complex_net.complex_net = CoordinateAndNet::new(coordinate, bits);
                        return Some(complex_net);
                    }
                }
            }
        }
        None
    }

    fn add_node_to_hidden(
        coordinate: Coordinate,
        complex_net: &mut ComplexNet,
        adj_net: usize,
        node_number: usize,
        _debug: bool,
    ) {
        let coord = get_connected_coords_at_coords(coordinate, node_number);

        if let Some(coord) = coord {
            let index_as_mask = 1 << node_number;
            if index_as_mask.to_usize().unwrap() & adj_net != 0 {
                // Also, is node_number in adj_net
                // if debug {
                //     println!(
                //         "ADD : {}  -> node # {n} {b} {x}",
                //         coord.unwrap(),
                //         n = node_number + 1,
                //         b = print_bits(adj_net),
                //         x = adj_net & index_as_mask != 0
                //     );
                // }
                complex_net
                    .hidden_neighbours
                    .insert(CoordinateAndNet::new(coord, adj_net));
            }
        }
    }

    fn all_match(
        surrounding_tiles: &[Option<&Tile>; 12],
        adj_and_nets: &[(usize, bool, usize)],
        _debug: bool,
    ) -> bool {
        let mut all_matched = true;

        // All the adjacent tiles must contain the net to be considered hidden.
        for (adj_idx, same, pattern) in adj_and_nets.iter() {
            let &adj_tile = &surrounding_tiles[*adj_idx];
            // if debug && adj_tile.is_some() {
            //     println!(
            //         "_AM : {adj_idx} -> {same} {pattern} U {adj_tile} ?",
            //         adj_idx = adj_idx,
            //         same = same,
            //         pattern = print_bits(*pattern),
            //         adj_tile = print_bits((adj_tile.unwrap()).bits)
            //     );
            // }

            //
            match adj_tile {
                Some(a) => {
                    if *same != a.contain_net(*pattern) {
                        all_matched = false;
                    }
                }
                _ => {
                    all_matched = false;
                }
            }
        }
        // if debug {
        //     println!("_AM : {all_matched}", all_matched = all_matched);
        // }
        all_matched
    }

    /// Whether `key` is a cross-tile fragment that replaces a net in the tile across a
    /// vertex (`frag_corner_hook`, `frag_vertex_s`): one of its neighbour nets meets the
    /// tile's own net only at a vertex (node 1-3).
    pub fn reaches_across_vertex(key: usize) -> bool {
        let own = key & NL;
        (1..5).any(|k| {
            let shared = (key >> (6 * k)) & NL & own;
            shared != 0 && shared.trailing_zeros() < 3
        })
    }

    /// Whether `key` is a cross-tile fragment that redraws a junction in the tile
    /// across an edge (`frag_hook_fork`): its neighbour net has three nodes or more.
    pub fn redraws_junction(key: usize) -> bool {
        ((key >> 6) & NL).count_ones() > 2
    }

    pub fn rotate_cw(&self) -> Vec<usize> {
        self.nodes
            .iter()
            .map(|n| bits::rotate_bits_345_left_and_012_right(*n))
            .collect()
    }

    pub fn rotate_ccw(&self) -> Vec<usize> {
        self.nodes
            .iter()
            .map(|n| bits::rotate_bits_345_right_and_012_left(*n))
            .collect()
    }

    pub fn intersects(&self, other: &Tile) -> bool {
        other.bits & self.bits != 0
    }

    pub fn contains(&self, node_mask: usize) -> bool {
        node_mask & self.bits != 0
    }

    pub fn contain_net(&self, net_mask: usize) -> bool {
        self.nodes.contains(&net_mask)
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.initialised {
            write!(f, "{:?}", self.nodes)
        } else {
            write!(f, "uninitialised")
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.initialised {
            write!(f, "{:?}", self.nodes)
        } else {
            write!(f, "?")
        }
    }
}

#[cfg(test)]
mod tests {
    /// The lookup tables must be keyed by u64: phf hashes a usize at the machine's
    /// width, so tables built on a 64-bit machine find nothing on 32-bit WebAssembly.
    #[test]
    fn test_lookup_tables_are_keyed_by_u64() {
        use super::{ADJ_CHECKS, MAP_PATHS};
        let _: &phf::Map<u64, &str> = &MAP_PATHS;
        type Checks = &'static [&'static [(usize, bool, usize)]];
        let _: &phf::Map<u64, Checks> = &ADJ_CHECKS;
        assert!(MAP_PATHS.contains_key(&(common::consts::WA as u64)));
    }

    #[test]
    fn test_reaches_across_vertex() {
        use common::consts::{A1, A2, A3, A4, A5, A6, B3, B4, B5, IN, N1, N2, N3, N4, N5, N6};
        // Corner hook: the same net in the tile across vertex 1.
        assert!(Tile::reaches_across_vertex(N1 | N4 | A1 | A4 | B4));
        assert!(Tile::reaches_across_vertex(IN | N1 | N4 | A1 | A4 | B4));
        // Vertex S: vertex 3 and the other edge in the tile across it.
        assert!(Tile::reaches_across_vertex(N3 | N6 | A3 | A5));
        // Ordinary cross-tile fragments meet their neighbours at an edge.
        assert!(!Tile::reaches_across_vertex(N2 | N6 | A6));
        assert!(!Tile::reaches_across_vertex(N4 | N5 | A1 | A4 | B3 | B5));
        assert!(!Tile::reaches_across_vertex(N5 | N6 | A2 | A6));
        assert!(!Tile::reaches_across_vertex(N1 | N4));
    }

    use std::{assert_eq, println, vec};

    use common::consts::{N1, N2, N3, N4, N5, N6};

    use crate::maze::tile::Tile;

    #[test]
    fn test_cw_rotation() {
        let tile = Tile::new(vec![N1 | N2, N5], true);
        let rotated_nodes = tile.rotate_cw();
        println!("CW: {:#06b} {:#06b}", rotated_nodes[0], rotated_nodes[1]);
        assert_eq!(rotated_nodes, vec![N1 | N3, N6]);
    }

    #[test]
    fn test_ccw_rotation() {
        let tile = Tile::new(vec![N1 | N2, N5], true);
        let rotated_nodes = tile.rotate_ccw();
        assert_eq!(rotated_nodes, vec![N2 | N3, N4]);
    }

    #[test]
    fn test_bits() {
        let tile = Tile::new(vec![N1 | N2, N5], true);
        assert_eq!(tile.bits, N1 | N2 | N5)
    }

    #[test]
    fn test_intersects() {
        let tile1 = Tile::new(vec![N1 | N2, N5], true);
        let tile2 = Tile::new(vec![N2 | N3, N6], true);
        assert!(tile1.intersects(&tile2));

        let tile1 = Tile::new(vec![N1 | N2, N5], true);
        let tile2 = Tile::new(vec![N3, N6], true);
        assert!(!tile1.intersects(&tile2));
    }

    #[test]
    fn test_contains() {
        let tile1 = Tile::new(vec![N1 | N2, N5], true);
        assert!(tile1.contains(N1));
        assert!(tile1.contains(N2));
        assert!(!tile1.contains(N3));
        assert!(!tile1.contains(N4));
        assert!(tile1.contains(N5));
        assert!(!tile1.contains(N6));

        let tile1 = Tile::new(vec![], true);
        assert!(!tile1.contains(N1));
    }

    #[test]
    fn test_contain_net() {
        let tile1 = Tile::new(vec![N1 | N2, N5], true);
        assert!(!tile1.contain_net(N1));
        assert!(!tile1.contain_net(N2));
        assert!(tile1.contain_net(N1 | N2));
        assert!(!tile1.contain_net(N3));
        assert!(!tile1.contain_net(N4));
        assert!(tile1.contain_net(N5));
        assert!(!tile1.contain_net(N6));

        let tile1 = Tile::new(vec![], true);
        assert!(!tile1.contains(N1));
    }

    #[test]
    fn test_is_empty() {
        let tile1 = Tile::new(vec![N1 | N2, N5], true);
        assert!(!tile1.is_empty());

        let tile1 = Tile::new(vec![], true);
        assert!(tile1.is_empty());
    }
}

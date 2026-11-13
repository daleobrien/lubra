//! Blank-area filling: grows tracks into the space left after reduction.

use rand::seq::SliceRandom;
use rand::Rng;
use rand_chacha::ChaCha8Rng;

use super::fill_clearance::DRAWN_CLEAR;

use crate::maze::maze::Maze;
use crate::maze::tile::Tile;

/// A place a new branch can start: tile net `bits` of tile `(i, j)` grows out of its
/// free node `k` into the tile across it.
#[derive(Clone, Copy)]
struct Seed {
    i: usize,
    j: usize,
    bits: usize,
    k: usize,
}

/// The longest branch grown in one go, in tiles. Shorter branches leave room for
/// later ones to fork off them, which fills an area with more than one snaking path.
const MAX_BRANCH: usize = 6;

/// The most tile nets a tile may already hold for a branch to enter it.
const MAX_SHARED: usize = 2;

impl Maze {
    /// Grows new dead-end branches into blank space until no branch of at least
    /// `min_length` tiles fits anywhere (0 does nothing).
    ///
    /// A branch starts at a tile net with one or two nodes and wanders from tile to
    /// tile through free nodes. It only enters tiles that hold at most `MAX_SHARED`
    /// tile nets, never the entrance or exit tile, and only where it is drawn clear of every other track (see
    /// `fill_clearance.rs`), so it fills gaps rather than crossing tracks. Existing dead
    /// ends are extended first, with no minimum; a branch off a two-node tile net
    /// makes a new junction, so it is only kept when it reaches `min_length` tiles.
    ///
    /// Every branch is attached at one end only and never touches a used node, so the
    /// maze stays a single loop-free tree. Choices are drawn from the maze's seed.
    /// `self.nets` is rebuilt afterwards.
    pub fn fill_blank_areas(&mut self, min_length: usize) {
        if min_length == 0 {
            return;
        }
        let mut rng = self.seeded_rng();
        loop {
            let mut grew = false;
            for seed in self.branch_seeds(&mut rng) {
                let needed = if seed.bits.count_ones() == 1 {
                    1
                } else {
                    min_length
                };
                grew |= self.grow_branch(seed, needed, &mut rng);
            }
            if !grew {
                break;
            }
        }
        self.rebuild_nets();
    }

    /// Every place a branch could start, dead ends first, each group shuffled.
    fn branch_seeds(&self, rng: &mut ChaCha8Rng) -> Vec<Seed> {
        let mut leaves = Vec::new();
        let mut tracks = Vec::new();
        for j in 0..self.number_of_rows {
            for i in 0..self.number_of_columns {
                if self.is_terminal(i, j) {
                    continue;
                }
                for &bits in &self.tiles[(j, i)].nodes {
                    let group = match bits.count_ones() {
                        1 => &mut leaves,
                        2 => &mut tracks,
                        _ => continue,
                    };
                    for k in 0..6 {
                        if self.can_grow_out_of(i, j, bits, k) {
                            group.push(Seed { i, j, bits, k });
                        }
                    }
                }
            }
        }
        leaves.shuffle(rng);
        tracks.shuffle(rng);
        leaves.extend(tracks);
        leaves
    }

    /// Whether tile net `bits` of tile `(i, j)` can grow out of node `k`: the node is
    /// free, the tile across it has room, and neither the longer tile net nor the new
    /// end across the node would be drawn touching another track.
    fn can_grow_out_of(&self, i: usize, j: usize, bits: usize, k: usize) -> bool {
        let Some((bi, bj)) = self.tile_across(i, j, k) else {
            return false;
        };
        !self.is_terminal(bi, bj)
            && self.tiles[(bj, bi)].nodes.len() <= MAX_SHARED
            && self.node_is_free(i, j, k)
            && self.drawn_clear(i, j, bits | (1 << k), Some(bits), None)
            && self.drawn_clear(bi, bj, 1 << k, None, Some((k, bits)))
    }

    /// Whether tile net `net` can be drawn on tile `(i, j)` without touching any
    /// track it is not joined to, on this tile or the twelve around it.
    ///
    /// `replacing` is the tile net of this tile that `net` takes the place of.
    /// `joining` is `(position, tile net)` for a neighbouring tile net that is about
    /// to be joined to `net`, so does not count either.
    fn drawn_clear(
        &self,
        i: usize,
        j: usize,
        net: usize,
        replacing: Option<usize>,
        joining: Option<(usize, usize)>,
    ) -> bool {
        let clear = |position: usize, other: usize| {
            DRAWN_CLEAR[position][net & 63] & (1 << (other & 63)) != 0
        };
        let same_tile = self.tiles[(j, i)]
            .nodes
            .iter()
            .all(|&other| Some(other) == replacing || clear(12, other));
        same_tile
            && self
                .get_connected_titles_for_tile(i, j)
                .iter()
                .enumerate()
                .all(|(position, tile)| {
                    tile.is_none_or(|tile| {
                        tile.nodes.iter().all(|&other| {
                            // Positions 0-5 are the tiles across nodes 0-5.
                            let joined = position < 6
                                && net & (1 << position) != 0
                                && (other & (1 << position) != 0
                                    || joining == Some((position, other)));
                            joined || clear(position, other)
                        })
                    })
                })
    }

    /// Grows one branch from `seed`. Returns whether it was kept; a branch shorter
    /// than `needed` tiles is undone.
    fn grow_branch(&mut self, seed: Seed, needed: usize, rng: &mut ChaCha8Rng) -> bool {
        let Seed { i, j, bits, k } = seed;
        // Earlier branches in this pass may have used the seed.
        if !self.tiles[(j, i)].contain_net(bits) || !self.can_grow_out_of(i, j, bits, k) {
            return false;
        }
        let mut undo: Vec<((usize, usize), Tile)> = Vec::new();
        let limit = rng.gen_range(needed.max(2)..=MAX_BRANCH.max(needed));

        // `here` is the tile whose tile net `net` is the end of the branch so far.
        let (mut here, mut net, mut exit) = ((i, j), bits, k);
        let mut length = 0;
        while let Some(next) = self.tile_across(here.0, here.1, exit) {
            self.set_tile(&mut undo, here, |tile_nets| {
                tile_nets.retain(|&t| t != net);
                tile_nets.push(net | (1 << exit));
            });
            self.set_tile(&mut undo, next, |tile_nets| tile_nets.push(1 << exit));
            length += 1;
            here = next;
            net = 1 << exit;
            if length >= limit {
                break;
            }
            let exits: Vec<usize> = (0..6)
                .filter(|&k| self.can_grow_out_of(here.0, here.1, net, k))
                .collect();
            match exits.choose(rng) {
                Some(&k) => exit = k,
                None => break,
            }
        }

        if length < needed {
            for (at, tile) in undo.into_iter().rev() {
                self.tiles[(at.1, at.0)] = tile;
            }
            return false;
        }
        true
    }

    /// Edits the tile nets of the tile at `at`, saving the old tile in `undo`.
    fn set_tile(
        &mut self,
        undo: &mut Vec<((usize, usize), Tile)>,
        at: (usize, usize),
        edit: impl FnOnce(&mut Vec<usize>),
    ) {
        let old = self.tiles[(at.1, at.0)].clone();
        let mut tile_nets = old.nodes.clone();
        edit(&mut tile_nets);
        undo.push((at, old));
        self.tiles[(at.1, at.0)] = Tile::new(tile_nets, true);
    }
}

#[cfg(test)]
mod tests {
    use crate::maze::maze::Maze;
    use crate::maze::tile::Tile;

    /// The number of node positions in use, a measure of how much track is drawn.
    fn used_nodes(maze: &Maze) -> u32 {
        maze.tiles
            .elements_row_major_iter()
            .flat_map(|tile| tile.nodes.iter())
            .map(|tile_net| tile_net.count_ones())
            .sum()
    }

    fn prepared(seed: u64) -> Maze {
        let mut maze = Maze::new(30, 20, true, false, false, Some(seed));
        maze.reduce_nets(1);
        maze.remove_cycles(false);
        maze.trim_dead_ends(2);
        maze
    }

    /// Writes every tile net, alone, on each tile around six centre tiles (one of
    /// each kind) for `tools/clearance.py` to measure. Run with
    /// `CLEARANCE_DIR=<dir> cargo test -p lib dump_clearance_shapes -- --ignored`.
    #[test]
    #[ignore]
    fn dump_clearance_shapes() {
        use crate::maze::maze::FileType;
        use common::consts::NODE_TO_COORD;
        use std::collections::BTreeSet;
        use std::fmt::Write;

        let dir = std::env::var("CLEARANCE_DIR").expect("set CLEARANCE_DIR");
        let mut manifest = String::new();
        let mut tiles = BTreeSet::new();
        for (i, j) in [(3usize, 3usize), (4, 3), (5, 3), (3, 4), (4, 4), (5, 4)] {
            let kind = (i + 3 * (j % 2)) % 6;
            writeln!(manifest, "{kind} 12 {i} {j}").unwrap();
            tiles.insert((i, j));
            for (p, (di, dj)) in NODE_TO_COORD[kind].into_iter().enumerate() {
                let at = ((i as i32 + di) as usize, (j as i32 + dj) as usize);
                writeln!(manifest, "{kind} {p} {} {}", at.0, at.1).unwrap();
                tiles.insert(at);
            }
        }
        std::fs::write(format!("{dir}/manifest.txt"), manifest).unwrap();
        for (i, j) in tiles {
            for net in 1..64 {
                let mut maze = Maze::new(9, 8, false, false, false, None);
                maze.tiles[(j, i)] = Tile::new(vec![net], true);
                maze.rebuild_nets();
                maze.render(format!("{dir}/t{i}_{j}_n{net}"), FileType::SVG)
                    .unwrap();
            }
        }
    }

    #[test]
    fn test_fill_blank_areas_keeps_one_tree_and_adds_track() {
        for seed in [1, 2, 3, 7, 42] {
            let mut maze = prepared(seed);
            let before = used_nodes(&maze);

            maze.fill_blank_areas(3);

            assert_eq!(maze.nets.len(), 1, "seed {seed}");
            assert_eq!(maze.loop_count(), 0, "seed {seed}");
            assert!(used_nodes(&maze) > before, "seed {seed}");
            // The entrance and exit stay plain dead ends.
            assert_eq!(maze.tiles[(0, 0)].nodes.len(), 1, "seed {seed}");
            assert_eq!(maze.tiles[(0, 0)].nodes[0].count_ones(), 1, "seed {seed}");
            assert_eq!(maze.tiles[(19, 29)].nodes.len(), 1, "seed {seed}");
            assert_eq!(maze.tiles[(19, 29)].nodes[0].count_ones(), 1, "seed {seed}");
        }
    }

    #[test]
    fn test_fill_blank_areas_zero_does_nothing() {
        let mut maze = prepared(1);
        let before = used_nodes(&maze);
        maze.fill_blank_areas(0);
        assert_eq!(used_nodes(&maze), before);
    }

    #[test]
    fn test_fill_blank_areas_is_repeatable() {
        let fill = |seed| {
            let mut maze = prepared(seed);
            maze.fill_blank_areas(3);
            maze.tiles
                .elements_row_major_iter()
                .map(|t| t.nodes.clone())
                .collect::<Vec<_>>()
        };
        assert_eq!(fill(5), fill(5));
    }
}

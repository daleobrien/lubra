//! Blank-area filling: grows tracks into the space left after reduction.

use rand::seq::SliceRandom;
use rand::Rng;
use rand_chacha::ChaCha8Rng;

use super::touching::Meeting;

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

/// How often a step of a branch is allowed to cross another track.
const CROSSING_CHANCE: f64 = 0.35;

/// The longest route a stuck dead end may take to reach an empty tile, in tiles.
const MAX_REACH: usize = 6;

/// The most tracks such a route may cross.
const MAX_CROSSINGS: usize = 3;

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
    /// When nothing more grows, dead ends stuck behind other tracks are routed to
    /// empty tiles past them (`reach_gaps`), and growing starts again.
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
            self.grow_branches(min_length, &mut rng);
            if !self.reach_gaps(&mut rng) {
                break;
            }
        }
        self.rebuild_nets();
    }

    /// Grows branches from every seed until none grows any more.
    fn grow_branches(&mut self, min_length: usize, rng: &mut ChaCha8Rng) {
        loop {
            let mut grew = false;
            for seed in self.branch_seeds(rng) {
                let needed = if seed.bits.count_ones() == 1 {
                    1
                } else {
                    min_length
                };
                grew |= self.grow_branch(seed, needed, rng);
            }
            if !grew {
                break;
            }
        }
    }

    /// Extends dead ends that are stuck behind other tracks into empty tiles beyond
    /// them. Returns whether any dead end was extended.
    ///
    /// Each dead end takes the shortest route it can find to an empty tile, of at most
    /// `MAX_REACH` tiles, passing other plain tracks at most `MAX_CROSSINGS` times. At
    /// each crossing it goes over or under at random. Every step obeys the rules of
    /// `can_grow_out_of`, so the route touches nothing and ends clear of the last
    /// track it crossed.
    fn reach_gaps(&mut self, rng: &mut ChaCha8Rng) -> bool {
        // Without crossings, a branch may not pass under any other track.
        let crossings = if self.crossings { MAX_CROSSINGS } else { 0 };
        let mut dead_ends = Vec::new();
        for j in 0..self.number_of_rows {
            for i in 0..self.number_of_columns {
                if self.is_terminal(i, j) {
                    continue;
                }
                for &bits in &self.tiles[(j, i)].nodes {
                    if bits.count_ones() == 1 {
                        dead_ends.push(((i, j), bits));
                    }
                }
            }
        }
        dead_ends.shuffle(rng);

        let mut reached = false;
        for (at, bits) in dead_ends {
            // An earlier route may have crossed this tile, but never changes a dead end.
            for reach in 1..=MAX_REACH {
                let mut undo = Vec::new();
                if self.route_to_gap(at, bits, reach, crossings, &mut undo, rng) {
                    reached = true;
                    break;
                }
            }
        }
        reached
    }

    /// Extends the dead end `net` of tile `here` by up to `steps` tiles so that it
    /// ends in a tile that was empty, crossing at most `crossings` tracks. Returns
    /// whether it did; if not, the maze is left as it was.
    fn route_to_gap(
        &mut self,
        here: (usize, usize),
        net: usize,
        steps: usize,
        crossings: usize,
        undo: &mut Vec<((usize, usize), Tile)>,
        rng: &mut ChaCha8Rng,
    ) -> bool {
        // Each way out, and whether taking it crosses a track. Plain ways first.
        let mut exits: Vec<(usize, bool)> = (0..6)
            .filter_map(|k| {
                if self.can_grow_out_of(here.0, here.1, net, k, false) {
                    Some((k, false))
                } else if crossings > 0 && self.can_grow_out_of(here.0, here.1, net, k, true) {
                    Some((k, true))
                } else {
                    None
                }
            })
            .collect();
        exits.shuffle(rng);
        exits.sort_by_key(|&(_, crosses)| crosses);

        for (exit, crosses) in exits {
            let Some(next) = self.tile_across(here.0, here.1, exit) else {
                continue;
            };
            let into_gap = self.tiles[(next.1, next.0)].nodes.is_empty();
            let mark = undo.len();
            // Tile nets are drawn in order, so the first passes under the rest.
            let under = crosses && rng.gen_bool(0.5);
            self.set_tile(undo, here, |tile_nets| {
                tile_nets.retain(|&t| t != net);
                if under {
                    tile_nets.insert(0, net | (1 << exit));
                } else {
                    tile_nets.push(net | (1 << exit));
                }
            });
            self.set_tile(undo, next, |tile_nets| tile_nets.push(1 << exit));

            let left = crossings - usize::from(crosses);
            if into_gap
                || (steps > 1 && self.route_to_gap(next, 1 << exit, steps - 1, left, undo, rng))
            {
                return true;
            }
            for (at, tile) in undo.drain(mark..).rev() {
                self.tiles[(at.1, at.0)] = tile;
            }
        }
        false
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
                        if self.can_grow_out_of(i, j, bits, k, false) {
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
    fn can_grow_out_of(&self, i: usize, j: usize, bits: usize, k: usize, may_cross: bool) -> bool {
        let Some((bi, bj)) = self.tile_across(i, j, k) else {
            return false;
        };
        !self.is_terminal(bi, bj)
            && self.tiles[(bj, bi)].nodes.len() <= MAX_SHARED
            && self.node_is_free(i, j, k)
            && self.drawn_clear(i, j, bits | (1 << k), None, may_cross)
            && self.drawn_clear(bi, bj, 1 << k, Some((k, bits)), false)
    }

    /// Whether tile net `net` can be drawn on tile `(i, j)` without touching any
    /// track it is not joined to, on this tile or the twelve around it.
    ///
    /// Tile nets of this tile that share a node with `net` are the one it replaces.
    /// `joining` is `(position, tile net)` for a neighbouring tile net that is about
    /// to be joined to `net`, so does not count either. With `may_cross`, a two-node
    /// `net` may also pass cleanly over one other two-node tile net.
    fn drawn_clear(
        &self,
        i: usize,
        j: usize,
        net: usize,
        joining: Option<(usize, usize)>,
        may_cross: bool,
    ) -> bool {
        let mut crossings = 0;
        self.meetings(i, j, net)
            .into_iter()
            .all(|(other, position, how)| match how {
                Meeting::Clear => true,
                // Only one plain track over one other plain track.
                Meeting::Crossing if may_cross => {
                    crossings += 1;
                    crossings == 1 && net.count_ones() == 2 && other.bits.count_ones() == 2
                }
                _ => joining == Some((position, other.bits)),
            })
    }

    /// Grows one branch from `seed`. Returns whether it was kept; a branch shorter
    /// than `needed` tiles is undone.
    fn grow_branch(&mut self, seed: Seed, needed: usize, rng: &mut ChaCha8Rng) -> bool {
        let Seed { i, j, bits, k } = seed;
        // Earlier branches in this pass may have used the seed.
        if !self.tiles[(j, i)].contain_net(bits) || !self.can_grow_out_of(i, j, bits, k, false) {
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
            // Always drawn, so the choices that follow are the same either way.
            let may_cross = rng.gen_bool(CROSSING_CHANCE) && self.crossings;
            let exits: Vec<usize> = (0..6)
                .filter(|&k| self.can_grow_out_of(here.0, here.1, net, k, may_cross))
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

        // The vertex S (frag_vertex_s) alone on each centre tile, one per orientation, in
        // the same frame as the single-net drawings.
        {
            use crate::renders::generated::fragment::MAP_PATHS;
            use crate::renders::svg::Svg;
            use crate::renders::vertex_s::VERTEX_S_KEYS;
            use common::types::bits::transform_tile;
            use common::types::coordinate::Coordinate;
            use common::types::point::{Point, TILE_HEIGHT, TILE_WIDTH};
            let margin = 5.0;
            let width = 10.0 * TILE_WIDTH / 2.0 + 5.0 * margin;
            let height = 8.0 * TILE_HEIGHT + 5.0 * margin;
            for (i, j) in [(3usize, 3usize), (4, 3), (5, 3), (3, 4), (4, 4), (5, 4)] {
                for (index, &key) in VERTEX_S_KEYS.iter().enumerate() {
                    let drawn = transform_tile(key, Coordinate { i, j });
                    let fragment = MAP_PATHS.get(&(drawn as u64)).expect("vertex S fragment");
                    let mut svg = Svg::new(width, height);
                    let at = Point {
                        x: i as f64,
                        y: j as f64,
                    };
                    svg.add_fragment(fragment, at, 1);
                    std::fs::write(format!("{dir}/s{index}_{i}_{j}.svg"), svg.to_string(false))
                        .unwrap();
                }
            }
        }

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

    #[test]
    fn test_reach_gaps_fills_empty_tiles_without_touching() {
        let empty_tiles = |maze: &Maze| {
            maze.tiles
                .elements_row_major_iter()
                .filter(|tile| tile.nodes.is_empty())
                .count()
        };
        let (mut before, mut after) = (0, 0);
        for seed in 1..=8 {
            let mut maze = prepared(seed);
            maze.separate_touching_tracks();
            let touching = maze.touching_count();
            let mut rng = maze.seeded_rng();
            maze.grow_branches(3, &mut rng);
            before += empty_tiles(&maze);

            while maze.reach_gaps(&mut rng) {
                maze.grow_branches(3, &mut rng);
            }
            maze.rebuild_nets();
            after += empty_tiles(&maze);

            assert_eq!(maze.nets.len(), 1, "seed {seed}");
            assert_eq!(maze.loop_count(), 0, "seed {seed}");
            // Growing may take a dead end out of a touch, but never adds one.
            assert!(maze.touching_count() <= touching, "seed {seed}");
        }
        // Wider tracks leave no clear route past others from about 0.13 of a tile, so
        // there reaching gaps may fill nothing; it must still never add empty tiles.
        assert!(
            after <= before,
            "{before} empty tiles before, {after} after"
        );
        if common::types::point::PATH_WIDTH <= 0.11 {
            assert!(after < before, "{before} empty tiles before, {after} after");
        }
    }
}

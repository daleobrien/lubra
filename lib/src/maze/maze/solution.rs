//! The solution: the route through the maze from the entrance to the exit.

use std::collections::{HashMap, VecDeque};

use crate::maze::maze::Maze;

/// One tile on the route: the tile net it follows, the node it comes in by and the
/// node it leaves by (logical node indices 0-5). The entrance has no way in and the
/// exit no way out: the route starts and ends in the middle of those tiles.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RouteStep {
    pub i: usize,
    pub j: usize,
    pub net: usize,
    pub entry: Option<usize>,
    pub exit: Option<usize>,
}

impl Maze {
    /// The route from the entrance `(0, 0)` to the exit (the last tile), one step per
    /// tile, or nothing when they are not joined. The maze is a tree, so the route is
    /// the only one.
    ///
    /// Node `k` of a tile is the same place as node `k` of the tile across it, and the
    /// nodes of one tile net are joined inside the tile.
    pub fn solution_route(&self) -> Vec<RouteStep> {
        let (columns, rows) = (self.number_of_columns, self.number_of_rows);
        if columns == 0 || rows == 0 {
            return Vec::new();
        }
        let (start, end) = ((0, 0), (columns - 1, rows - 1));
        // The tile net holding node `k` of tile `(i, j)`.
        let net_at = |(i, j): (usize, usize), k: usize| {
            self.tiles[(j, i)]
                .nodes
                .iter()
                .copied()
                .find(|net| net & (1 << k) != 0)
        };

        // Search from the entrance's nodes; a state is a node of a tile, reached by
        // coming in through it.
        type State = ((usize, usize), usize);
        let mut came_from: HashMap<State, Option<State>> = HashMap::new();
        let mut queue: VecDeque<State> = VecDeque::new();
        for &net in &self.tiles[(start.1, start.0)].nodes {
            for k in (0..6).filter(|k| net & (1 << k) != 0) {
                // Leaving the entrance through node k.
                came_from.insert((start, k + 6), None);
                queue.push_back((start, k + 6));
            }
        }
        // States with `k >= 6` mean "leaving through node k - 6"; others "arrived by k".
        let mut finish = None;
        while let Some(state) = queue.pop_front() {
            let (tile, k) = state;
            let mut next: Vec<State> = Vec::new();
            if k >= 6 {
                // Cross into the tile across the node.
                if let Some(across) = self.tile_across(tile.0, tile.1, k - 6) {
                    if net_at(across, k - 6).is_some() {
                        next.push((across, k - 6));
                    }
                }
            } else {
                if tile == end {
                    finish = Some(state);
                    break;
                }
                if let Some(net) = net_at(tile, k) {
                    for out in (0..6).filter(|&o| o != k && net & (1 << o) != 0) {
                        next.push((tile, out + 6));
                    }
                }
            }
            for n in next {
                if let std::collections::hash_map::Entry::Vacant(e) = came_from.entry(n) {
                    e.insert(Some(state));
                    queue.push_back(n);
                }
            }
        }
        let Some(finish) = finish else {
            return Vec::new();
        };

        // Walk back, then turn the states into steps.
        let mut states = vec![finish];
        while let Some(Some(previous)) = came_from.get(states.last().expect("a state")) {
            states.push(*previous);
        }
        states.reverse();
        let mut steps: Vec<RouteStep> = Vec::new();
        let mut entry: Option<usize> = None;
        for (tile, k) in states {
            if k >= 6 {
                let exit = k - 6;
                let net = net_at(tile, exit).unwrap_or(1 << exit);
                steps.push(RouteStep {
                    i: tile.0,
                    j: tile.1,
                    net,
                    entry,
                    exit: Some(exit),
                });
            } else {
                entry = Some(k);
            }
        }
        if let Some(arrived) = entry {
            let net = net_at(end, arrived).unwrap_or(1 << arrived);
            steps.push(RouteStep {
                i: end.0,
                j: end.1,
                net,
                entry: Some(arrived),
                exit: None,
            });
        }
        steps
    }
}

#[cfg(test)]
mod tests {
    use crate::maze::maze::Maze;

    #[test]
    fn test_route_runs_from_the_entrance_to_the_exit_through_joined_nodes() {
        for seed in [1, 2, 3] {
            let mut maze = Maze::new(20, 12, true, false, false, Some(seed));
            maze.reduce_nets(1);
            maze.remove_cycles(false);
            maze.trim_dead_ends(2);
            let route = maze.solution_route();
            assert!(route.len() > 2, "seed {seed}");
            let (first, last) = (route[0], route[route.len() - 1]);
            assert_eq!((first.i, first.j, first.entry), (0, 0, None));
            assert_eq!((last.i, last.j, last.exit), (19, 11, None));
            for pair in route.windows(2) {
                let (a, b) = (pair[0], pair[1]);
                let k = a.exit.expect("an exit");
                assert_eq!(b.entry, Some(k), "seed {seed}");
                assert_eq!(maze.tile_across(a.i, a.j, k), Some((b.i, b.j)));
                assert!(a.net & (1 << k) != 0 && b.net & (1 << k) != 0);
            }
            // A tree has one route, so no tile net is used twice.
            let mut seen = std::collections::HashSet::new();
            assert!(route.iter().all(|s| seen.insert((s.i, s.j, s.net))));
        }
    }
}

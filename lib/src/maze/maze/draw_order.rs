//! The order fragments are drawn in, so that tracks pass over and under each other
//! cleanly.
//!
//! Each fragment's white fill hides (most of) what was drawn before it, so where two
//! tracks cross, the one drawn later passes over. A crossing is not always inside
//! one tile: many fragments reach into the tiles around, so the pieces of two tracks
//! either side of a tile edge cross too. Drawn simply tile by tile, a track passing
//! under could then have one wall hidden and the other drawn on top of the track
//! passing over.
//!
//! So two fragments meet where a wall of one runs inside the fill of the other, as
//! drawn; each such pair is a crossing with one side over the other, and every piece
//! of the track passing over that meets a piece of the track passing under is drawn
//! after it.

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};

use crate::maze::maze::Maze;
use crate::renders::ink::{fragment_fills, fragment_walls};
use common::types::point::{TILE_HEIGHT, TILE_WIDTH};

/// How finely walls and fills are followed when finding where fragments meet, in
/// drawing units (a tile is 120 wide).
const STEP: f64 = 3.0;

/// How far inside a fill a wall must run to count as covered, in drawing units: less
/// is where two pieces of one track, or tracks drawn side by side, share an edge.
const DEPTH: f64 = 1.5;

/// How many tiles apart (in columns and rows) two fragments can be and still meet.
const REACH: usize = 3;

/// One drawn fragment, as `drawing_order` needs it.
#[derive(Clone, Copy, Debug)]
pub(super) struct Piece<'a> {
    /// Its tile.
    pub(super) i: usize,
    pub(super) j: usize,
    /// The tile net it draws.
    pub(super) nodes: usize,
    /// Whether it is track (not a marker or a tile outline), so takes part.
    pub(super) track: bool,
    /// Its SVG, as placed at its tile.
    pub(super) svg: &'a str,
}

/// How a crossing was found, strongest first: which crossings decide the order when
/// two would decide it differently.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Strength {
    /// Two tracks of one tile: the tile's order of nets says which passes over.
    SameTile,
    /// Tracks of different tiles.
    Across,
}

type Point = (f64, f64);

/// A fragment's walls and fills in drawing units, with the box round them.
struct Shape {
    walls: Vec<Vec<Point>>,
    fills: Vec<Vec<Point>>,
    low: Point,
    high: Point,
}

impl Shape {
    fn of(piece: &Piece) -> Shape {
        let offset = (
            piece.i as f64 * TILE_WIDTH / 2.0,
            piece.j as f64 * TILE_HEIGHT,
        );
        let walls = fragment_walls(piece.svg, offset, STEP);
        let fills = fragment_fills(piece.svg, offset, STEP);
        let mut low = (f64::INFINITY, f64::INFINITY);
        let mut high = (f64::NEG_INFINITY, f64::NEG_INFINITY);
        for &(x, y) in walls.iter().chain(&fills).flatten() {
            low = (low.0.min(x), low.1.min(y));
            high = (high.0.max(x), high.1.max(y));
        }
        Shape {
            walls,
            fills,
            low,
            high,
        }
    }

    fn overlaps(&self, other: &Shape) -> bool {
        self.low.0 <= other.high.0
            && other.low.0 <= self.high.0
            && self.low.1 <= other.high.1
            && other.low.1 <= self.high.1
    }

    /// Whether one of `other`'s walls runs inside this shape's fill.
    fn covers(&self, other: &Shape) -> bool {
        other.walls.iter().flatten().any(|&point| {
            point.0 >= self.low.0
                && point.0 <= self.high.0
                && point.1 >= self.low.1
                && point.1 <= self.high.1
                && self
                    .fills
                    .iter()
                    .any(|fill| inside(fill, point) && distance_to_edge(fill, point) > DEPTH)
        })
    }
}

/// Whether `point` is inside the closed outline `outline` (even-odd).
fn inside(outline: &[Point], point: Point) -> bool {
    let mut inside = false;
    let mut previous = outline[outline.len() - 1];
    for &here in outline {
        if (here.1 > point.1) != (previous.1 > point.1) {
            let x = here.0 + (point.1 - here.1) * (previous.0 - here.0) / (previous.1 - here.1);
            if point.0 < x {
                inside = !inside;
            }
        }
        previous = here;
    }
    inside
}

/// How far `point` is from the nearest edge of the closed outline `outline`.
fn distance_to_edge(outline: &[Point], point: Point) -> f64 {
    let mut previous = outline[outline.len() - 1];
    let mut nearest = f64::INFINITY;
    for &here in outline {
        let (dx, dy) = (here.0 - previous.0, here.1 - previous.1);
        let length = dx * dx + dy * dy;
        let t = if length > 0.0 {
            (((point.0 - previous.0) * dx + (point.1 - previous.1) * dy) / length).clamp(0.0, 1.0)
        } else {
            0.0
        };
        let closest = (previous.0 + t * dx, previous.1 + t * dy);
        nearest = nearest.min((point.0 - closest.0).hypot(point.1 - closest.1));
        previous = here;
    }
    nearest
}

impl Maze {
    /// The order to draw `pieces` in: indices into `pieces`.
    ///
    /// For each pair of pieces that meet, the one given later passes over: in a
    /// tile that is the tile's own order, which `separate_touching_tracks` and
    /// `fill_blank_areas` set. Then each piece of the track passing over (the piece
    /// and those joined to it across its nodes) that meets a piece of the track
    /// passing under is drawn after it. The tiles' own crossings are settled first;
    /// a pair already settled the other way is left so. Otherwise pieces keep the
    /// order given.
    pub(super) fn drawing_order(&self, pieces: &[Piece]) -> Vec<usize> {
        let mut by_net: HashMap<(usize, usize, usize), Vec<usize>> = HashMap::new();
        let mut by_tile: HashMap<(usize, usize), Vec<usize>> = HashMap::new();
        for (index, piece) in pieces.iter().enumerate() {
            if piece.track {
                by_net
                    .entry((piece.i, piece.j, piece.nodes))
                    .or_default()
                    .push(index);
                by_tile.entry((piece.i, piece.j)).or_default().push(index);
            }
        }

        // Each piece with the pieces joined to it across its nodes.
        let near: Vec<Vec<usize>> = pieces
            .iter()
            .enumerate()
            .map(|(index, piece)| {
                let mut near = vec![index];
                if !piece.track {
                    return near;
                }
                for k in (0..6).filter(|&k| piece.nodes & (1 << k) != 0) {
                    let Some((ni, nj)) = self.tile_across(piece.i, piece.j, k) else {
                        continue;
                    };
                    for &nodes in &self.tiles[(nj, ni)].nodes {
                        if nodes & (1 << k) != 0 {
                            near.extend(by_net.get(&(ni, nj, nodes)).into_iter().flatten());
                        }
                    }
                }
                near
            })
            .collect();

        // Every pair of pieces that meet (and are not one track), the earlier first.
        let shapes: Vec<Option<Shape>> = pieces
            .iter()
            .map(|piece| piece.track.then(|| Shape::of(piece)))
            .collect();
        let mut meets: HashSet<(usize, usize)> = HashSet::new();
        let mut crossings: Vec<(Strength, usize, usize)> = Vec::new();
        for (index, piece) in pieces.iter().enumerate() {
            let Some(shape) = &shapes[index] else {
                continue;
            };
            let columns = piece.i.saturating_sub(2 * REACH)..=piece.i + 2 * REACH;
            for ni in columns {
                for nj in piece.j.saturating_sub(REACH)..=piece.j + REACH {
                    for &other in by_tile.get(&(ni, nj)).into_iter().flatten() {
                        if other <= index || near[index].contains(&other) {
                            continue;
                        }
                        let Some(other_shape) = &shapes[other] else {
                            continue;
                        };
                        if !shape.overlaps(other_shape)
                            || !(shape.covers(other_shape) || other_shape.covers(shape))
                        {
                            continue;
                        }
                        meets.insert((index, other));
                        let strength = if (ni, nj) == (piece.i, piece.j) {
                            Strength::SameTile
                        } else {
                            Strength::Across
                        };
                        crossings.push((strength, index, other));
                    }
                }
            }
        }
        crossings.sort_unstable();

        // `after[p]`: the pieces to be drawn after `p`. The tiles' own crossings
        // come first, as they are; then each crossing settles the pairs round it.
        let mut after: Vec<Vec<usize>> = vec![Vec::new(); pieces.len()];
        let mut settled: HashSet<(usize, usize)> = HashSet::new();
        for &(strength, under, over) in &crossings {
            if strength == Strength::SameTile && settled.insert((under, over)) {
                after[under].push(over);
            }
        }
        for &(_, under, over) in &crossings {
            for &low in &near[under] {
                for &high in &near[over] {
                    let pair = (low.min(high), low.max(high));
                    if low == high || !meets.contains(&pair) || settled.contains(&(high, low)) {
                        continue;
                    }
                    if settled.insert((low, high)) {
                        after[low].push(high);
                    }
                }
            }
        }

        topological_order(&after)
    }
}

/// An order of `0..after.len()` with each node before the nodes in its `after`
/// list, taking the lowest index free each time, so that with no constraints the
/// order is `0, 1, 2, ...`. Where the constraints go round in a circle, the lowest
/// index still waiting is taken to break it.
fn topological_order(after: &[Vec<usize>]) -> Vec<usize> {
    let mut waiting_on = vec![0usize; after.len()];
    for list in after {
        for &next in list {
            waiting_on[next] += 1;
        }
    }
    let mut free: BinaryHeap<Reverse<usize>> = (0..after.len())
        .filter(|&node| waiting_on[node] == 0)
        .map(Reverse)
        .collect();
    let mut placed = vec![false; after.len()];
    let mut order = Vec::with_capacity(after.len());
    while order.len() < after.len() {
        let node = match free.pop() {
            Some(Reverse(node)) if !placed[node] => node,
            Some(_) => continue,
            // A circle: take the lowest index not yet placed.
            None => (0..after.len())
                .find(|&node| !placed[node])
                .unwrap_or_default(),
        };
        placed[node] = true;
        order.push(node);
        for &next in &after[node] {
            waiting_on[next] = waiting_on[next].saturating_sub(1);
            if waiting_on[next] == 0 && !placed[next] {
                free.push(Reverse(next));
            }
        }
    }
    order
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_topological_order_keeps_the_order_without_constraints() {
        assert_eq!(topological_order(&[vec![], vec![], vec![]]), vec![0, 1, 2]);
    }

    #[test]
    fn test_topological_order_follows_constraints() {
        // 2 before 0, 0 before 1.
        let after = vec![vec![1], vec![], vec![0]];
        assert_eq!(topological_order(&after), vec![2, 0, 1]);
    }

    #[test]
    fn test_topological_order_breaks_circles() {
        // 0 before 1 before 2 before 0: every node is still placed once.
        let after = vec![vec![1], vec![2], vec![0], vec![]];
        let mut order = topological_order(&after);
        assert_eq!(order.len(), 4);
        order.sort_unstable();
        assert_eq!(order, vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_inside_and_distance_to_edge() {
        let square = [(0.0, 0.0), (10.0, 0.0), (10.0, 10.0), (0.0, 10.0)];
        assert!(inside(&square, (5.0, 5.0)));
        assert!(!inside(&square, (15.0, 5.0)));
        assert!((distance_to_edge(&square, (5.0, 2.0)) - 2.0).abs() < 1e-9);
    }

    #[test]
    fn test_a_wall_inside_a_fill_is_covered() {
        let fill = "<path fill='#FFF' d='M0 0L20 0L20 20L0 20z'/>";
        let wall = "<path stroke='black' d='M10 -10L10 30'/>";
        let edge = "<path stroke='black' d='M0 -10L0 30'/>";
        let piece = |svg| Piece {
            i: 0,
            j: 0,
            nodes: 1,
            track: true,
            svg,
        };
        let (fill, wall, edge) = (
            Shape::of(&piece(fill)),
            Shape::of(&piece(wall)),
            Shape::of(&piece(edge)),
        );
        assert!(fill.covers(&wall));
        assert!(!fill.covers(&edge), "a wall along the edge is not covered");
        assert!(!wall.covers(&fill), "a wall has no fill");
    }
}

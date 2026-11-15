//! Kink detection: finds the places where the drawn walls bend badly.
//!
//! Every wall is made of straight lines and circular arcs, so along a wall the
//! heading is known everywhere and the curvature is 0 on a line and ±1/radius on
//! an arc. Walls are joined end to end across fragments (their ends meet exactly at
//! the openings), and each joined wall is checked for two defects:
//!
//! - a *break*: the heading jumps where two pieces meet;
//! - a *wiggle*: the wall turns one way and then straight back the other, and both
//!   curves are tight. The walls of two back-to-back bends of a 20-unit centreline
//!   have radii 9.2 and 30.8, which is the shape of every hump and double hook that
//!   has been redrawn by hand so far.
//!
//! A wall that only ever bends one way, however tightly, has no kink. Neither has a
//! junction's rounded corner followed by a gentle curve the other way, nor the round
//! cap of a dead end (walls are cut there, as the two sides of a track are different
//! walls).

use std::collections::HashMap;
use std::f64::consts::PI;

use common::consts::NX;
use common::types::point::{PATH_WIDTH, TILE_WIDTH};

/// A heading jump of more than this (radians) where two pieces meet is a break.
/// Fragment coordinates are written to one decimal place, which alone can shift the
/// heading at the end of a short arc by a few degrees.
pub const BREAK_ANGLE: f64 = 8.0 * PI / 180.0;

/// A reversal is a wiggle when the tighter of its two curves has a wall radius
/// below this (the S-curve fragments use 15.2 or more, so they pass) and the other
/// is below `KINK_WIDE_RADIUS`.
pub const KINK_RADIUS: f64 = 15.0;

/// The wider curve of a wiggle has a wall radius below this. A junction's rounded
/// corner running into an ordinary arc (49.2 or more) is above it.
pub const KINK_WIDE_RADIUS: f64 = 35.0;

/// Curves that turn less than this (radians) do not count towards a wiggle.
pub const KINK_TURN: f64 = 10.0 * PI / 180.0;

/// Two opposite curves with more straight than this between them are separate bends.
pub const KINK_STRAIGHT: f64 = 25.0;

/// Kinks closer than this are one kink: the two walls of a track kink together.
const SAME_KINK: f64 = 30.0;

/// An arc of half the track's width that turns at least this far (radians) is the
/// round cap of a dead end.
const CAP_TURN: f64 = 150.0 * PI / 180.0;

/// How close two wall ends must be to be the same point.
const JOIN_DISTANCE: f64 = 0.3;

/// Pieces shorter than this are dropped: rounding leaves them with no usable heading.
const SHORTEST_PIECE: f64 = 1.0;

/// The kind of defect found.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum KinkKind {
    /// The heading jumps.
    Break,
    /// The wall turns one way then tightly back the other.
    Wiggle,
}

/// One kink in the drawn maze, in drawing units (before the y flip).
#[derive(Clone, Copy, Debug)]
pub struct Kink {
    pub x: f64,
    pub y: f64,
    pub kind: KinkKind,
    /// The fragment keys either side of the kink (equal when it is inside one fragment).
    pub keys: (usize, usize),
    /// The tighter wall radius at a wiggle; 0 for a break.
    pub radius: f64,
    /// The wider wall radius at a wiggle; 0 for a break.
    pub wide_radius: f64,
}

/// One fragment as drawn: its `MAP_PATHS` key, its SVG and where it is placed.
pub struct DrawnFragment<'a> {
    pub key: usize,
    pub fragment: &'a str,
    pub x: f64,
    pub y: f64,
}

/// A line or arc of a wall, in the direction of travel.
#[derive(Clone, Copy, Debug)]
struct Piece {
    start: (f64, f64),
    end: (f64, f64),
    heading_start: f64,
    heading_end: f64,
    /// Signed: positive turns anticlockwise, 0 for a line.
    curvature: f64,
    length: f64,
    key: usize,
}

impl Piece {
    fn line(start: (f64, f64), end: (f64, f64), key: usize) -> Piece {
        let heading = (end.1 - start.1).atan2(end.0 - start.0);
        Piece {
            start,
            end,
            heading_start: heading,
            heading_end: heading,
            curvature: 0.0,
            length: distance(start, end),
            key,
        }
    }

    /// An SVG arc from `start` to `end` (equal radii, no rotation).
    fn arc(
        start: (f64, f64),
        end: (f64, f64),
        radius: f64,
        large: bool,
        sweep: bool,
        key: usize,
    ) -> Piece {
        let chord = distance(start, end);
        // SVG scales a radius that is too small to reach up to half the chord.
        let radius = radius.max(chord / 2.0);
        let to_centre = (radius * radius - chord * chord / 4.0).max(0.0).sqrt();
        let side = if large != sweep { 1.0 } else { -1.0 };
        let (dx, dy) = (end.0 - start.0, end.1 - start.1);
        let centre = (
            (start.0 + end.0) / 2.0 - side * to_centre * dy / chord,
            (start.1 + end.1) / 2.0 + side * to_centre * dx / chord,
        );
        let direction = if sweep { 1.0 } else { -1.0 };
        let angle_start = (start.1 - centre.1).atan2(start.0 - centre.0);
        let angle_end = (end.1 - centre.1).atan2(end.0 - centre.0);
        let swept = ((angle_end - angle_start) * direction).rem_euclid(2.0 * PI);
        Piece {
            start,
            end,
            heading_start: angle_start + direction * PI / 2.0,
            heading_end: angle_end + direction * PI / 2.0,
            curvature: direction / radius,
            length: swept * radius,
            key,
        }
    }

    fn reversed(&self) -> Piece {
        Piece {
            start: self.end,
            end: self.start,
            heading_start: self.heading_end + PI,
            heading_end: self.heading_start + PI,
            curvature: -self.curvature,
            length: self.length,
            key: self.key,
        }
    }

    /// How far the piece turns, in radians (signed).
    fn turn(&self) -> f64 {
        self.curvature * self.length
    }

    /// Whether the piece is the round cap of a dead end, where a wall runs round the
    /// end of the track and back along its other side.
    fn is_cap(&self) -> bool {
        let half_width = PATH_WIDTH * TILE_WIDTH;
        self.curvature != 0.0
            && (1.0 / self.curvature.abs() - half_width).abs() < 0.2
            && self.turn().abs() >= CAP_TURN
    }
}

fn distance(a: (f64, f64), b: (f64, f64)) -> f64 {
    (a.0 - b.0).hypot(a.1 - b.1)
}

/// The difference between two headings, in `(-π, π]`.
fn heading_change(from: f64, to: f64) -> f64 {
    let change = (to - from).rem_euclid(2.0 * PI);
    if change > PI {
        change - 2.0 * PI
    } else {
        change
    }
}

/// The `d` attribute of every stroked path in a fragment.
fn wall_paths(fragment: &str) -> impl Iterator<Item = &str> {
    fragment
        .split("<path")
        .filter(|path| path.contains("stroke='"))
        .filter_map(|path| {
            let rest = &path[path.find(" d='")? + 4..];
            Some(&rest[..rest.find('\'')?])
        })
}

/// Splits path data into commands and numbers.
fn tokens(d: &str) -> Vec<&str> {
    let mut out = Vec::new();
    let mut number_start = None;
    for (index, c) in d.char_indices() {
        let in_number = c.is_ascii_digit() || c == '.' || (c == '-' && number_start.is_none());
        if in_number {
            number_start.get_or_insert(index);
            continue;
        }
        if let Some(start) = number_start.take() {
            out.push(&d[start..index]);
        }
        if c == '-' {
            number_start = Some(index);
        } else if c.is_ascii_alphabetic() {
            out.push(&d[index..index + 1]);
        }
    }
    if let Some(start) = number_start {
        out.push(&d[start..]);
    }
    out
}

/// The walls of one fragment placed at `(x, y)`: each is the pieces of one subpath.
/// Only the commands the fragments use are understood (`M m L l A Z z`); a path
/// with anything else is cut short there.
fn walls(drawn: &DrawnFragment) -> Vec<Vec<Piece>> {
    let mut out = Vec::new();
    for d in wall_paths(drawn.fragment) {
        let tokens = tokens(d);
        let number = |index: usize| tokens.get(index).and_then(|t| t.parse::<f64>().ok());
        let mut wall: Vec<Piece> = Vec::new();
        let mut current = (0.0, 0.0);
        let mut command = "";
        let mut index = 0;
        while index < tokens.len() {
            if tokens[index].starts_with(|c: char| c.is_ascii_alphabetic()) {
                command = tokens[index];
                index += 1;
                if command == "M" || command == "m" {
                    out.push(std::mem::take(&mut wall));
                }
                continue;
            }
            let relative = command == "m" || command == "l";
            match command {
                "M" | "m" | "L" | "l" => {
                    let (Some(px), Some(py)) = (number(index), number(index + 1)) else {
                        break;
                    };
                    let point = if relative {
                        (current.0 + px, current.1 + py)
                    } else {
                        (px + drawn.x, py + drawn.y)
                    };
                    if command == "L" || command == "l" {
                        wall.push(Piece::line(current, point, drawn.key));
                    } else {
                        // Further pairs after a move are lines.
                        command = if relative { "l" } else { "L" };
                    }
                    current = point;
                    index += 2;
                }
                "A" => {
                    let (Some(radius), Some(large), Some(sweep), Some(px), Some(py)) = (
                        number(index),
                        number(index + 3),
                        number(index + 4),
                        number(index + 5),
                        number(index + 6),
                    ) else {
                        break;
                    };
                    let point = (px + drawn.x, py + drawn.y);
                    wall.push(Piece::arc(
                        current,
                        point,
                        radius,
                        large != 0.0,
                        sweep != 0.0,
                        drawn.key,
                    ));
                    current = point;
                    index += 7;
                }
                _ => break,
            }
        }
        out.push(wall);
    }
    // Cut each wall at dead-end caps and drop pieces too short to have a heading.
    out.into_iter()
        .flat_map(|wall| {
            wall.split(Piece::is_cap)
                .map(|side| {
                    side.iter()
                        .copied()
                        .filter(|piece| piece.length >= SHORTEST_PIECE)
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .filter(|wall| !wall.is_empty())
        .collect()
}

/// Joins walls whose ends meet into chains, each a run of pieces in travel order.
/// An end shared by more than two walls is left as an end.
fn chains(walls: Vec<Vec<Piece>>) -> Vec<Vec<Piece>> {
    // Wall ends by grid cell, to find the ends that meet. `end` is 0 for a wall's
    // start and 1 for its end.
    let cell = |p: (f64, f64)| ((p.0 * 2.0).round() as i64, (p.1 * 2.0).round() as i64);
    let end_point = |wall: usize, end: usize| {
        if end == 0 {
            walls[wall][0].start
        } else {
            walls[wall][walls[wall].len() - 1].end
        }
    };
    let mut by_cell: HashMap<(i64, i64), Vec<(usize, usize)>> = HashMap::new();
    for wall in 0..walls.len() {
        for end in 0..2 {
            by_cell
                .entry(cell(end_point(wall, end)))
                .or_default()
                .push((wall, end));
        }
    }
    let partner = |wall: usize, end: usize| -> Option<(usize, usize)> {
        let point = end_point(wall, end);
        let (cx, cy) = cell(point);
        let mut found = None;
        for dx in -1..=1 {
            for dy in -1..=1 {
                for &(other, other_end) in by_cell.get(&(cx + dx, cy + dy)).into_iter().flatten() {
                    let same = (other, other_end) == (wall, end);
                    if same || distance(point, end_point(other, other_end)) > JOIN_DISTANCE {
                        continue;
                    }
                    if found.is_some() {
                        return None;
                    }
                    found = Some((other, other_end));
                }
            }
        }
        found
    };

    let mut used = vec![false; walls.len()];
    let mut out = Vec::new();
    // Chains that have an open end first, then whatever is left (closed loops).
    let open_first = (0..walls.len())
        .filter(|&wall| partner(wall, 0).is_none() || partner(wall, 1).is_none())
        .chain(0..walls.len());
    for first in open_first.collect::<Vec<_>>() {
        if used[first] {
            continue;
        }
        // Enter the first wall at an open end if it has one.
        let entry = if partner(first, 0).is_some() && partner(first, 1).is_none() {
            1
        } else {
            0
        };
        let mut chain = Vec::new();
        let mut at = Some((first, entry));
        while let Some((wall, entered)) = at {
            if used[wall] {
                break;
            }
            used[wall] = true;
            if entered == 0 {
                chain.extend(walls[wall].iter().copied());
            } else {
                chain.extend(walls[wall].iter().rev().map(Piece::reversed));
            }
            at = partner(wall, 1 - entered);
        }
        out.push(chain);
    }
    out
}

/// The kinks along one chain of pieces.
fn chain_kinks(chain: &[Piece]) -> Vec<Kink> {
    let mut out = Vec::new();
    for (index, piece) in chain.iter().enumerate().skip(1) {
        let before = &chain[index - 1];
        let jump = heading_change(before.heading_end, piece.heading_start).abs();
        if distance(before.end, piece.start) <= SHORTEST_PIECE && jump > BREAK_ANGLE {
            out.push(Kink {
                x: piece.start.0,
                y: piece.start.1,
                kind: KinkKind::Break,
                keys: (before.key & NX, piece.key & NX),
                radius: 0.0,
                wide_radius: 0.0,
            });
        }
        if piece.curvature == 0.0 || piece.turn().abs() < KINK_TURN {
            continue;
        }
        // The last real curve before this one, if only a little straight lies between.
        let mut straight = 0.0;
        let previous = chain[..index].iter().rev().find(|earlier| {
            let curve = earlier.curvature != 0.0 && earlier.turn().abs() >= KINK_TURN;
            if !curve {
                straight += earlier.length;
            }
            curve || straight > KINK_STRAIGHT
        });
        let Some(previous) = previous.filter(|_| straight <= KINK_STRAIGHT) else {
            continue;
        };
        let (first, second) = (1.0 / previous.curvature.abs(), 1.0 / piece.curvature.abs());
        let (radius, wide) = (first.min(second), first.max(second));
        let reverses = previous.curvature * piece.curvature < 0.0;
        if reverses && radius < KINK_RADIUS && wide < KINK_WIDE_RADIUS {
            out.push(Kink {
                x: (previous.end.0 + piece.start.0) / 2.0,
                y: (previous.end.1 + piece.start.1) / 2.0,
                kind: KinkKind::Wiggle,
                keys: (previous.key & NX, piece.key & NX),
                radius,
                wide_radius: wide,
            });
        }
    }
    out
}

/// Every kink in the drawn fragments. The two walls of a track kink together, so
/// kinks of one kind within a track's width of each other are reported once (the
/// tighter one for wiggles).
pub fn find_kinks(drawn: &[DrawnFragment]) -> Vec<Kink> {
    let all_walls: Vec<Vec<Piece>> = drawn.iter().flat_map(walls).collect();
    let mut found: Vec<Kink> = chains(all_walls)
        .iter()
        .flat_map(|chain| chain_kinks(chain))
        .collect();
    found.sort_by(|a, b| a.radius.total_cmp(&b.radius));
    let mut out: Vec<Kink> = Vec::new();
    for kink in found {
        let near = |other: &Kink| {
            other.kind == kink.kind && distance((other.x, other.y), (kink.x, kink.y)) < SAME_KINK
        };
        if !out.iter().any(near) {
            out.push(kink);
        }
    }
    out
}

/// A fragment key as its node groups, e.g. `56/15/26`: the tile's own nodes, then
/// each neighbouring net it also draws. The inverted bit is ignored.
pub fn key_label(key: usize) -> String {
    let nodes = |group: usize| -> String {
        (0..6)
            .filter(|k| group & (1 << k) != 0)
            .map(|k| char::from(b'1' + k as u8))
            .collect()
    };
    (0..5)
        .map(|group| (key >> (6 * group)) & 0b11_1111)
        .filter(|&group| group != 0)
        .map(nodes)
        .collect::<Vec<_>>()
        .join("/")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn drawn(fragment: &str) -> DrawnFragment<'_> {
        DrawnFragment {
            key: 3,
            fragment,
            x: 0.0,
            y: 0.0,
        }
    }

    #[test]
    fn test_walls_reads_lines_arcs_and_relative_moves() {
        let found = walls(&drawn(
            "<path fill='#FFF' d='M0 0L9 9z'/><path stroke='black' d='M0 0L10 0A10 10 0 0 1 20 10m5 0l0 10'/>",
        ));
        assert_eq!(found.len(), 2, "the filled path is not a wall");
        let (line, arc) = (found[0][0], found[0][1]);
        assert!((line.length - 10.0).abs() < 1e-9 && line.curvature == 0.0);
        // A quarter circle turning anticlockwise, leaving the line smoothly.
        assert!((arc.turn() - PI / 2.0).abs() < 1e-9, "{arc:?}");
        assert!(heading_change(line.heading_end, arc.heading_start).abs() < 1e-9);
        assert_eq!(found[1][0].start, (25.0, 10.0));
        assert_eq!(found[1][0].end, (25.0, 20.0));
    }

    #[test]
    fn test_smooth_bend_has_no_kink() {
        let kinks = find_kinks(&[drawn(
            "<path stroke='black' d='M0 0L10 0A5 5 0 0 1 15 5L15 20'/>",
        )]);
        assert!(kinks.is_empty(), "{kinks:?}");
    }

    #[test]
    fn test_corner_is_a_break() {
        let kinks = find_kinks(&[drawn("<path stroke='black' d='M0 0L10 0L10 10'/>")]);
        assert_eq!(kinks.len(), 1);
        assert_eq!(kinks[0].kind, KinkKind::Break);
        assert_eq!((kinks[0].x, kinks[0].y), (10.0, 0.0));
    }

    #[test]
    fn test_tight_reversal_is_a_wiggle_but_a_gentle_one_is_not() {
        let s_bend = |radius: f64| {
            format!(
                "<path stroke='black' d='M0 0A{radius} {radius} 0 0 1 {radius} {radius}A{radius} {radius} 0 0 0 {d} {d}'/>",
                d = 2.0 * radius
            )
        };
        let tight = s_bend(9.0);
        let kinks = find_kinks(&[drawn(&tight)]);
        assert_eq!(kinks.len(), 1, "{kinks:?}");
        assert_eq!(kinks[0].kind, KinkKind::Wiggle);
        assert!((kinks[0].radius - 9.0).abs() < 1e-9);

        let gentle = s_bend(30.0);
        assert!(find_kinks(&[drawn(&gentle)]).is_empty());
    }

    #[test]
    fn test_rounded_corner_into_a_wide_curve_is_not_a_wiggle() {
        let fragment = "<path stroke='black' d='M0 0A9 9 0 0 1 9 9A50 50 0 0 0 59 59'/>";
        assert!(find_kinks(&[drawn(fragment)]).is_empty());
    }

    #[test]
    fn test_dead_end_cap_is_not_a_wiggle() {
        // Up one side of a track curving round (0, 0) at radius 20, round its cap (as
        // wide as the track) and back down the other side.
        let half = PATH_WIDTH * TILE_WIDTH;
        let (outer, inner) = (20.0 + half, 20.0 - half);
        let fragment = format!(
            "<path stroke='black' d='M{outer} 0A{outer} {outer} 0 0 1 0 {outer}A{half} {half} 0 0 1 0 {inner}A{inner} {inner} 0 0 0 {inner} 0'/>"
        );
        assert!(find_kinks(&[drawn(&fragment)]).is_empty());
    }

    #[test]
    fn test_wiggle_is_found_across_two_fragments() {
        // The reversal happens where the two fragments' walls meet, with the second
        // wall drawn towards the join.
        let first = DrawnFragment {
            key: 3,
            fragment: "<path stroke='black' d='M0 0A9 9 0 0 1 9 9'/>",
            x: 0.0,
            y: 0.0,
        };
        let second = DrawnFragment {
            key: 5,
            fragment: "<path stroke='black' d='M8 8A9 9 0 0 1 -1 -1'/>",
            x: 10.0,
            y: 10.0,
        };
        let kinks = find_kinks(&[first, second]);
        assert_eq!(kinks.len(), 1, "{kinks:?}");
        let mut keys = [kinks[0].keys.0, kinks[0].keys.1];
        keys.sort_unstable();
        assert_eq!(keys, [3, 5]);
    }

    #[test]
    fn test_both_walls_of_a_track_count_once() {
        let fragment = "<path stroke='black' d='M0 0A9 9 0 0 1 9 9A9 9 0 0 0 18 18M20 0A9 9 0 0 1 29 9A9 9 0 0 0 38 18'/>";
        assert_eq!(find_kinks(&[drawn(fragment)]).len(), 1);
    }

    #[test]
    fn test_key_label() {
        use common::consts::{A1, A5, B2, B6, IN, N5, N6};
        assert_eq!(key_label(N5 | N6), "56");
        assert_eq!(key_label(IN | N5 | N6 | A1 | A5 | B2 | B6), "56/15/26");
    }
}

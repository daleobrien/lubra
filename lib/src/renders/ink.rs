//! A hand-drawn "ink" look, after maze-rs's `-I`.
//!
//! The maze has the same fragments as the plain render, but every wall is
//! flattened to short pieces, nudged by a smooth noise field and drawn with a pen
//! whose width wanders, in brown ink. The noise is seeded from the maze's seed, so a
//! maze always comes out the same, and depends only on where a point is on the page,
//! so walls that meet at a tile edge still meet after the wobble.
//!
//! Tracks pass over one another, so each fragment's inside is painted in the paper
//! colour (wobbled the same way) before its walls; the track drawn later hides the
//! walls of the one beneath, which reads as a break in the ink.
//!
//! Every tile wobbles differently, so fragments can't be drawn once and reused: an
//! inked maze is bigger and slower to write than a plain one.

use std::f64::consts::PI;
use std::fmt::Write as _;

/// The ink, the pencil used for the entrance and exit, and the paper inside a track.
const INK: &str = "#2b2118";
const PENCIL: &str = "#c0392b";
const PAPER: &str = "#ffffff";

/// The middle pen width, in drawing units (the plain walls are 8).
const PEN_WIDTH: f64 = 5.0;

/// The pen widths, as fractions of the middle one.
const WIDTHS: [f64; 3] = [0.8, 1.0, 1.22];

/// How many pieces of a wall are drawn with one width before the pen changes.
const CHUNK: usize = 6;

/// How far a point may be nudged, the size of a wobble, and the longest piece a line
/// or arc is flattened into, in drawing units (a tile is 120 wide).
const AMPLITUDE: f64 = 2.1;
const FEATURE: f64 = 27.0;
const STEP: f64 = 6.0;

/// Different noise fields from the one seed.
const SALT_X: u64 = 0x5851_F42D_4C95_7F2D;
const SALT_Y: u64 = 0x1405_7B7E_F767_814F;
const SALT_WIDTH: u64 = 0x2545_F491_4F6C_DD1D;
const SALT_PENCIL: [u64; 3] = [
    0x9E37_79B9_7F4A_7C15,
    0xBF58_476D_1CE4_E5B9,
    0x94D0_49BB_1331_11EB,
];

type Point = (f64, f64);

// ---- noise ----

/// A value in [-1, 1) for each lattice point.
fn lattice(ix: i64, iy: i64, seed: u64) -> f64 {
    // splitmix64's finaliser over the mixed coordinates
    let mut z = seed
        ^ (ix as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15)
        ^ (iy as u64).wrapping_mul(0xC2B2_AE3D_27D4_EB4F);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^= z >> 31;
    (z >> 11) as f64 / (1u64 << 52) as f64 - 1.0
}

/// Smooth value noise in [-1, 1].
fn noise(x: f64, y: f64, seed: u64) -> f64 {
    let (x0, y0) = (x.floor(), y.floor());
    let (fx, fy) = (x - x0, y - y0);
    let (ix, iy) = (x0 as i64, y0 as i64);
    let (u, v) = (fx * fx * (3.0 - 2.0 * fx), fy * fy * (3.0 - 2.0 * fy));
    let top = lattice(ix, iy, seed) * (1.0 - u) + lattice(ix + 1, iy, seed) * u;
    let bottom = lattice(ix, iy + 1, seed) * (1.0 - u) + lattice(ix + 1, iy + 1, seed) * u;
    top * (1.0 - v) + bottom * v
}

/// The pen: where it puts a point and how wide it is there.
pub struct Pen {
    seed: u64,
}

impl Pen {
    /// A pen whose wobble is fixed by `seed` (the maze's seed).
    pub fn new(seed: u64) -> Pen {
        Pen { seed }
    }

    fn wobble(&self, p: Point) -> Point {
        self.wobble_with(p, 0)
    }

    /// `p` nudged by the noise field `salt` picks (0 for the walls).
    fn wobble_with(&self, (x, y): Point, salt: u64) -> Point {
        let (nx, ny) = (x / FEATURE, y / FEATURE);
        (
            x + AMPLITUDE * noise(nx, ny, self.seed ^ salt ^ SALT_X),
            y + AMPLITUDE * noise(nx, ny, self.seed ^ salt ^ SALT_Y),
        )
    }

    /// Which of the [`WIDTHS`] the pen has at `p`.
    fn width_at(&self, (x, y): Point) -> usize {
        let n = noise(
            x / (FEATURE * 2.0),
            y / (FEATURE * 2.0),
            self.seed ^ SALT_WIDTH,
        );
        if n < -0.2 {
            0
        } else if n > 0.2 {
            2
        } else {
            1
        }
    }
}

// ---- reading fragments ----

/// One subpath of a fragment, flattened to points, and whether it was closed.
struct Line {
    points: Vec<Point>,
    closed: bool,
}

fn distance(p: Point, q: Point) -> f64 {
    (q.0 - p.0).hypot(q.1 - p.1)
}

/// Adds the points of a straight piece from `from` to `to`, `from` excluded.
fn line_points(out: &mut Vec<Point>, from: Point, to: Point, step: f64) {
    let length = distance(from, to);
    if length < 1e-9 {
        return;
    }
    let n = (length / step).ceil().max(1.0) as usize;
    for k in 1..=n {
        let t = k as f64 / n as f64;
        out.push((from.0 + (to.0 - from.0) * t, from.1 + (to.1 - from.1) * t));
    }
}

/// Adds the points of an SVG arc (equal radii, no rotation), `from` excluded.
fn arc_points(
    out: &mut Vec<Point>,
    from: Point,
    to: Point,
    radius: f64,
    (large, sweep): (bool, bool),
    step: f64,
) {
    let (x1, y1) = ((from.0 - to.0) / 2.0, (from.1 - to.1) / 2.0);
    let d2 = x1 * x1 + y1 * y1;
    if d2 < 1e-12 {
        return;
    }
    // SVG scales a radius that is too small up to half the chord.
    let r = radius.abs().max(d2.sqrt());
    let sign = if large == sweep { -1.0 } else { 1.0 };
    let coef = sign * ((r * r - d2) / d2).max(0.0).sqrt();
    let (cx1, cy1) = (coef * y1, -coef * x1);
    let (cx, cy) = (cx1 + (from.0 + to.0) / 2.0, cy1 + (from.1 + to.1) / 2.0);
    let theta = (y1 - cy1).atan2(x1 - cx1);
    let mut delta = (-y1 - cy1).atan2(-x1 - cx1) - theta;
    if sweep && delta < 0.0 {
        delta += 2.0 * PI;
    } else if !sweep && delta > 0.0 {
        delta -= 2.0 * PI;
    }
    let n = (delta.abs() * r / step).ceil().max(2.0) as usize;
    for k in 1..n {
        let angle = theta + delta * k as f64 / n as f64;
        out.push((cx + r * angle.cos(), cy + r * angle.sin()));
    }
    out.push(to);
}

/// Splits path data into commands and numbers.
fn tokens(d: &str) -> Vec<&str> {
    let mut out = Vec::new();
    let mut start = None;
    for (index, c) in d.char_indices() {
        let digit = c.is_ascii_digit() || c == '.';
        if digit || (c == '-' && start.is_none()) {
            start.get_or_insert(index);
            continue;
        }
        if let Some(s) = start.take() {
            out.push(&d[s..index]);
        }
        if c == '-' {
            start = Some(index);
        } else if c.is_ascii_alphabetic() {
            out.push(&d[index..index + 1]);
        }
    }
    if let Some(s) = start {
        out.push(&d[s..]);
    }
    out
}

/// The subpaths of path data, flattened, moved by `offset`.
fn lines(d: &str, offset: Point) -> Vec<Line> {
    flattened(d, offset, STEP)
}

/// The subpaths of path data, flattened to pieces of at most `step`, moved by
/// `offset`. The fragments use only `M m L l A Z z`; anything else ends the path there.
fn flattened(d: &str, offset: Point, step: f64) -> Vec<Line> {
    let tokens = tokens(d);
    let number = |index: usize| tokens.get(index).and_then(|t| t.parse::<f64>().ok());
    let mut out: Vec<Line> = Vec::new();
    let mut current = (0.0, 0.0);
    let mut command = "M";
    let mut index = 0;
    while index < tokens.len() {
        if tokens[index].starts_with(|c: char| c.is_ascii_alphabetic()) {
            command = tokens[index];
            index += 1;
            if command == "Z" || command == "z" {
                if let Some(line) = out.last_mut() {
                    line.closed = true;
                    current = line.points[0];
                }
            }
            continue;
        }
        match command {
            "M" | "m" | "L" | "l" => {
                let (Some(x), Some(y)) = (number(index), number(index + 1)) else {
                    break;
                };
                let relative = command == "m" || command == "l";
                let to = if relative {
                    (current.0 + x, current.1 + y)
                } else {
                    (x + offset.0, y + offset.1)
                };
                if command == "M" || command == "m" {
                    out.push(Line {
                        points: vec![to],
                        closed: false,
                    });
                    // Further pairs after a move are lines.
                    command = if relative { "l" } else { "L" };
                } else if let Some(line) = out.last_mut() {
                    line_points(&mut line.points, current, to, step);
                }
                current = to;
                index += 2;
            }
            "A" => {
                let values: Option<Vec<f64>> = (0..7).map(|k| number(index + k)).collect();
                let Some(v) = values else {
                    break;
                };
                let to = (v[5] + offset.0, v[6] + offset.1);
                if let Some(line) = out.last_mut() {
                    arc_points(
                        &mut line.points,
                        current,
                        to,
                        v[0],
                        (v[3] != 0.0, v[4] != 0.0),
                        step,
                    );
                }
                current = to;
                index += 7;
            }
            _ => break,
        }
    }
    out.retain(|line| line.points.len() > 1);
    out
}

/// The value of attribute `name` in an SVG element's text, if it has one.
fn attribute<'a>(element: &'a str, name: &str) -> Option<&'a str> {
    let at = element.find(&format!(" {name}='"))? + name.len() + 3;
    let rest = &element[at..];
    Some(&rest[..rest.find('\'')?])
}

/// The walls (stroked subpaths) of a fragment placed at `offset`, flattened to
/// pieces of at most `step`, in drawing units. Closing `z`s are not followed.
pub fn fragment_walls(fragment: &str, offset: Point, step: f64) -> Vec<Vec<Point>> {
    fragment
        .split('<')
        .filter(|element| {
            attribute(element, "stroke").is_some_and(|s| s != "none" && s != "transparent")
        })
        .filter_map(|element| attribute(element, "d"))
        .flat_map(|d| flattened(d, offset, STEP.min(step)))
        .map(|line| line.points)
        .collect()
}

// ---- writing ----

/// Appends a polyline as path data.
fn push_polyline(out: &mut String, points: &[Point]) {
    for (k, &(x, y)) in points.iter().enumerate() {
        let _ = write!(out, "{}{x:.1} {y:.1}", if k == 0 { "M" } else { "L" });
    }
}

/// A closed polyline round a circle.
fn circle(cx: f64, cy: f64, r: f64) -> Vec<Point> {
    let n = (2.0 * PI * r / STEP).ceil().max(12.0) as usize;
    (0..=n)
        .map(|k| {
            let angle = 2.0 * PI * k as f64 / n as f64;
            (cx + r * angle.cos(), cy + r * angle.sin())
        })
        .collect()
}

/// One fragment drawn in ink, placed at `offset` in drawing units: its insides in
/// the paper colour, then its walls in ink, then any dot (the entrance and exit) in
/// pencil.
pub fn ink_fragment(fragment: &str, offset: Point, pen: &Pen) -> String {
    let mut out = String::new();
    let mut walls: [String; 3] = Default::default();
    let mut dots = String::new();
    for element in fragment.split('<').filter(|e| !e.trim().is_empty()) {
        if element.starts_with("circle") {
            let get = |name| attribute(element, name).and_then(|v| v.parse::<f64>().ok());
            if let (Some(cx), Some(cy), Some(r)) = (get("cx"), get("cy"), get("r")) {
                let points: Vec<Point> = circle(cx + offset.0, cy + offset.1, r)
                    .into_iter()
                    .map(|p| pen.wobble(p))
                    .collect();
                push_polyline(&mut dots, &points);
                dots.push('Z');
            }
            continue;
        }
        let Some(d) = attribute(element, "d") else {
            continue;
        };
        let filled = attribute(element, "fill").is_some_and(|f| f != "none" && f != "transparent");
        let stroked =
            attribute(element, "stroke").is_some_and(|s| s != "none" && s != "transparent");
        let shapes = lines(d, offset);
        if filled {
            let mut inside = String::new();
            for line in &shapes {
                let points: Vec<Point> = line.points.iter().map(|&p| pen.wobble(p)).collect();
                push_polyline(&mut inside, &points);
                inside.push('Z');
            }
            let _ = write!(out, "<path fill='{PAPER}' fill-opacity='1' d='{inside}'/>");
        }
        if stroked {
            for line in &shapes {
                // A closing `z` is not drawn: in a stroked fragment it runs back along
                // a wall or across an opening.
                let points: Vec<Point> = line.points.iter().map(|&p| pen.wobble(p)).collect();
                // Consecutive chunks share a point, so the pen never lifts.
                let mut start = 0;
                while start + 1 < points.len() {
                    let end = (start + CHUNK).min(points.len() - 1);
                    let width = pen.width_at(points[(start + end) / 2]);
                    push_polyline(&mut walls[width], &points[start..=end]);
                    start = end;
                }
            }
        }
    }
    for (factor, d) in WIDTHS.iter().zip(&walls) {
        if !d.is_empty() {
            let _ = write!(
                out,
                "<path fill='none' stroke='{INK}' stroke-width='{:.2}' stroke-linecap='round' stroke-linejoin='round' d='{d}'/>",
                PEN_WIDTH * factor
            );
        }
    }
    if !dots.is_empty() {
        let _ = write!(
            out,
            "<path fill='{PENCIL}' fill-opacity='0.85' stroke='{INK}' stroke-width='{:.2}' d='{dots}'/>",
            PEN_WIDTH * 0.75
        );
    }
    out
}

/// The solution, `over` where it is on top and `under` where it passes beneath a
/// crossing, as markup. With a `pen` it is pencilled in, gone over twice where on top
/// and once lightly underneath, as in maze-rs; without, it is a plain red trace.
pub fn solution_markup(over: &[Vec<Point>], under: &[Vec<Point>], pen: Option<&Pen>) -> String {
    let mut out = String::new();
    let mut pass = |lines: &[Vec<Point>], salt: u64, width: f64, opacity: &str| {
        let mut d = String::new();
        for line in lines {
            let points: Vec<Point> = match pen {
                Some(pen) => line.iter().map(|&p| pen.wobble_with(p, salt)).collect(),
                None => line.clone(),
            };
            push_polyline(&mut d, &points);
        }
        if !d.is_empty() {
            let _ = write!(
                out,
                "<path fill='none' stroke='{PENCIL}' stroke-opacity='{opacity}' stroke-width='{width:.2}' stroke-linecap='round' stroke-linejoin='round' d='{d}'/>"
            );
        }
    };
    match pen {
        Some(_) => {
            pass(over, SALT_PENCIL[0], PEN_WIDTH * 2.2, "0.45");
            pass(over, SALT_PENCIL[1], PEN_WIDTH * 1.5, "0.45");
            pass(under, SALT_PENCIL[2], PEN_WIDTH * 1.5, "0.18");
        }
        None => {
            pass(over, 0, PEN_WIDTH * 1.5, "0.75");
            pass(under, 0, PEN_WIDTH * 1.5, "0.25");
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_noise_is_smooth_and_bounded() {
        for k in 0..1000 {
            let (x, y) = (k as f64 * 0.137, k as f64 * 0.071);
            let n = noise(x, y, 7);
            assert!((-1.0..=1.0).contains(&n));
            assert!((noise(x + 1e-6, y, 7) - n).abs() < 1e-3);
        }
    }

    #[test]
    fn test_arcs_end_where_they_should() {
        // A quarter circle of radius 10 from (10, 0) to (0, 10) about the origin.
        let found = lines("M10 0A10 10 0 0 1 0 10", (0.0, 0.0));
        let points = &found[0].points;
        assert_eq!(*points.last().unwrap(), (0.0, 10.0));
        for &(x, y) in points {
            assert!(
                (x.hypot(y) - 10.0).abs() < 1e-9,
                "({x}, {y}) is off the circle"
            );
        }
    }

    #[test]
    fn test_relative_moves_and_closing_are_read() {
        let found = lines("M10 0l0 12m5 0l0 -12z", (100.0, 0.0));
        assert_eq!(found.len(), 2);
        assert_eq!(found[0].points.first(), Some(&(110.0, 0.0)));
        assert_eq!(found[0].points.last(), Some(&(110.0, 12.0)));
        assert_eq!(found[1].points.first(), Some(&(115.0, 12.0)));
        assert!(found[1].closed && !found[0].closed);
    }

    #[test]
    fn test_a_wall_shared_by_two_tiles_wobbles_the_same_in_both() {
        // The wobble depends only on where a point is, so the end of one tile's wall
        // and the start of the next tile's meet.
        let pen = Pen::new(3);
        let a = ink_fragment("<path stroke='black' d='M0 0L60 0'/>", (0.0, 0.0), &pen);
        let b = ink_fragment("<path stroke='black' d='M0 0L60 0'/>", (60.0, 0.0), &pen);
        let end_of_a = a
            .rsplit('L')
            .next()
            .unwrap()
            .split('\'')
            .next()
            .unwrap()
            .to_string();
        let start_of_b = b
            .split("d='M")
            .nth(1)
            .unwrap()
            .split('L')
            .next()
            .unwrap()
            .to_string();
        assert_eq!(end_of_a, start_of_b);
    }

    #[test]
    fn test_insides_come_before_walls_and_dots_last() {
        let pen = Pen::new(1);
        let fragment = "<path stroke='black' fill='#FFF' d='M0 0L10 0A5 5 0 0 1 10 10L0 10'/><circle cx='5' cy='5' r='3' fill='#E02020'/>";
        let ink = ink_fragment(fragment, (0.0, 0.0), &pen);
        let inside = ink.find(PAPER).unwrap();
        let wall = ink.find(&format!("stroke='{INK}' stroke-width")).unwrap();
        let dot = ink.find(&format!("fill='{PENCIL}'")).unwrap();
        assert!(inside < wall && wall < dot, "{ink}");
    }

    #[test]
    fn test_same_seed_same_drawing() {
        let fragment = "<path stroke='black' d='M0 0L120 0A60 60 0 0 1 60 103.9'/>";
        let a = ink_fragment(fragment, (0.0, 0.0), &Pen::new(5));
        assert_eq!(a, ink_fragment(fragment, (0.0, 0.0), &Pen::new(5)));
        assert_ne!(a, ink_fragment(fragment, (0.0, 0.0), &Pen::new(6)));
    }
}

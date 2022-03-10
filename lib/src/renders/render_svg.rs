use std::collections::HashMap;
use std::error::Error;
use std::fmt::format;
use std::fs;
use std::io::prelude::*;
use std::ptr::addr_of_mut;

const U: f64 = 100.0;
const M: f64 = 50.0;
const H: f64 = 86.60254037844386; // U * f64::sqrt(3.0) / 2.0;

pub struct Point {
    x: f64,
    y: f64,
}

pub struct Line {
    start: Point,
    delta: Point,
}

pub struct Coordinate {
    i: u32,
    j: u32,
}

pub struct Segment {
    centre: Point,
    radius: f64,
    segment: u64, // Can be much smaller, but now this structure is alligned
}

pub struct Path {
    stroke: String,
    stroke_width: f64,
    path: String,
    opacity: f64,
}

impl Path {
    pub fn new(stroke: String, stroke_width: f64, opacity: f64) -> Path {
        Path {
            stroke: stroke,
            stroke_width: stroke_width,
            path: String::new(),
            opacity: opacity,
        }
    }

    pub fn to_string(&self) -> String {
        let d = &self.path;
        let colour = &self.stroke;
        let w = &self.stroke_width;
        let opacity = &self.opacity;

        if d.is_empty() {
            return "".to_string();
        }
        return format!("<path stroke-opacity=\"{opacity}\" stroke-width=\"{w}\" stroke=\"{colour}\" d=\"{d}\"/>",
                       opacity = opacity, w = w, colour = colour, d = d);
    }

    pub fn append(&mut self, path: &str) {
        self.path.push_str(path);
    }

    fn polar(&self, x: f64, y: f64, radius: f64, angle: f64) -> (f64, f64) {
        let (a, b) = f64::sin_cos(angle);
        return (x + (radius * b), y + (radius * a));
    }

    pub fn line(&mut self, line: &Line) {
        let path = format!(
            "M {} {} l {} {} z ",
            line.start.x, line.start.y, line.delta.x, line.delta.y
        );
        self.path.push_str(&path);
    }
    // https://developer.mozilla.org/en-US/docs/Web/SVG/Tutorial/Paths
    pub fn arc(&mut self, x: f64, y: f64, r: f64, segment: u32, ccw: bool) {
        let ccw = if ccw { "1" } else { "0" };

        let pi6 = 2.0 * std::f64::consts::FRAC_PI_6;
        let angles_from_segment: HashMap<u32, (f64, f64)> = HashMap::from([
            (1, (0.0 * pi6, 1.0 * pi6)),
            (2, (1.0 * pi6, 2.0 * pi6)),
            (3, (2.0 * pi6, 3.0 * pi6)),
            (4, (3.0 * pi6, 4.0 * pi6)),
            (5, (4.0 * pi6, 5.0 * pi6)),
            (6, (5.0 * pi6, 6.0 * pi6)),
        ]);

        assert!(segment <= 6 && segment > 0);
        let (start, end) = angles_from_segment.get(&segment).unwrap_or(&(0.0, 0.0));
        let (a, b) = self.polar(x, y, r, *start);
        let (c, d) = self.polar(x, y, r, *end);
        self.path.push_str(&format!(
            "M {a} {b} A {r} {r} 0 0 {ccw} {c} {d} ",
            a = a,
            b = b,
            c = c,
            d = d,
            r = r,
            ccw = ccw,
        ));
    }
}

pub struct Tile {
    lines: Vec<Line>,
    arcs: Vec<Segment>,
}

impl Tile {
    pub fn to_string(&self) -> String {
        let mut path = Path::new("black".to_string(), 1.0, 1.0);
        // self.lines.iter().map(|line| path.line(line));
        for line in self.lines.iter() {
            path.line(line);
        }
        return path.to_string();
    }
}

pub trait S1 {
    fn new(origin: Point) -> Self;
}

impl S1 for Tile {
    fn new(origin: Point) -> Self {
        let mut lines: Vec<Line> = Vec::new();
        let mut arcs: Vec<Segment> = Vec::new();

        // Make a consts and add addition, so I can add in the origin with +
        lines.push(Line {
            start: Point {
                x: origin.x,
                y: origin.y,
            },
            delta: Point { x: U, y: 0.0 },
        });

        lines.push(Line {
            start: Point {
                x: origin.x + U,
                y: origin.y,
            },
            delta: Point { x: -M, y: H },
        });
        lines.push(Line {
            start: Point {
                x: origin.x + M,
                y: origin.y + H,
            },
            delta: Point { x: -M, y: -H },
        });
        return Tile {
            lines: lines,
            arcs: arcs,
        };
    }
}

pub struct RenderSvg {
    pub style: Option<String>,
    tiles: Vec<Vec<Tile>>,
    width: f64,
    height: f64,
}

impl RenderSvg {
    pub fn new() -> Self {
        let mut tiles: Vec<Vec<Tile>> = Vec::new();

        for i in 1..2 {
            let mut row: Vec<Tile> = Vec::new();
            for j in 1..2 {
                let tile: Tile = S1::new(Point {
                    x: U * i as f64,
                    y: U * f64::sqrt(3.0) / 2.0 * j as f64,
                });
                row.push(tile);
            }
            tiles.push(row);
        }

        let mut obj = Self {
            style: None,
            tiles: tiles,
            width: 1000.0,
            height: 1000.0,
        };

        return obj;
    }

    // Path commands
    // https://github.com/bodoni/svg/blob/master/src/node/element/path/data.rs
    pub fn render(self, file_name: String) -> Result<(), Box<dyn Error>> {
        let mut svg = String::new();

        let margin = 10.0;
        let w = self.width;
        let h = self.height;

        let view_box = format!(
            "viewBox=\"-{m} -{m} {w} {h}\" ",
            w = w + margin,
            h = h + margin,
            m = margin
        );

        svg.push_str(&format!("<svg id=\"maze\" xmlns=\"http://www.w3.org/2000/svg\" {view_box} width=\"{w}px\" height=\"{h}px\" fill-opacity=\"0\" stroke=\"black\">", w = w + 2.0 * margin, h = h + 2.0 * margin, view_box = view_box));

        for row in &self.tiles {
            for tile in row {
                println!("> {}", tile.to_string());
                svg.push_str(&tile.to_string());
            }
        }
        svg.push_str("</svg>");

        fs::write(file_name, svg.as_bytes())?;
        Ok(())
    }
}

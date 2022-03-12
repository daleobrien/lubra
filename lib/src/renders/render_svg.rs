use std::collections::HashMap;
use std::error::Error;
use std::fs;

use crate::renders::constants::{M, U};
use crate::renders::primatives::{Coordinate, Line, Point, Segment};
use crate::renders::shapes::{
    A1_A, A1_V, A2_A, A2_V, A3_A, A3_V, LONG_L1_A, LONG_L1_V, LONG_L2_A, LONG_L2_V, LONG_L3_A,
    LONG_L3_V, SHORT_S1_A, SHORT_S1_V, SHORT_S2_A, SHORT_S2_V, SHORT_S3_A, SHORT_S3_V, TRIANGLE_A,
    TRIANGLE_V,
};

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
    pub fn arc(&mut self, arc: &Segment) {
        let ccw = "1";
        let x = arc.centre.x;
        let y = arc.centre.y;
        let r = arc.radius;
        let segment = arc.segment;

        let pi6 = 2.0 * std::f64::consts::FRAC_PI_6;
        let angles_from_segment: HashMap<u64, (f64, f64)> = HashMap::from([
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
    coordinate: Coordinate,
}

impl Tile {
    pub fn to_string(&self) -> String {
        let mut path = Path::new("black".to_string(), 1.0, 1.0);
        // self.lines.iter().map(|line| path.line(line));
        for line in self.lines.iter() {
            path.line(line);
        }
        for arc in self.arcs.iter() {
            path.arc(arc);
        }
        return path.to_string();
    }
}

pub trait S1 {
    fn new(coordinate: Coordinate) -> Self;
}

pub trait S2 {
    fn new(coordinate: Coordinate) -> Self;
}

pub trait S3 {
    fn new(coordinate: Coordinate) -> Self;
}

pub trait L1 {
    fn new(coordinate: Coordinate) -> Self;
}

pub trait L2 {
    fn new(coordinate: Coordinate) -> Self;
}

pub trait L3 {
    fn new(coordinate: Coordinate) -> Self;
}

pub trait A1 {
    fn new(coordinate: Coordinate) -> Self;
}

pub trait A2 {
    fn new(coordinate: Coordinate) -> Self;
}

pub trait A3 {
    fn new(coordinate: Coordinate) -> Self;
}

pub fn create_triangle(flipped: bool) -> [Line; 3] {
    return if flipped { TRIANGLE_V } else { TRIANGLE_A };
}

pub fn calculate_origin_and_orientation(coordinate: &Coordinate) -> (Point, bool) {
    let origin = Point {
        x: M * coordinate.i as f64,
        y: U * f64::sqrt(3.0) / 2.0 * coordinate.j as f64,
    };
    let flipped = (coordinate.j + coordinate.i) % 2 == 0;

    return (origin, flipped);
}

impl S1 for Tile {
    fn new(coordinate: Coordinate) -> Self {
        let mut lines: Vec<Line> = Vec::new();
        let arcs: Vec<Segment> = Vec::new();
        let (origin, flipped) = calculate_origin_and_orientation(&coordinate);

        create_triangle(flipped).map(|line| lines.push(line + origin));
        lines.push(if flipped { SHORT_S1_V } else { SHORT_S1_A } + origin);

        return Tile {
            lines,
            arcs,
            coordinate,
        };
    }
}

impl S2 for Tile {
    fn new(coordinate: Coordinate) -> Self {
        let mut lines: Vec<Line> = Vec::new();
        let arcs: Vec<Segment> = Vec::new();
        let (origin, flipped) = calculate_origin_and_orientation(&coordinate);

        create_triangle(flipped).map(|line| lines.push(line + origin));
        lines.push(if flipped { SHORT_S2_V } else { SHORT_S2_A } + origin);

        return Tile {
            lines,
            arcs,
            coordinate,
        };
    }
}

impl S3 for Tile {
    fn new(coordinate: Coordinate) -> Self {
        let mut lines: Vec<Line> = Vec::new();
        let arcs: Vec<Segment> = Vec::new();
        let (origin, flipped) = calculate_origin_and_orientation(&coordinate);

        create_triangle(flipped).map(|line| lines.push(line + origin));
        lines.push(if flipped { SHORT_S3_V } else { SHORT_S3_A } + origin);

        return Tile {
            lines,
            arcs,
            coordinate,
        };
    }
}

impl L1 for Tile {
    fn new(coordinate: Coordinate) -> Self {
        let mut lines: Vec<Line> = Vec::new();
        let arcs: Vec<Segment> = Vec::new();
        let (origin, flipped) = calculate_origin_and_orientation(&coordinate);

        create_triangle(flipped).map(|line| lines.push(line + origin));
        lines.push(if flipped { LONG_L1_V } else { LONG_L1_A } + origin);

        return Tile {
            lines,
            arcs,
            coordinate,
        };
    }
}

impl L2 for Tile {
    fn new(coordinate: Coordinate) -> Self {
        let mut lines: Vec<Line> = Vec::new();
        let arcs: Vec<Segment> = Vec::new();
        let (origin, flipped) = calculate_origin_and_orientation(&coordinate);

        create_triangle(flipped).map(|line| lines.push(line + origin));
        lines.push(if flipped { LONG_L2_V } else { LONG_L2_A } + origin);

        return Tile {
            lines,
            arcs,
            coordinate,
        };
    }
}

impl L3 for Tile {
    fn new(coordinate: Coordinate) -> Self {
        let mut lines: Vec<Line> = Vec::new();
        let arcs: Vec<Segment> = Vec::new();
        let (origin, flipped) = calculate_origin_and_orientation(&coordinate);

        create_triangle(flipped).map(|line| lines.push(line + origin));
        lines.push(if flipped { LONG_L3_V } else { LONG_L3_A } + origin);

        return Tile {
            lines,
            arcs,
            coordinate,
        };
    }
}

impl A1 for Tile {
    fn new(coordinate: Coordinate) -> Self {
        let mut lines: Vec<Line> = Vec::new();
        let mut arcs: Vec<Segment> = Vec::new();
        let (origin, flipped) = calculate_origin_and_orientation(&coordinate);

        create_triangle(flipped).map(|line| lines.push(line + origin));
        arcs.push(if flipped { A1_V } else { A1_A } + origin);

        return Tile {
            lines,
            arcs,
            coordinate,
        };
    }
}

impl A2 for Tile {
    fn new(coordinate: Coordinate) -> Self {
        let mut lines: Vec<Line> = Vec::new();
        let mut arcs: Vec<Segment> = Vec::new();
        let (origin, flipped) = calculate_origin_and_orientation(&coordinate);

        create_triangle(flipped).map(|line| lines.push(line + origin));
        arcs.push(if flipped { A2_V } else { A2_A } + origin);

        return Tile {
            lines,
            arcs,
            coordinate,
        };
    }
}

impl A3 for Tile {
    fn new(coordinate: Coordinate) -> Self {
        let mut lines: Vec<Line> = Vec::new();
        let mut arcs: Vec<Segment> = Vec::new();
        let (origin, flipped) = calculate_origin_and_orientation(&coordinate);

        create_triangle(flipped).map(|line| lines.push(line + origin));
        arcs.push(if flipped { A3_V } else { A3_A } + origin);

        return Tile {
            lines,
            arcs,
            coordinate,
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

        for i in 1..8 {
            let mut row: Vec<Tile> = Vec::new();
            for j in 1..6 {
                let tile: Tile = A3::new(Coordinate { i, j });
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
                // println!("> {}", tile.to_string());
                svg.push_str(&tile.to_string());
            }
        }
        svg.push_str("</svg>");

        fs::write(file_name, svg.as_bytes())?;
        Ok(())
    }
}

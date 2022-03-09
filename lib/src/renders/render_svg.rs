use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::io::prelude::*;
use std::ptr::addr_of_mut;

pub enum TileType {
    S1,
    S2,
    S3,
    A1,
    A2,
    A3,
    L1,
    L2,
    L3,
    C1,
    C2,
    C3,
    X1,
    Z1,
}

pub struct Point {
    x: f64,
    y: f64,
}

pub struct Coordinate {
    i: u32,
    j: u32,
}

pub struct Path {
    stroke: String,
    stroke_width: f64,
    path: String,
    opacity: f64,
    unitSize: f64,
}

impl Path {
    pub fn new(stroke: String, stroke_width: f64, opacity: f64, unitSize: f64) -> Path {
        Path {
            stroke: stroke,
            stroke_width: stroke_width,
            path: String::new(),
            opacity: opacity,
            unitSize: unitSize,
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

    /*

    <polyline points="60, 110 65, 120 70, 115 75, 130 80, 125 85, 140 90, 135 95, 150 100, 145"/>

     i.e. (0,0), (1,1), and (2,2) -> <polygon points="0, 0 1, 1 2, 2"/>

     <circle cx="25" cy="75" r="20"/>

     <path d="M 10 10 H 90 V 90 H 10 L 10 10"/>

     # line, horizontal, vertical
     H x (or) h dx
     V y (or) v dy

     # Bezier curve (first point already defined, last point drawn)
     C x1 y1, x2 y2, x y
     (or)
     c dx1 dy1, dx2 dy2, dx dy

    */
    pub fn append(&mut self, path: &str) {
        self.path.push_str(path);
    }
    pub fn s0(&mut self, x: f64, y: f64) {
        self.path.push_str(&format!("M {},{} ", x, y));
    }

    fn polar(&self, x: f64, y: f64, radius: f64, angle: f64) -> (f64, f64) {
        let (a, b) = f64::sin_cos(angle);
        return (x + (radius * b), y + (radius * a));
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
    path: Path,
    position: Coordinate,
    unitSize: f64,
    flipped: bool,
    origin: Point,
    tileType: TileType,
}

impl Tile {
    pub fn new(stroke: String, position: Coordinate, tileType: TileType) -> Tile {
        let unitSize = 100.0;

        let origin = Point {
            // x: s / 2.0 * (position.i as f64 + (position.j % 2) as f64 / 1.0),
            x: unitSize / 2.0 * (position.i as f64),
            y: unitSize * (position.j as f64 * f64::sqrt(3.0) / 2.0),
        };
        let flipped = (position.j + position.i) % 2 == 1;
        println!("{} {} -> {}", origin.x, origin.y, flipped);

        // stroke: String, stroke_width: f64, opacity: f64
        let mut path = Path::new("black".to_string(), 3.0, 1.0, unitSize);

        let h: f64 = unitSize * f64::sqrt(3.0) / 2.0;

        // if flipped {
        //     path.append(&format!(
        //         " M {x} {y} h {s} l {c} {h} z ",
        //         s = unitSize,
        //         c = -unitSize / 2.0,
        //         x = origin.x,
        //         y = origin.y,
        //         h = h
        //     ));
        // } else {
        //     path.append(&format!(
        //         "M {x} {y} h {s} l {c} {h} z ",
        //         s = unitSize,
        //         c = -unitSize / 2.0,
        //         x = origin.x,
        //         y = origin.y + h,
        //         h = -h
        //     ));
        // }
        let radius: f64 = unitSize / 2.0;

        match tileType {
            TileType::S1 => {}
            TileType::S2 => {}
            TileType::S3 => {}
            TileType::A1 => {
                if flipped {
                    path.arc(origin.x, origin.y, radius, 1, true);
                } else {
                    path.arc(origin.x, origin.y + h, radius, 6, true);
                }
            }
            TileType::A2 => {
                if flipped {
                    path.arc(origin.x + unitSize, origin.y, radius, 3, true);
                } else {
                    path.arc(origin.x + unitSize, origin.y + h, radius, 4, true);
                }
            }
            TileType::A3 => {
                if flipped {
                    path.arc(origin.x + unitSize / 2.0, origin.y + h, radius, 5, true);
                } else {
                    path.arc(origin.x + unitSize / 2.0, origin.y, radius, 2, true);
                }
            }
            TileType::C1 => {
                if flipped {
                    path.arc(origin.x + unitSize / 2.0, origin.y + h, radius, 5, true);
                } else {
                    path.arc(origin.x + unitSize / 2.0, origin.y, radius, 2, true);
                }
                path.append(&format!(
                    "M {x} {y} v {h} z ",
                    x = origin.x + unitSize / 2.0,
                    y = origin.y,
                    h = h
                ));
            }
            TileType::C2 => {}
            TileType::C3 => {}
            TileType::L1 => {
                path.append(&format!(
                    "M {x} {y} v {h} z ",
                    x = origin.x + unitSize / 2.0,
                    y = origin.y,
                    h = h
                ));
            }
            TileType::L2 => {
                let c = 3.0 * unitSize / 4.0;
                let y = if flipped { origin.y } else { origin.y + h };
                let d = if flipped {
                    unitSize / 4.0 * f64::sqrt(3.0)
                } else {
                    -unitSize / 4.0 * f64::sqrt(3.0)
                };
                path.append(&format!(
                    "M {x} {y} l {c} {d} z ",
                    x = origin.x,
                    y = y,
                    c = c,
                    d = d,
                ));
            }
            TileType::L3 => {
                let d = if flipped { -h / 2.0 } else { h / 2.0 };
                path.append(&format!(
                    "M {x} {y} l {c} {d} z ",
                    x = origin.x + unitSize / 4.0,
                    y = origin.y + h / 2.0,
                    c = 3.0 / 4.0 * unitSize,
                    d = d,
                ));
            }
            TileType::X1 => {}
            TileType::Z1 => {}
        }

        let mut obj = Self {
            path: path,
            position: position,
            unitSize: unitSize,
            flipped: flipped,
            origin: origin,
            tileType: tileType,
        };

        return obj;
    }

    pub fn to_string(&self) -> String {
        return self.path.to_string();
    }
    // fn create(&mut self, flags: u8) {
    //     self.path.arc(x, y, r, segment, ccw);
    // }
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

        let validTiles = [TileType::A1, TileType::A2, TileType::A3, TileType::C1];

        for i in 1..20 {
            let mut row: Vec<Tile> = Vec::new();
            for j in 1..20 {
                let mut choice: TileType = TileType::C1;
                match (i * j) % 7 {
                    0 => choice = TileType::A2,
                    1 => choice = TileType::A1,
                    2 => choice = TileType::A3,
                    4 => choice = TileType::L1,
                    5 => choice = TileType::L2,
                    6 => choice = TileType::L3,
                    7 => choice = TileType::C1,
                    _ => {}
                }
                let tile = Tile::new("black".to_string(), Coordinate { i: i, j: j }, choice);
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

        // // Test line to begin with
        // // obj.add_to_path("walls", " M 0 0 h 100 v 100 h -100 z");
        // // quadratic curve
        // //obj.add_to_path("walls", "M 10 0 q 100 100 200 0");
        //
        // obj.add_to_path("walls", " M 0 0 h 200 l -100 173 z");
        //
        // // SQRT 3 -> 1.732050807568877293527446341505872366943
        //
        // // Make the function even more abstract!, just which vertice
        // obj.paths
        //     .get_mut("walls")
        //     .unwrap()
        //     .arc(100.0, 173.0, 70.0, 5, true);
        //
        // obj.paths
        //     .get_mut("walls")
        //     .unwrap()
        //     .arc(100.0, 173.0, 130.0, 5, true);

        return obj;
    }

    // pub fn add_to_path(&mut self, key: &str, text: &str) {
    //     let tile = self.tiles.get_mut(key).unwrap();
    //     tile.append(text);
    // }
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

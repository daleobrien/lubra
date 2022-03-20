use std::collections::HashMap;
use std::error::Error;
use std::fs;

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use crate::renders::constants::{
    TileType, H, L01, L02, L07, L08, L102, L103, L106, L11, L110, L116, L117, L12, L120, L123, L13,
    L14, L15, L16, L17, L18, L19, L20, L21, L22, L23, L24, L25, L26, L31, L32, L35, L36, L37, L38,
    L39, L40, L41, L42, L43, L44, L45, L46, L47, L48, L49, L50, L51, L52, L53, L54, L56, L57, L59,
    L60, L62, L63, L66, L67, L69, L72, L74, L76, L77, L80, L81, L83, L92, L96, L98, L99, M, U, W,
    WALL,
};
use crate::renders::primatives::{Coordinate, Line, Point, Radius, Segment, SegmentOffset};
use crate::renders::shapes::{
    A1_A, A1_V, A2_A, A2_V, A3_A, A3_V, N1_A, N1_V, N2_A, N2_V, N3_A, N3_V, T1_A, T1_V, T2_A,
    T2_ARC, T2_V, T3_A, T3_V, TRIANGLE_A, TRIANGLE_V,
};
use crate::renders::tile_shapes_all::ALL_TILES;
use crate::renders::tile_shapes_primitive::PRIMITIVE_TILES;

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
        return format!("<path stroke-opacity=\"{opacity}\" stroke-width=\"{w}\" stroke-linecap=\"round\" stroke=\"{colour}\" d=\"{d}\"/>",
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

        // assert!(segment <= 6 && segment > 0);

        let (start, end) = angles_from_segment
            .get(&(segment % 7))
            .unwrap_or(&(0.0, 0.0));
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

    pub fn semi(&mut self, arc: &Segment) {
        let ccw = "1";
        let x = arc.centre.x;
        let y = arc.centre.y;
        let r = arc.radius;
        let segment = arc.segment;

        let pi6 = 2.0 * std::f64::consts::FRAC_PI_6;
        let angles_from_segment: HashMap<u64, (f64, f64)> = HashMap::from([
            (1, (0.0 * pi6, 3.0 * pi6)),
            (2, (1.0 * pi6, 4.0 * pi6)),
            (3, (2.0 * pi6, 5.0 * pi6)),
            (4, (3.0 * pi6, 6.0 * pi6)),
            (5, (4.0 * pi6, 7.0 * pi6)),
            (6, (5.0 * pi6, 8.0 * pi6)),
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
    faint: Vec<Line>,
    arcs: Vec<Segment>,
    farcs: Vec<Segment>,
    semi: Vec<Segment>,
}

impl Distribution<TileType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> TileType {
        match rng.gen_range(0..=20) {
            // rand 0.8
            // 0 => TileType::S1,
            // 1 => TileType::S2,
            // 2 => TileType::S3,
            0 => TileType::Y1,
            1 => TileType::Y2,
            2 => TileType::Y3,
            3 => TileType::A1,
            4 => TileType::A2,
            5 => TileType::A3,
            6 => TileType::L1,
            7 => TileType::L2,
            8 => TileType::L3,
            9 => TileType::N1,
            10 => TileType::N2,
            11 => TileType::N3,
            12 => TileType::C1,
            13 => TileType::C2,
            14 => TileType::C3,
            15 => TileType::T1,
            16 => TileType::T2,
            17 => TileType::T3,
            18 => TileType::X1,
            19 => TileType::X2,
            20 => TileType::X3,
            _ => TileType::Z1,
        }
    }
}

impl Tile {
    // Change to a list of TileTypes, Could be a fixed length, since it can't be londer than 0
    // Just fill the remaining with X3
    fn new(coordinate: Coordinate, tiles: [&TileType; 6]) -> Self {
        let mut lines: Vec<Line> = Vec::new();
        let mut faint: Vec<Line> = Vec::new();
        let mut arcs: Vec<Segment> = Vec::new();
        let mut farcs: Vec<Segment> = Vec::new();
        let mut semi: Vec<Segment> = Vec::new();

        let (origin, flipped) = calculate_origin_and_orientation(&coordinate);

        for tile in tiles {
            let only_an_arc: Option<Segment> = match tile {
                TileType::A1 => Some(if flipped { A1_V } else { A1_A } + origin),
                TileType::A2 => Some(if flipped { A2_V } else { A2_A } + origin),
                TileType::A3 => Some(if flipped { A3_V } else { A3_A } + origin),
                TileType::C1 => Some(if flipped { A1_V } else { A1_A } + origin),
                TileType::C2 => Some(if flipped { A2_V } else { A2_A } + origin),
                TileType::C3 => Some(if flipped { A3_V } else { A3_A } + origin),
                _ => None,
            };

            match only_an_arc {
                Some(arc) => {
                    arcs.push(arc + Radius { r: -W });
                    arcs.push(arc + Radius { r: W });
                }
                _ => {}
            }

            match tile {
                // TileType::S1 => {
                //     if flipped {
                //         lines.push(L05 + origin);
                //         lines.push(L06 + origin);
                //         semi.push(M1_V + origin);
                //     } else {
                //         lines.push(L29 + origin);
                //         lines.push(L30 + origin);
                //         semi.push(M1_A + origin);
                //     }
                // }
                // TileType::S2 => {
                //     if flipped {
                //         lines.push(L03 + origin);
                //         lines.push(L04 + origin);
                //         semi.push(M2_V + origin);
                //     } else {
                //         lines.push(L27 + origin);
                //         lines.push(L28 + origin);
                //         semi.push(M2_A + origin);
                //     }
                // }
                // TileType::S3 => {
                //     if flipped {
                //         lines.push(L10 + origin);
                //         lines.push(L09 + origin);
                //         semi.push(M3_V + origin);
                //     } else {
                //         lines.push(L34 + origin);
                //         lines.push(L33 + origin);
                //         semi.push(M3_A + origin);
                //     }
                // }
                TileType::T1 => {
                    if flipped {
                        semi.push(T1_V + origin);
                    } else {
                        semi.push(T1_A + origin);
                    }
                }
                TileType::T2 => {
                    if flipped {
                        if tiles.contains(&&TileType::L1) {
                            farcs.push(T2_ARC + origin + SegmentOffset { offset: 3 });
                            farcs.push(T2_ARC + origin + SegmentOffset { offset: 4 });
                            arcs.push(T2_ARC + origin + SegmentOffset { offset: 5 });
                        } else {
                            semi.push(T2_V + origin);
                        }
                    } else {
                        if tiles.contains(&&TileType::L1) {
                            farcs.push(T2_ARC + origin + SegmentOffset { offset: 3 });
                            farcs.push(T2_ARC + origin + SegmentOffset { offset: 4 });
                            arcs.push(T2_ARC + origin + SegmentOffset { offset: 2 });
                        } else {
                            semi.push(T2_A + origin);
                        }
                    }
                }
                TileType::T3 => {
                    if flipped {
                        semi.push(T3_V + origin);
                    } else {
                        semi.push(T3_A + origin);
                    }
                }
                TileType::N1 => {
                    if flipped {
                        semi.push(N1_V + origin);
                    } else {
                        semi.push(N1_A + origin);
                    }
                }
                TileType::N2 => {
                    if flipped {
                        semi.push(N2_V + origin);
                    } else {
                        semi.push(N2_A + origin);
                    }
                }
                TileType::N3 => {
                    if flipped {
                        semi.push(N3_V + origin);
                    } else {
                        semi.push(N3_A + origin);
                    }
                }
                TileType::L1 => {
                    lines.push(L49 + origin);
                    lines.push(L52 + origin);
                }
                TileType::L2 => {
                    if flipped {
                        lines.push(L51 + origin);
                        lines.push(L54 + origin);
                    } else {
                        lines.push(L60 + origin);
                        lines.push(L57 + origin);
                    }
                }
                TileType::L3 => {
                    if flipped {
                        lines.push(L50 + origin);
                        lines.push(L53 + origin);
                    } else {
                        lines.push(L59 + origin);
                        lines.push(L56 + origin);
                    }
                }
                TileType::Y1 => {
                    if flipped {
                        lines.push(L50 + origin);
                        lines.push(L53 + origin);

                        lines.push(L74 + origin);
                        lines.push(L66 + origin);
                        lines.push(L02 + origin);

                        faint.push(L92 + origin);
                        faint.push(L103 + origin);
                    } else {
                        lines.push(L59 + origin);
                        lines.push(L56 + origin);
                        //
                        lines.push(L80 + origin);
                        lines.push(L26 + origin);
                        lines.push(L72 + origin);
                        //
                        faint.push(L102 + origin);
                        faint.push(L116 + origin);
                    }
                }
                TileType::Y2 => {
                    if flipped {
                        lines.push(L49 + origin);
                        lines.push(L52 + origin);

                        lines.push(L62 + origin);
                        lines.push(L77 + origin);
                        lines.push(L12 + origin);

                        faint.push(L106 + origin);
                        faint.push(L96 + origin);
                    } else {
                        lines.push(L49 + origin);
                        lines.push(L52 + origin);
                        lines.push(L67 + origin);
                        lines.push(L83 + origin);
                        lines.push(L36 + origin);
                        faint.push(L120 + origin);
                        faint.push(L98 + origin);
                    }
                }
                TileType::Y3 => {
                    if flipped {
                        lines.push(L51 + origin);
                        lines.push(L54 + origin);
                        lines.push(L76 + origin);
                        lines.push(L63 + origin);
                        lines.push(L08 + origin);
                        faint.push(L123 + origin);
                        faint.push(L110 + origin);
                    } else {
                        lines.push(L60 + origin);
                        lines.push(L57 + origin);
                        lines.push(L32 + origin);
                        lines.push(L81 + origin);
                        lines.push(L69 + origin);

                        faint.push(L117 + origin);
                        faint.push(L99 + origin);
                    }
                }

                //
                TileType::C1 => {
                    if flipped {
                        lines.push(L07 + origin);
                        lines.push(L08 + origin);
                        lines.push(L18 + origin);
                        lines.push(L17 + origin);
                        faint.push(L21 + origin);
                        faint.push(L22 + origin);
                    } else {
                        lines.push(L31 + origin);
                        lines.push(L32 + origin);
                        lines.push(L42 + origin);
                        lines.push(L41 + origin);
                        faint.push(L45 + origin);
                        faint.push(L46 + origin);
                    }
                }
                TileType::C2 => {
                    if flipped {
                        lines.push(L01 + origin);
                        lines.push(L02 + origin);
                        lines.push(L15 + origin);
                        lines.push(L16 + origin);
                        faint.push(L19 + origin);
                        faint.push(L20 + origin);
                    } else {
                        lines.push(L25 + origin);
                        lines.push(L26 + origin);
                        lines.push(L40 + origin);
                        lines.push(L39 + origin);
                        faint.push(L43 + origin);
                        faint.push(L44 + origin);
                    }
                }
                TileType::C3 => {
                    if flipped {
                        lines.push(L11 + origin);
                        lines.push(L12 + origin);
                        lines.push(L13 + origin);
                        lines.push(L14 + origin);
                        faint.push(L23 + origin);
                        faint.push(L24 + origin);
                    } else {
                        lines.push(L35 + origin);
                        lines.push(L36 + origin);
                        lines.push(L37 + origin);
                        lines.push(L38 + origin);
                        faint.push(L47 + origin);
                        faint.push(L48 + origin);
                    }
                }
                TileType::X1 => {
                    arcs.push(if flipped { A1_V } else { A1_A } + origin + Radius { r: -W });
                    arcs.push(if flipped { A2_V } else { A2_A } + origin + Radius { r: -W });
                    arcs.push(if flipped { A3_V } else { A3_A } + origin + Radius { r: -W });
                }
                TileType::X2 => {
                    if flipped {
                        lines.push(L50 + origin);
                        lines.push(L53 + origin);

                        lines.push(L74 + origin);
                        lines.push(L66 + origin);
                        lines.push(L02 + origin);

                        faint.push(L92 + origin);
                        faint.push(L103 + origin);

                        // faint.push(L63 + origin);
                        lines.push(L08 + origin);
                        lines.push(L07 + origin);
                    } else {
                        lines.push(L59 + origin);
                        lines.push(L56 + origin);
                        //
                        lines.push(L80 + origin);
                        lines.push(L26 + origin);
                        lines.push(L72 + origin);
                        //
                        faint.push(L102 + origin);
                        faint.push(L116 + origin);

                        lines.push(L31 + origin);
                        lines.push(L32 + origin);
                        //faint.push(L69 + origin);
                    }
                }
                TileType::X3 => {
                    // This is always empty
                }
                TileType::Z1 => {
                    create_triangle(flipped).map(|line| lines.push(line + origin));
                }
                _ => {}
            }
        }

        return Tile {
            lines,
            faint,
            arcs,
            semi,
            farcs,
        };
    }

    pub fn add_to_path(&self, path: &mut Path, faint_path: &mut Path) {
        // self.lines.iter().map(|line| path.line(line));
        for line in self.lines.iter() {
            path.line(line);
        }

        for arc in self.arcs.iter() {
            path.arc(arc);
        }
        for semi in self.semi.iter() {
            path.semi(semi);
        }

        for line in self.faint.iter() {
            faint_path.line(line);
        }
        for farc in self.farcs.iter() {
            faint_path.arc(farc);
        }
    }
}

pub fn create_triangle(flipped: bool) -> [Line; 3] {
    return if flipped { TRIANGLE_V } else { TRIANGLE_A };
}

pub fn calculate_origin_and_orientation(coordinate: &Coordinate) -> (Point, bool) {
    let x_margin = 50.0;
    let y_margin = 0.0;
    let origin = Point {
        x: (M + x_margin) * coordinate.i as f64,
        y: (U + y_margin) * f64::sqrt(3.0) / 2.0 * coordinate.j as f64,
    };
    let flipped = (coordinate.j + coordinate.i) % 2 == 1;

    return (origin, flipped);
}

pub struct RenderSvg {
    pub style: Option<String>,
    tiles: Vec<Vec<Tile>>,
    walls: Vec<Vec<Tile>>,
    width: f64,
    height: f64,
}

impl RenderSvg {
    pub fn new() -> Self {
        let mut tiles: Vec<Vec<Tile>> = Vec::new();
        let mut walls: Vec<Vec<Tile>> = Vec::new();

        let obj = Self {
            style: None,
            tiles,
            walls,
            width: 0f64,
            height: 0f64,
        };

        return obj;
    }

    pub fn display_one_tile(&mut self) {
        let mut row_path: Vec<Tile> = Vec::new();
        let mut row_wall: Vec<Tile> = Vec::new();

        let tile = [
            &TileType::L1,
            &TileType::T2,
            &TileType::X3,
            &TileType::X3,
            &TileType::X3,
            &TileType::X3,
        ];

        let path1: Tile = Tile::new(Coordinate { i: 0, j: 0 }, tile);
        let path2: Tile = Tile::new(Coordinate { i: 1, j: 0 }, tile);

        row_path.push(path1);
        row_path.push(path2);

        let wall1: Tile = Tile::new(Coordinate { i: 0, j: 0 }, WALL);
        let wall2: Tile = Tile::new(Coordinate { i: 1, j: 0 }, WALL);

        row_wall.push(wall1);
        row_wall.push(wall2);

        self.tiles.push(row_path);
        self.walls.push(row_wall);

        self.width = 2.0 * (M + 50.0);
        self.height = H + 0.0;
    }

    pub fn display_all_tiles(&mut self) {
        let n: u32 = 26;
        let m: u32 = (ALL_TILES.len() as u32) / 3u32;

        let mut tile_index = 0;

        for j in 0..m {
            let mut row_path: Vec<Tile> = Vec::new();
            let mut row_wall: Vec<Tile> = Vec::new();
            if j % 2 != 0 {
                continue;
            }
            for i in 0..n {
                if i % 2 != 0 {
                    continue;
                }

                if tile_index + 1 >= ALL_TILES.len() {
                    continue;
                }
                let path: Tile = Tile::new(Coordinate { i, j }, ALL_TILES[tile_index]);
                tile_index += 1;
                let wall: Tile = Tile::new(Coordinate { i, j }, WALL);
                row_path.push(path);
                row_wall.push(wall);
            }
            self.tiles.push(row_path);
            self.walls.push(row_wall);
        }

        self.width = (n + 1) as f64 * (M + 50.0);
        self.height = (m) as f64 * (H + 00.0);
    }

    pub fn display_basic_tiles(&mut self) {
        //

        let n: u32 = 18;
        let m: u32 = (PRIMITIVE_TILES.len() as u32) / 3u32;

        let mut tile_index = 0;

        for j in 0..m {
            let mut row_path: Vec<Tile> = Vec::new();
            let mut row_wall: Vec<Tile> = Vec::new();
            if j % 2 != 0 {
                continue;
            }
            for i in 0..n {
                if i % 3 != 0 {
                    continue;
                }
                // let tile_type: TileType = rand::random();
                // let path: Tile = Tile::new(Coordinate { i, j }, tile_type);
                let tile_type = &PRIMITIVE_TILES[tile_index];
                let path: Tile = Tile::new(
                    Coordinate { i, j },
                    [
                        tile_type,
                        &TileType::X3,
                        &TileType::X3,
                        &TileType::X3,
                        &TileType::X3,
                        &TileType::X3,
                    ],
                );
                let wall: Tile = Tile::new(
                    Coordinate { i, j },
                    [
                        &TileType::Z1,
                        &TileType::Z1,
                        &TileType::Z1,
                        &TileType::Z1,
                        &TileType::Z1,
                        &TileType::Z1,
                    ],
                );
                row_path.push(path);
                row_wall.push(wall);

                tile_index += 1;
            }
            self.tiles.push(row_path);
            self.walls.push(row_wall);
        }

        self.width = (n + 1) as f64 * M;
        self.height = (m) as f64 * H;
    }

    // Path commands
    // https://github.com/bodoni/svg/blob/master/src/node/element/path/data.rs
    pub fn render(self, file_name: String) -> Result<(), Box<dyn Error>> {
        let mut svg = String::new();

        let margin = 50.0;
        let w = self.width;
        let h = self.height;

        let view_box = format!(
            "viewBox=\"-{m} -{m} {w} {h}\" ",
            w = w + 2.0 * margin,
            h = h + 2.0 * margin,
            m = margin
        );

        svg.push_str(&format!(
            "<svg id=\"maze\" xmlns=\"http://www.w3.org/2000/svg\" {view_box} width=\"{w}px\" height=\"{h}px\" fill-opacity=\"0\" stroke-linecap=\"round\" stroke=\"black\" >",
            w = w + 2.0 * margin, h = h + 2.0 * margin, view_box = view_box));

        let mut path = Path::new("black".to_string(), 3.0, 1.0);
        let mut faint_path = Path::new("gray".to_string(), 3.0, 0.2);
        let mut wall = Path::new("brown".to_string(), 3.0, 0.3);

        for row in &self.tiles {
            for tile in row {
                tile.add_to_path(&mut path, &mut faint_path);
            }
        }
        for row in &self.walls {
            for tile in row {
                tile.add_to_path(&mut wall, &mut faint_path)
            }
        }

        svg.push_str(&faint_path.to_string());
        svg.push_str(&path.to_string());
        svg.push_str(&wall.to_string());
        svg.push_str("</svg>");

        fs::write(file_name, svg.as_bytes())?;
        Ok(())
    }
}

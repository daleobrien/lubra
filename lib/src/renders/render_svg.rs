use std::error::Error;
use std::fs;

use rand::seq::SliceRandom;

use crate::renders::constants::{
    TileFlags, H, L01, L02, L07, L08, L102, L103, L106, L11, L110, L116, L117, L12, L120, L123,
    L13, L14, L15, L16, L17, L18, L19, L20, L21, L22, L23, L24, L25, L26, L31, L32, L35, L36, L37,
    L38, L39, L40, L41, L42, L43, L44, L45, L46, L47, L48, L49, L50, L51, L52, L53, L54, L56, L57,
    L59, L60, L62, L63, L66, L67, L69, L72, L74, L76, L77, L80, L81, L83, L92, L96, L98, L99, M, U,
    W,
};
use crate::renders::primatives::{Coordinate, Line, Point, Radius, Segment, SegmentOffset};
use crate::renders::shapes::{
    A1_A, A1_V, A2_A, A2_V, A3_A, A3_V, N1_A, N1_V, N2_A, N2_V, N3_A, N3_V, T1_A, T1_A_ARC, T1_V,
    T1_V_ARC, T2_A, T2_ARC, T2_V, T3_A, T3_ARC, T3_V, TRIANGLE_A, TRIANGLE_V,
};
use crate::renders::svg_path::Path;
use crate::renders::tile_subsets::{match_side_1, match_side_1_2_3};

pub struct Tile {
    lines: Vec<Line>,
    faint: Vec<Line>,
    arcs: Vec<Segment>,
    farcs: Vec<Segment>,
    semi: Vec<Segment>,
    fsemi: Vec<Segment>,
}

impl Tile {
    // Change to a list of TileTypes, Could be a fixed length, since it can't be londer than 0
    // Just fill the remaining with X3
    fn new(coordinate: Coordinate, mask: TileFlags) -> Self {
        let mut lines: Vec<Line> = Vec::new();
        let mut faint: Vec<Line> = Vec::new();
        let mut arcs: Vec<Segment> = Vec::new();
        let mut farcs: Vec<Segment> = Vec::new();
        let mut semi: Vec<Segment> = Vec::new();
        let mut fsemi: Vec<Segment> = Vec::new();

        let (origin, flipped) = calculate_origin_and_orientation(&coordinate);

        // let mask = TileFlags::from_bits_truncate(tile);
        if mask.contains(TileFlags::A1) {
            let arc = if flipped { A1_V } else { A1_A } + origin;
            arcs.push(arc + Radius { r: -W });
            arcs.push(arc + Radius { r: W });
        }

        if mask.contains(TileFlags::A2) {
            let arc = if flipped { A2_V } else { A2_A } + origin;
            arcs.push(arc + Radius { r: -W });
            arcs.push(arc + Radius { r: W });
        }

        if mask.contains(TileFlags::A3) {
            let arc = if flipped { A3_V } else { A3_A } + origin;
            arcs.push(arc + Radius { r: -W });
            arcs.push(arc + Radius { r: W });
        }

        if mask.contains(TileFlags::T1) {
            if flipped {
                if mask.contains(TileFlags::Y1) {
                    fsemi.push(T1_V + origin);
                } else if mask.contains(TileFlags::L3) {
                    farcs.push(T1_V_ARC + origin + SegmentOffset { offset: 1 });
                    farcs.push(T1_V_ARC + origin + SegmentOffset { offset: 2 });
                    arcs.push(T1_V_ARC + origin + SegmentOffset { offset: 3 });
                } else if mask.contains(TileFlags::L2) {
                    farcs.push(T1_V_ARC + origin + SegmentOffset { offset: 3 });
                    farcs.push(T1_V_ARC + origin + SegmentOffset { offset: 2 });
                    arcs.push(T1_V_ARC + origin + SegmentOffset { offset: 1 });
                } else {
                    semi.push(T1_V + origin);
                }
            } else {
                if mask.contains(TileFlags::Y1) {
                    fsemi.push(T1_A + origin);
                } else if mask.contains(TileFlags::L3) {
                    farcs.push(T1_A_ARC + origin + SegmentOffset { offset: 6 });
                    farcs.push(T1_A_ARC + origin + SegmentOffset { offset: 5 });
                    arcs.push(T1_A_ARC + origin + SegmentOffset { offset: 4 });
                } else if mask.contains(TileFlags::L2) {
                    farcs.push(T1_A_ARC + origin + SegmentOffset { offset: 4 });
                    farcs.push(T1_A_ARC + origin + SegmentOffset { offset: 5 });
                    arcs.push(T1_A_ARC + origin + SegmentOffset { offset: 6 });
                } else {
                    semi.push(T1_A + origin);
                }
            }
        }

        if mask.contains(TileFlags::T2) {
            if flipped {
                if mask.contains(TileFlags::Y2) {
                    fsemi.push(T2_V + origin);
                } else if mask.contains(TileFlags::L3) {
                    farcs.push(T2_ARC + origin + SegmentOffset { offset: 5 });
                    farcs.push(T2_ARC + origin + SegmentOffset { offset: 4 });
                    arcs.push(T2_ARC + origin + SegmentOffset { offset: 3 });
                } else if mask.contains(TileFlags::L1) {
                    farcs.push(T2_ARC + origin + SegmentOffset { offset: 3 });
                    farcs.push(T2_ARC + origin + SegmentOffset { offset: 4 });
                    arcs.push(T2_ARC + origin + SegmentOffset { offset: 5 });
                } else {
                    semi.push(T2_V + origin);
                }
            } else {
                if mask.contains(TileFlags::Y2) {
                    fsemi.push(T2_A + origin);
                } else if mask.contains(TileFlags::L3) {
                    farcs.push(T2_ARC + origin + SegmentOffset { offset: 3 });
                    farcs.push(T2_ARC + origin + SegmentOffset { offset: 2 });
                    arcs.push(T2_ARC + origin + SegmentOffset { offset: 4 });
                } else if mask.contains(TileFlags::L1) {
                    farcs.push(T2_ARC + origin + SegmentOffset { offset: 3 });
                    farcs.push(T2_ARC + origin + SegmentOffset { offset: 4 });
                    arcs.push(T2_ARC + origin + SegmentOffset { offset: 2 });
                } else {
                    semi.push(T2_A + origin);
                }
            }
        }

        if mask.contains(TileFlags::T3) {
            if flipped {
                if mask.contains(TileFlags::Y3) {
                    fsemi.push(T3_V + origin);
                } else if mask.contains(TileFlags::L2) {
                    farcs.push(T3_ARC + origin + SegmentOffset { offset: 6 });
                    farcs.push(T3_ARC + origin + SegmentOffset { offset: 5 });
                    arcs.push(T3_ARC + origin + SegmentOffset { offset: 1 });
                } else if mask.contains(TileFlags::L1) {
                    farcs.push(T3_ARC + origin + SegmentOffset { offset: 1 });
                    farcs.push(T3_ARC + origin + SegmentOffset { offset: 6 });
                    arcs.push(T3_ARC + origin + SegmentOffset { offset: 5 });
                } else {
                    semi.push(T3_V + origin);
                }
            } else {
                if mask.contains(TileFlags::Y3) {
                    fsemi.push(T3_A + origin);
                } else if mask.contains(TileFlags::L2) {
                    farcs.push(T3_ARC + origin + SegmentOffset { offset: 1 });
                    farcs.push(T3_ARC + origin + SegmentOffset { offset: 2 });
                    arcs.push(T3_ARC + origin + SegmentOffset { offset: 6 });
                } else if mask.contains(TileFlags::L1) {
                    farcs.push(T3_ARC + origin + SegmentOffset { offset: 1 });
                    farcs.push(T3_ARC + origin + SegmentOffset { offset: 6 });
                    arcs.push(T3_ARC + origin + SegmentOffset { offset: 2 });
                } else {
                    semi.push(T3_A + origin);
                }
            }
        }

        if mask.contains(TileFlags::N1) {
            if flipped {
                semi.push(N1_V + origin);
            } else {
                semi.push(N1_A + origin);
            }
        }
        if mask.contains(TileFlags::N2) {
            if flipped {
                semi.push(N2_V + origin);
            } else {
                semi.push(N2_A + origin);
            }
        }

        if mask.contains(TileFlags::N3) {
            if flipped {
                semi.push(N3_V + origin);
            } else {
                semi.push(N3_A + origin);
            }
        }

        if mask.contains(TileFlags::L1) {
            lines.push(L49 + origin);
            lines.push(L52 + origin);
        }

        if mask.contains(TileFlags::L2) {
            if flipped {
                lines.push(L51 + origin);
                lines.push(L54 + origin);
            } else {
                lines.push(L60 + origin);
                lines.push(L57 + origin);
            }
        }

        if mask.contains(TileFlags::L3) {
            if flipped {
                lines.push(L50 + origin);
                lines.push(L53 + origin);
            } else {
                lines.push(L59 + origin);
                lines.push(L56 + origin);
            }
        }

        if mask.contains(TileFlags::Y1) {
            if flipped {
                lines.push(L74 + origin);
                lines.push(L66 + origin);
                lines.push(L02 + origin);

                faint.push(L92 + origin);
                faint.push(L103 + origin);
            } else {
                lines.push(L80 + origin);
                lines.push(L26 + origin);
                lines.push(L72 + origin);
                //
                faint.push(L102 + origin);
                faint.push(L116 + origin);
            }
        }

        if mask.contains(TileFlags::Y2) {
            if flipped {
                lines.push(L62 + origin);
                lines.push(L77 + origin);
                lines.push(L12 + origin);

                faint.push(L106 + origin);
                faint.push(L96 + origin);
            } else {
                lines.push(L67 + origin);
                lines.push(L83 + origin);
                lines.push(L36 + origin);
                faint.push(L120 + origin);
                faint.push(L98 + origin);
            }
        }
        if mask.contains(TileFlags::Y3) {
            if flipped {
                lines.push(L76 + origin);
                lines.push(L63 + origin);
                lines.push(L08 + origin);
                faint.push(L123 + origin);
                faint.push(L110 + origin);
            } else {
                lines.push(L32 + origin);
                lines.push(L81 + origin);
                lines.push(L69 + origin);

                faint.push(L117 + origin);
                faint.push(L99 + origin);
            }
        }

        if mask.contains(TileFlags::C1) {
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
        if mask.contains(TileFlags::C2) {
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
        if mask.contains(TileFlags::C3) {
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

        if mask.contains(TileFlags::X1) {
            arcs.push(if flipped { A1_V } else { A1_A } + origin + Radius { r: -W });
            arcs.push(if flipped { A2_V } else { A2_A } + origin + Radius { r: -W });
            arcs.push(if flipped { A3_V } else { A3_A } + origin + Radius { r: -W });
        }
        if mask.contains(TileFlags::X2) {
            // TOO many lines here, some we get for free
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

        if mask.contains(TileFlags::W1) {
            create_triangle(flipped).map(|line| lines.push(line + origin));
        }

        return Tile {
            lines,
            faint,
            arcs,
            semi,
            fsemi,
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
        for fsemi in self.fsemi.iter() {
            faint_path.semi(fsemi);
        }
    }
}

pub fn create_triangle(flipped: bool) -> [Line; 3] {
    return if flipped { TRIANGLE_V } else { TRIANGLE_A };
}

pub fn calculate_origin_and_orientation(coordinate: &Coordinate) -> (Point, bool) {
    let x_margin = 0.0;
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

        let path1: Tile = Tile::new(Coordinate { i: 0, j: 0 }, TileFlags::Y2 | TileFlags::T2);
        let path2: Tile = Tile::new(Coordinate { i: 1, j: 0 }, TileFlags::Y2 | TileFlags::T2);

        row_path.push(path1);
        row_path.push(path2);

        let wall1: Tile = Tile::new(Coordinate { i: 0, j: 0 }, TileFlags::W1);
        let wall2: Tile = Tile::new(Coordinate { i: 1, j: 0 }, TileFlags::W1);
        //
        row_wall.push(wall1);
        row_wall.push(wall2);

        self.tiles.push(row_path);
        self.walls.push(row_wall);

        self.width = 2.0 * (M + 50.0);
        self.height = H + 0.0;
    }

    pub fn display_all_tiles(&mut self) {
        let mut tile_index = 0;

        for j in 0..63 {
            let mut row_path: Vec<Tile> = Vec::new();
            let mut row_wall: Vec<Tile> = Vec::new();

            let possible_tiles = match_side_1_2_3(j);

            let mut i = 0;
            //println!("");
            for tile in possible_tiles.iter() {
                //println!("{j} {i} {:?}", TileFlags::from_bits_truncate(*tile));

                let coord1: Coordinate = Coordinate { i: i * 2, j: j * 2 };
                let coord2: Coordinate = Coordinate {
                    i: i * 2 + 1,
                    j: j * 2,
                };

                let path1: Tile = Tile::new(coord1, TileFlags::from_bits_truncate(*tile));
                let wall1: Tile = Tile::new(coord1, TileFlags::W1);

                let path2: Tile = Tile::new(coord2, TileFlags::from_bits_truncate(*tile));
                let wall2: Tile = Tile::new(coord2, TileFlags::W1);

                row_path.push(path1);
                row_wall.push(wall1);
                row_path.push(path2);
                row_wall.push(wall2);
                i += 1;
            }
            self.tiles.push(row_path);
            self.walls.push(row_wall);
        }

        self.width = (10 * 2) as f64 * (M + 50.0);
        self.height = (63 * 2) as f64 * (H + 00.0);
    }

    pub fn display_random_grid(&mut self) {
        let mut tile_index = 0;

        let m = 9;
        let n = 6;
        for j in 0..n {
            let mut row_path: Vec<Tile> = Vec::new();
            let mut row_wall: Vec<Tile> = Vec::new();

            //println!("");
            for i in 0..m {
                //println!("{j} {i} {:?}", TileFlags::from_bits_truncate(*tile));

                let coord: Coordinate = Coordinate { i, j };

                let mut tile: Option<&u32> = Some(&TileFlags::W1.bits());
                if i == 0 && j == 0 {
                    let possible_tiles = match_side_1(0);
                    tile = possible_tiles.choose(&mut rand::thread_rng());
                    if tile.is_some() {
                        let path: Tile = Tile::new(
                            coord,
                            TileFlags::from_bits_truncate(*tile.unwrap_or(&TileFlags::W1.bits())),
                        );
                        row_path.push(path);
                    }
                }

                let wall: Tile = Tile::new(coord, TileFlags::W1);

                row_wall.push(wall);
            }
            self.tiles.push(row_path);
            self.walls.push(row_wall);
        }

        self.width = (m + 1) as f64 * M;
        self.height = (n) as f64 * H;
    }

    // Path commands
    // https://github.com/bodoni/svg/blob/master/src/node/element/path/data.rs
    pub fn render(self, file_name: String) -> Result<(), Box<dyn Error>> {
        let mut svg = String::new();

        let margin = 5.0;
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

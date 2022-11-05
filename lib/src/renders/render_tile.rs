use crate::renders::constants::{
    TileFlags, L01, L02, L07, L08, L102, L103, L106, L11, L110, L116, L117, L12, L120, L123, L13,
    L14, L15, L16, L17, L18, L19, L20, L21, L22, L23, L24, L25, L26, L31, L32, L35, L36, L37, L38,
    L39, L40, L41, L42, L43, L44, L45, L46, L47, L48, L49, L50, L51, L52, L53, L54, L56, L57, L59,
    L60, L62, L63, L66, L67, L69, L72, L74, L76, L77, L80, L81, L83, L92, L96, L98, L99, M, U, W,
};
use crate::renders::primatives::{Coordinate, Line, Point, Radius, Segment, SegmentOffset};
use crate::renders::shapes::{
    A1_A, A1_V, A2_A, A2_V, A3_A, A3_V, N1_A, N1_V, N2_A, N2_V, N3_A, N3_V, T1_A, T1_A_ARC, T1_V,
    T1_V_ARC, T2_A, T2_ARC, T2_V, T3_A, T3_ARC, T3_V, TRIANGLE_A, TRIANGLE_V,
};
use crate::renders::svg_path::Path;

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

pub struct Tile {
    lines: Vec<Line>,
    faint: Vec<Line>,
    arcs: Vec<Segment>,
    farcs: Vec<Segment>,
    semi: Vec<Segment>,
    fsemi: Vec<Segment>,
}

pub fn rotate(coordinate: Coordinate, raw_mask: TileFlags)-> TileFlags{
    // Rotate tile, depending on position so it renders
    let mut mask = TileFlags::Z1;
    if raw_mask.contains(TileFlags::W1) {
        mask |= TileFlags::W1;
    }
    if raw_mask.contains(TileFlags::X1) {
        mask |= TileFlags::X1;
    }
    if raw_mask.contains(TileFlags::X2) {
        mask |= TileFlags::X2;
    }

    let rotate_index = coordinate.i % 3;
    match rotate_index {
        1 => {
            if raw_mask.contains(TileFlags::Y1) {
                mask |= TileFlags::Y2;
            }
            if raw_mask.contains(TileFlags::Y2) {
                mask |= TileFlags::Y3;
            }
            if raw_mask.contains(TileFlags::Y3) {
                mask |= TileFlags::Y1;
            }
            if raw_mask.contains(TileFlags::A1) {
                mask |= TileFlags::A2;
            }
            if raw_mask.contains(TileFlags::A2) {
                mask |= TileFlags::A3;
            }
            if raw_mask.contains(TileFlags::A3) {
                mask |= TileFlags::A1;
            }
            if raw_mask.contains(TileFlags::C1) {
                mask |= TileFlags::C2;
            }
            if raw_mask.contains(TileFlags::C2) {
                mask |= TileFlags::C3;
            }
            if raw_mask.contains(TileFlags::C3) {
                mask |= TileFlags::C1;
            }
            if raw_mask.contains(TileFlags::L1) {
                mask |= TileFlags::L2;
            }
            if raw_mask.contains(TileFlags::L2) {
                mask |= TileFlags::L3;
            }
            if raw_mask.contains(TileFlags::L3) {
                mask |= TileFlags::L1;
            }
            if raw_mask.contains(TileFlags::N1) {
                mask |= TileFlags::N2;
            }
            if raw_mask.contains(TileFlags::N2) {
                mask |= TileFlags::N3;
            }
            if raw_mask.contains(TileFlags::N3) {
                mask |= TileFlags::N1;
            }
            if raw_mask.contains(TileFlags::T1) {
                mask |= TileFlags::T2;
            }
            if raw_mask.contains(TileFlags::T2) {
                mask |= TileFlags::T3;
            }
            if raw_mask.contains(TileFlags::T3) {
                mask |= TileFlags::T1;
            }
        }
        2 => {
            if raw_mask.contains(TileFlags::Y1) {
                mask |= TileFlags::Y3;
            }
            if raw_mask.contains(TileFlags::Y2) {
                mask |= TileFlags::Y1;
            }
            if raw_mask.contains(TileFlags::Y3) {
                mask |= TileFlags::Y2;
            }
            if raw_mask.contains(TileFlags::A1) {
                mask |= TileFlags::A3;
            }
            if raw_mask.contains(TileFlags::A2) {
                mask |= TileFlags::A1;
            }
            if raw_mask.contains(TileFlags::A3) {
                mask |= TileFlags::A2;
            }
            if raw_mask.contains(TileFlags::C1) {
                mask |= TileFlags::C3;
            }
            if raw_mask.contains(TileFlags::C2) {
                mask |= TileFlags::C1;
            }
            if raw_mask.contains(TileFlags::C3) {
                mask |= TileFlags::C2;
            }
            if raw_mask.contains(TileFlags::L1) {
                mask |= TileFlags::L3;
            }
            if raw_mask.contains(TileFlags::L2) {
                mask |= TileFlags::L1;
            }
            if raw_mask.contains(TileFlags::L3) {
                mask |= TileFlags::L2;
            }
            if raw_mask.contains(TileFlags::N1) {
                mask |= TileFlags::N3;
            }
            if raw_mask.contains(TileFlags::N2) {
                mask |= TileFlags::N1;
            }
            if raw_mask.contains(TileFlags::N3) {
                mask |= TileFlags::N2;
            }
            if raw_mask.contains(TileFlags::T1) {
                mask |= TileFlags::T3;
            }
            if raw_mask.contains(TileFlags::T2) {
                mask |= TileFlags::T1;
            }
            if raw_mask.contains(TileFlags::T3) {
                mask |= TileFlags::T2;
            }
        }
        _ => {
            mask = raw_mask;
        }
    }
    return mask
}

impl Tile {


    // Change to a list of TileTypes, Could be a fixed length, since it can't be londer than 0
    // Just fill the remaining with X3
    pub fn new(coordinate: Coordinate, raw_mask: TileFlags) -> Self {
        let mut lines: Vec<Line> = Vec::new();
        let mut faint: Vec<Line> = Vec::new();
        let mut arcs: Vec<Segment> = Vec::new();
        let mut farcs: Vec<Segment> = Vec::new();
        let mut semi: Vec<Segment> = Vec::new();
        let mut fsemi: Vec<Segment> = Vec::new();

        let (origin, flipped) = calculate_origin_and_orientation(&coordinate);

        // Title can be rotated
        let mask = rotate(coordinate, raw_mask) ;

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

        if !mask.contains(TileFlags::X2) {
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

        // TODO: For these and others, randomally change which line goes on top
        // TODO: Add in some other variations too
        // TODO: Use the upper 2 bits to specify the variation!
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

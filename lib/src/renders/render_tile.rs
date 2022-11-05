use crate::renders::constants::{
    L001, L002, L005, L006, L007, L008, L011, L012, L013, L014, L015, L016, L017, L018, L019, L020, L021, L022, 
    L023, L024, L025, L026, L035, L036, L037, L038, L039, L040, L041, L042, L043, L044, L045, L046, L047, L048, 
    L049, L050, L051, L052, L053, L054, L056, L057, L059, L060, L062, L063, L065, L066, L067, L069, L072, L073, 
    L074, L076, L077, L080, L081, L083, L085, L086, L087,L092, L096, L098, L099, L102, L103, L104, L106, 
    M, TileFlags, U, W, 
};
use crate::renders::primatives::{Coordinate, Line, Point, Radius, Segment, SegmentOffset};
use crate::renders::shapes::{
    A1_A, A1_V, A2_A, A2_V, A3_A, A3_V, N1_A, N1_V, N2_A, N2_V, N3_A, N3_V, T1_A, T1_A_ARC, T1_V,
    T1_V_ARC, T2_A, T2_ARC, T2_V, T3_A, T3_ARC, T3_V, TRIANGLE_A, TRIANGLE_V,
};
use crate::renders::svg_path::Path;
use rand::Rng;

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

        let mut rng = rand::thread_rng();

        let y_random = rng.gen_range(0..2); // 0,1

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
                lines.push(L049 + origin);
                lines.push(L052 + origin);
            }
    
            if mask.contains(TileFlags::L2) {
                if flipped {
                    lines.push(L051 + origin);
                    lines.push(L054 + origin);
                } else {
                    lines.push(L060 + origin);
                    lines.push(L057 + origin);
                }
            }
    
            if mask.contains(TileFlags::L3) {
                if flipped {
                    lines.push(L050 + origin);
                    lines.push(L053 + origin);
                } else {
                    lines.push(L059 + origin);
                    lines.push(L056 + origin);
                }
            }
        }

        if mask.contains(TileFlags::Y1) {
            if flipped {
                lines.push(L074 + origin);
                lines.push(L066 + origin);
                lines.push(L002 + origin);

                faint.push(L092 + origin);
                faint.push(L103 + origin);
            } else {
                lines.push(L080 + origin);
                lines.push(L026 + origin);
                lines.push(L072 + origin);
                //
                faint.push(L102 + origin);
                faint.push(L087 + origin);
            }
        }

        if mask.contains(TileFlags::Y2) {
            if flipped {
                lines.push(L062 + origin);
                lines.push(L077 + origin);
                lines.push(L012 + origin);

                faint.push(L106 + origin);
                faint.push(L096 + origin);
            } else {
                lines.push(L067 + origin);
                lines.push(L083 + origin);
                lines.push(L036 + origin);
                faint.push(L106 + origin);
                faint.push(L098 + origin);
            }
        }
        if mask.contains(TileFlags::Y3) {
            if flipped {
                lines.push(L076 + origin);
                lines.push(L063 + origin);
                lines.push(L008 + origin);
                faint.push(L104 + origin);
                faint.push(L092 + origin);
            } else {
                lines.push(L006 + origin);
                lines.push(L081 + origin);
                lines.push(L069 + origin);
                faint.push(L085 + origin);
                faint.push(L099 + origin);
            }
        }

        // TODO: For these and others, randomally change which line goes on top
        // TODO: Add in some other variations too
        // TODO: Use the upper 2 bits to specify the variation!
        if mask.contains(TileFlags::C1) {
            if flipped {
                lines.push(L007 + origin);
                lines.push(L008 + origin);
                lines.push(L018 + origin);
                lines.push(L017 + origin);
                faint.push(L021 + origin);
                faint.push(L022 + origin);
            } else {
                lines.push(L005 + origin);
                lines.push(L006 + origin);
                lines.push(L042 + origin);
                lines.push(L041 + origin);
                faint.push(L045 + origin);
                faint.push(L046 + origin);
            }
        }
        if mask.contains(TileFlags::C2) {
            if flipped {
                lines.push(L001 + origin);
                lines.push(L002 + origin);
                lines.push(L015 + origin);
                lines.push(L016 + origin);
                faint.push(L019 + origin);
                faint.push(L020 + origin);
            } else {
                lines.push(L025 + origin);
                lines.push(L026 + origin);
                lines.push(L040 + origin);
                lines.push(L039 + origin);
                faint.push(L043 + origin);
                faint.push(L044 + origin);
            }
        }
        if mask.contains(TileFlags::C3) {
            if flipped {
                lines.push(L011 + origin);
                lines.push(L012 + origin);
                lines.push(L013 + origin);
                lines.push(L014 + origin);
                faint.push(L023 + origin);
                faint.push(L024 + origin);
            } else {
                lines.push(L035 + origin);
                lines.push(L036 + origin);
                lines.push(L037 + origin);
                lines.push(L038 + origin);
                faint.push(L047 + origin);
                faint.push(L048 + origin);
            }
        }

        if mask.contains(TileFlags::X1) {
            arcs.push(if flipped { A1_V } else { A1_A } + origin + Radius { r: -W });
            arcs.push(if flipped { A2_V } else { A2_A } + origin + Radius { r: -W });
            arcs.push(if flipped { A3_V } else { A3_A } + origin + Radius { r: -W });
        }
        if mask.contains(TileFlags::X2) {
       
            if flipped {
                let x = rng.gen_range(0..2); // 0,1
                match x {
                    0u32 => {
                        for line in [L050,L053, L074,L066,L002, L008, L007] {
                            lines.push(line+origin);
                        }
                        for line in [L103,L092] {
                            faint.push(line+origin);
                        }
                    }
                    _ => {
                        for line in [L051,L054, L065,L011,L073, L007,L008] {
                            lines.push(line+origin);
                        }
                        for line in [L104, L096] {
                            faint.push(line+origin);
                        }
                    }
                };
               
          
            } else {
                for line in [L059,L056, L080,L026,L072, L005,L006] {
                    lines.push(line+origin);
                }
                for line in [L102,L086] {
                    faint.push(line+origin);
                }
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

use crate::renders::constants::{
    L001, L002, L003, L005, L006, L007, L008, L011, L012, L013, L014, L015, L016, L017, L018, L019, L020, L021, L022, 
    L023, L024, L025, L026, L035, L036, L037, L038, L039, L040, L041, L042, L043, L044, L045, L046, L047, L048, 
    L049, L050, L051, L052, L053, L054, L056, L057, L059, L060, L061, L062, L063, L064, L065, L066, L067, L068,
    L069, L070, L071, L072, L073, 
    L074, L075, L076, L077,L078,L079, L080, L081, L082, L083, L084, L085, L086, L087, L088, L089, 
    L091, L092, L093, L094, L095, L096,
    L097, L098, L099, 
    L100, L101, L102, L103, L104, L105, L106, L107,
    L121,
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

#[inline]
fn push2(lines: &mut Vec<Line>, origin:Point, line1: Line, line2: Line){
    lines.push(line1 + origin);
    lines.push(line2 + origin);
}

#[inline]
fn push3(lines: &mut Vec<Line>, origin:Point, line1: Line, line2: Line, line3: Line){
    lines.push(line1 + origin);
    lines.push(line2 + origin);
    lines.push(line3 + origin);
}

#[inline]
fn push4(lines: &mut Vec<Line>, origin:Point, line1: Line, line2: Line, line3: Line, line4: Line){
    lines.push(line1 + origin);
    lines.push(line2 + origin);
    lines.push(line3 + origin);
    lines.push(line4 + origin);
}

#[inline]
fn paint_l1( lines: &mut Vec<Line>, origin:Point)  {
    push2(lines, origin, L049, L052);
 }

 #[inline]
fn paint_l2(lines: &mut Vec<Line>, origin:Point, flipped: bool)  {
    if flipped {
        push2(lines, origin, L051, L054);
    } else {
        push2(lines, origin, L057, L060);
    }
}

#[inline]
fn paint_l3( lines: &mut Vec<Line>, origin:Point, flipped: bool)  {
    if flipped {
        push2(lines, origin, L050, L053);
    } else {
        push2(lines, origin, L056, L059);
    }
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
            if mask.contains(TileFlags::L1) && !mask.contains(TileFlags::Y2) {
                paint_l1(&mut lines, origin);
            }
            if mask.contains(TileFlags::L2) && !mask.contains(TileFlags::Y3){
                paint_l2(&mut lines, origin, flipped);
            }
            if mask.contains(TileFlags::L3) && !mask.contains(TileFlags::Y1) {
                paint_l3(&mut lines, origin, flipped);
            }
        }

        let zero_or_one = rng.gen_range(0..2); // 0,1

        if mask.contains(TileFlags::Y1) {
            if zero_or_one==0  {
                paint_l2(&mut lines, origin, flipped);
                if flipped {
                    push3(&mut lines, origin, L011, L065,L073);
                    push2(&mut faint, origin, L091, L104);                    
                } else {
                    push3(&mut lines, origin, L035, L071,L079);
                    push2(&mut faint, origin, L088, L101);                    
                }
            } else {
                paint_l3(&mut lines, origin, flipped);
                if flipped {
                    push3(&mut lines, origin, L002, L066,L074);
                    push2(&mut faint, origin, L092, L103);                    
                } else {
                    push3(&mut lines, origin, L026, L072,L080);
                    push2(&mut faint, origin, L087, L102);                    
                }
            }
        }

        if mask.contains(TileFlags::Y2) {
            if zero_or_one==0 {
                paint_l3(&mut lines, origin, flipped);
                if flipped {
                    push3(&mut lines, origin, L007, L061,L078);
                    push2(&mut faint, origin, L095, L105);
                } else {
                    push3(&mut lines, origin, L005, L068,L084);
                    push2(&mut faint, origin, L078, L097);
                }
            } else {
                paint_l1(&mut lines, origin);
                if flipped {
                    push3(&mut lines, origin, L012, L062,L077);
                    push2(&mut faint, origin, L096, L106);
                } else {
                    push3(&mut lines, origin, L036, L067,L083);
                    push2(&mut faint, origin, L089, L098);
                }
            }
            
        }
        if mask.contains(TileFlags::Y3) {

            if zero_or_one==0 {
                paint_l1(&mut lines, origin);
                if flipped {
                    push3(&mut lines, origin, L001, L075,L064);
                    push2(&mut faint, origin, L107, L094);
                } else {
                    push3(&mut lines, origin, L025, L070,L082);
                    push2(&mut faint, origin, L085, L100);
                }
            }else{
                paint_l2(&mut lines, origin, flipped);
                if flipped {
                    push3(&mut lines, origin, L008, L063,L076);
                    push2(&mut faint, origin, L093, L121);
                } else {
                    push3(&mut lines, origin, L006, L069,L081);
                    push2(&mut faint, origin, L086, L099);
                }
            }
           
        }

        // TODO: For these and others, randomally change which line goes on top
        // TODO: Add in some other variations too
        // TODO: Use the upper 2 bits to specify the variation!
        if mask.contains(TileFlags::C1) {
            if flipped {
                push4(&mut lines, origin, L007, L008, L017, L018);
                push2(&mut faint, origin, L021, L022);
            } else {
                push4(&mut lines, origin, L005, L006, L042, L041);
                push2(&mut faint, origin, L045, L046);
            }
        }
        if mask.contains(TileFlags::C2) {
            if flipped {
                push4(&mut lines, origin, L001, L002, L015, L016);
                push2(&mut faint, origin, L019, L020);
            } else {
                push4(&mut lines, origin, L025, L026, L040, L039);
                push2(&mut faint, origin, L043, L044);
            }
        }
        if mask.contains(TileFlags::C3) {
            if flipped {
                push4(&mut lines, origin, L011, L012, L013, L014);
                push2(&mut faint, origin, L023, L024);
            } else {
                push4(&mut lines, origin, L035, L036, L037, L038);
                push2(&mut faint, origin, L047, L048);
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
                //let x = 2;
                match x {
                    0 => {
                        paint_l3(&mut lines, origin, flipped);
                        for line in [L074,L066,L002,L008,L007] {
                            lines.push(line+origin);
                        }
                        push2(&mut faint, origin, L103, L092);
                    }
                    1 => {
                        paint_l2(&mut lines, origin, flipped);
                        for line in [L065,L011,L073,L007,L008] {
                            lines.push(line+origin);
                        }
                        push2(&mut faint, origin, L091, L104);
                    }
                    _ => {
                        paint_l1(&mut lines, origin);
                        for line in [L012,L062,L077, L002,L001]{
                            lines.push(line+origin);
                        }
                        push2(&mut faint, origin, L096, L106);
                    }
                };
          
            } else {
                paint_l3(&mut lines, origin, flipped);
                for line in [L080,L026,L072, L005,L006] {
                    lines.push(line+origin);
                }
                push2(&mut faint, origin, L102, L087);
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

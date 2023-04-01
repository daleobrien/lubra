use crate::renders::primatives::{Coordinate, Line, Point, Radius, Segment, SegmentOffset};
use crate::renders::shapes::{
    A1_A, A1_V, A2_A, A2_V, A3_A, A3_V, N1_A, N1_V, N2_A, N2_V, N3_A, N3_V, T1_A, T1_A_ARC, T1_V,
    T1_V_ARC, T2_A, T2_ARC, T2_V, T3_A, T3_ARC, T3_V, TRIANGLE_A, TRIANGLE_V,
};
use crate::renders::strokes::{
    L001, L002, L005, L006, L007, L008, L011, L012, L013, L014, L015, L016, L017, L018, L019,
    L020, L021, L022, L023, L024, L025, L026, L035, L036, L037, L038, L039, L040, L041, L042, L043,
    L044, L045, L046, L047, L048, L049, L050, L051, L052, L053, L054, L056, L057, L059, L060, L061,
    L062, L063, L064, L065, L066, L067, L068, L069, L070, L071, L072, L073, L074, L075, L076, L077,
    L078, L079, L080, L081, L082, L083, L084, L085, L086, L087, L088, L089, L091, L092, L093, L094,
    L095, L096, L097, L098, L099, L100, L101, L102, L103, L104, L105, L106, L107, L121, M, U, W,
};
use crate::renders::svg::Path;
use crate::maze::tile::Tile;

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

pub fn rotate(coordinate: Coordinate, tile: Tile) -> Tile {
    // Rotate tile, depending on position so it renders
    return match coordinate.i % 3 {
        1 => tile.rotate_cw(),
        2 => tile.rotate_ccw(),
        _ => tile,
    };
}

#[inline]
fn push2(lines: &mut Vec<Line>, origin: Point, line1: Line, line2: Line) {
    lines.push(line1 + origin);
    lines.push(line2 + origin);
}

#[inline]
fn push3(lines: &mut Vec<Line>, origin: Point, line1: Line, line2: Line, line3: Line) {
    lines.push(line1 + origin);
    lines.push(line2 + origin);
    lines.push(line3 + origin);
}

#[inline]
fn push4(lines: &mut Vec<Line>, origin: Point, line1: Line, line2: Line, line3: Line, line4: Line) {
    lines.push(line1 + origin);
    lines.push(line2 + origin);
    lines.push(line3 + origin);
    lines.push(line4 + origin);
}

#[inline]
fn paint_l1(lines: &mut Vec<Line>, origin: Point) {
    push2(lines, origin, L049, L052);
}

#[inline]
fn paint_l2(lines: &mut Vec<Line>, origin: Point, flipped: bool) {
    if flipped {
        push2(lines, origin, L051, L054);
    } else {
        push2(lines, origin, L057, L060);
    }
}

#[inline]
fn paint_l3(lines: &mut Vec<Line>, origin: Point, flipped: bool) {
    if flipped {
        push2(lines, origin, L050, L053);
    } else {
        push2(lines, origin, L056, L059);
    }
}

#[inline]
fn paint_y1(lines: &mut Vec<Line>, faint: &mut Vec<Line>, origin: Point, flipped: bool) {
    let mut rng = rand::thread_rng();
    if rng.gen_range(0..2) == 0 {
        paint_l2(lines, origin, flipped);
        if flipped {
            push3(lines, origin, L011, L065, L073);
            push2(faint, origin, L091, L104);
        } else {
            push3(lines, origin, L035, L071, L079);
            push2(faint, origin, L088, L101);
        }
    } else {
        paint_l3(lines, origin, flipped);
        if flipped {
            push3(lines, origin, L002, L066, L074);
            push2(faint, origin, L092, L103);
        } else {
            push3(lines, origin, L026, L072, L080);
            push2(faint, origin, L087, L102);
        }
    }
}

#[inline]
fn paint_y2(lines: &mut Vec<Line>, faint: &mut Vec<Line>, origin: Point, flipped: bool) {
    let mut rng = rand::thread_rng();
    if rng.gen_range(0..2) == 0 {
        paint_l3(lines, origin, flipped);
        if flipped {
            push3(lines, origin, L007, L061, L078);
            push2(faint, origin, L095, L105);
        } else {
            push3(lines, origin, L005, L068, L084);
            push2(faint, origin, L078, L097);
        }
    } else {
        paint_l1(lines, origin);
        if flipped {
            push3(lines, origin, L012, L062, L077);
            push2(faint, origin, L096, L106);
        } else {
            push3(lines, origin, L036, L067, L083);
            push2(faint, origin, L089, L098);
        }
    }
}

#[inline]
fn paint_y3(lines: &mut Vec<Line>, faint: &mut Vec<Line>, origin: Point, flipped: bool) {
    let mut rng = rand::thread_rng();
    if rng.gen_range(0..2) == 0 {
        paint_l1(lines, origin);
        if flipped {
            push3(lines, origin, L001, L064, L075);
            push2(faint, origin, L094, L107);
        } else {
            push3(lines, origin, L025, L070, L082);
            push2(faint, origin, L085, L100);
        }
    } else {
        paint_l2(lines, origin, flipped);
        if flipped {
            push3(lines, origin, L008, L063, L076);
            push2(faint, origin, L093, L121);
        } else {
            push3(lines, origin, L006, L069, L081);
            push2(faint, origin, L086, L099);
        }
    }
}
pub struct Glyph {
    lines: Vec<Line>,
    faint: Vec<Line>,
    arcs: Vec<Segment>,
    farcs: Vec<Segment>,
    semi: Vec<Segment>,
    fsemi: Vec<Segment>,
    pub nets: [u32; 6],
}

// ----

static NODE_MASKS: [Tile; 8] = [
    Tile::X1,
    Tile::X2,
    Tile::A1,
    Tile::A2,
    Tile::A3,
    Tile::L1,
    Tile::L2,
    Tile::L3,
];

fn get_nets(flag: Tile) -> Vec<Vec<usize>> {
    let mut nodes = Vec::new();
    match flag {
        Tile::X1 => nodes.push(vec![4, 5, 6]), // All edges connect to each other
        Tile::X2 => {
            nodes.push(vec![1, 6]); // Each edge and it's oposite point connect to each other
            nodes.push(vec![2, 5]);
            nodes.push(vec![3, 4]);
        }
        Tile::A1 => nodes.push(vec![4, 5]),
        Tile::A2 => nodes.push(vec![4, 6]),
        Tile::A3 => nodes.push(vec![5, 6]),
        Tile::L1 => nodes.push(vec![1, 6]),
        Tile::L2 => nodes.push(vec![2, 5]),
        Tile::L3 => nodes.push(vec![3, 4]),
        Tile::N1 => nodes.push(vec![1]),
        Tile::N2 => nodes.push(vec![2]),
        Tile::N3 => nodes.push(vec![3]),
        Tile::T1 => nodes.push(vec![6]),
        Tile::T2 => nodes.push(vec![5]),
        Tile::T3 => nodes.push(vec![4]),
        _ => {}
    }
    return nodes;
}

impl Glyph {
    // Change to a list of TileTypes, Could be a fixed length, since it can't be londer than 0
    // Just fill the remaining with X3
    pub fn new(coordinate: Coordinate, raw_mask: Tile, last_free_net: u32) -> Self {
        let mut lines: Vec<Line> = Vec::new();
        let mut faint: Vec<Line> = Vec::new();
        let mut arcs: Vec<Segment> = Vec::new();
        let mut farcs: Vec<Segment> = Vec::new();
        let mut semi: Vec<Segment> = Vec::new();
        let mut fsemi: Vec<Segment> = Vec::new();
        let mut nets: [u32; 6] = [0; 6];

        let (origin, flipped) = calculate_origin_and_orientation(&coordinate);

        let mut rng = rand::thread_rng();

        // Title can be rotated
        let mask = rotate(coordinate, raw_mask);

        // Add in the Net ids
        let mut last_free_net_max = last_free_net;

        for tile_primative in NODE_MASKS.iter() {
            for node_groups in get_nets(*tile_primative & raw_mask) {
                last_free_net_max += 1;
                for index in node_groups.iter() {
                    nets[*index - 1] = last_free_net_max;
                }
            }
        }
        //println!("{:?} nets -> {:?}", coordinate, nets);

        //
        if mask.contains(Tile::A1) && !mask.contains(Tile::C1) {
            let arc = if flipped { A1_V } else { A1_A } + origin;
            arcs.push(arc + Radius { r: -W });
            arcs.push(arc + Radius { r: W });
        }

        if mask.contains(Tile::A2) && !mask.contains(Tile::C2) {
            let arc = if flipped { A2_V } else { A2_A } + origin;
            arcs.push(arc + Radius { r: -W });
            arcs.push(arc + Radius { r: W });
        }

        if mask.contains(Tile::A3) && !mask.contains(Tile::C3) {
            let arc = if flipped { A3_V } else { A3_A } + origin;
            arcs.push(arc + Radius { r: -W });
            arcs.push(arc + Radius { r: W });
        }

        if mask.contains(Tile::T1) {
            if flipped {
                if mask.contains(Tile::Y1) {
                    fsemi.push(T1_V + origin);
                } else if mask.contains(Tile::L3) {
                    farcs.push(T1_V_ARC + origin + SegmentOffset { offset: 1 });
                    farcs.push(T1_V_ARC + origin + SegmentOffset { offset: 2 });
                    arcs.push(T1_V_ARC + origin + SegmentOffset { offset: 3 });
                } else if mask.contains(Tile::L2) {
                    farcs.push(T1_V_ARC + origin + SegmentOffset { offset: 3 });
                    farcs.push(T1_V_ARC + origin + SegmentOffset { offset: 2 });
                    arcs.push(T1_V_ARC + origin + SegmentOffset { offset: 1 });
                } else {
                    semi.push(T1_V + origin);
                }
            } else {
                if mask.contains(Tile::Y1) {
                    fsemi.push(T1_A + origin);
                } else if mask.contains(Tile::L3) {
                    farcs.push(T1_A_ARC + origin + SegmentOffset { offset: 6 });
                    farcs.push(T1_A_ARC + origin + SegmentOffset { offset: 5 });
                    arcs.push(T1_A_ARC + origin + SegmentOffset { offset: 4 });
                } else if mask.contains(Tile::L2) {
                    farcs.push(T1_A_ARC + origin + SegmentOffset { offset: 4 });
                    farcs.push(T1_A_ARC + origin + SegmentOffset { offset: 5 });
                    arcs.push(T1_A_ARC + origin + SegmentOffset { offset: 6 });
                } else {
                    semi.push(T1_A + origin);
                }
            }
        }

        if mask.contains(Tile::T2) {
            if flipped {
                if mask.contains(Tile::Y2) {
                    fsemi.push(T2_V + origin);
                } else if mask.contains(Tile::L3) {
                    farcs.push(T2_ARC + origin + SegmentOffset { offset: 5 });
                    farcs.push(T2_ARC + origin + SegmentOffset { offset: 4 });
                    arcs.push(T2_ARC + origin + SegmentOffset { offset: 3 });
                } else if mask.contains(Tile::L1) {
                    farcs.push(T2_ARC + origin + SegmentOffset { offset: 3 });
                    farcs.push(T2_ARC + origin + SegmentOffset { offset: 4 });
                    arcs.push(T2_ARC + origin + SegmentOffset { offset: 5 });
                } else {
                    semi.push(T2_V + origin);
                }
            } else {
                if mask.contains(Tile::Y2) {
                    fsemi.push(T2_A + origin);
                } else if mask.contains(Tile::L3) {
                    farcs.push(T2_ARC + origin + SegmentOffset { offset: 3 });
                    farcs.push(T2_ARC + origin + SegmentOffset { offset: 2 });
                    arcs.push(T2_ARC + origin + SegmentOffset { offset: 4 });
                } else if mask.contains(Tile::L1) {
                    farcs.push(T2_ARC + origin + SegmentOffset { offset: 3 });
                    farcs.push(T2_ARC + origin + SegmentOffset { offset: 4 });
                    arcs.push(T2_ARC + origin + SegmentOffset { offset: 2 });
                } else {
                    semi.push(T2_A + origin);
                }
            }
        }

        if mask.contains(Tile::T3) {
            if flipped {
                if mask.contains(Tile::Y3) {
                    fsemi.push(T3_V + origin);
                } else if mask.contains(Tile::L2) {
                    farcs.push(T3_ARC + origin + SegmentOffset { offset: 6 });
                    farcs.push(T3_ARC + origin + SegmentOffset { offset: 5 });
                    arcs.push(T3_ARC + origin + SegmentOffset { offset: 1 });
                } else if mask.contains(Tile::L1) {
                    farcs.push(T3_ARC + origin + SegmentOffset { offset: 1 });
                    farcs.push(T3_ARC + origin + SegmentOffset { offset: 6 });
                    arcs.push(T3_ARC + origin + SegmentOffset { offset: 5 });
                } else {
                    semi.push(T3_V + origin);
                }
            } else {
                if mask.contains(Tile::Y3) {
                    fsemi.push(T3_A + origin);
                } else if mask.contains(Tile::L2) {
                    farcs.push(T3_ARC + origin + SegmentOffset { offset: 1 });
                    farcs.push(T3_ARC + origin + SegmentOffset { offset: 2 });
                    arcs.push(T3_ARC + origin + SegmentOffset { offset: 6 });
                } else if mask.contains(Tile::L1) {
                    farcs.push(T3_ARC + origin + SegmentOffset { offset: 1 });
                    farcs.push(T3_ARC + origin + SegmentOffset { offset: 6 });
                    arcs.push(T3_ARC + origin + SegmentOffset { offset: 2 });
                } else {
                    semi.push(T3_A + origin);
                }
            }
        }

        if mask.contains(Tile::N1) {
            if flipped {
                semi.push(N1_V + origin);
            } else {
                semi.push(N1_A + origin);
            }
        }
        if mask.contains(Tile::N2) {
            if flipped {
                semi.push(N2_V + origin);
            } else {
                semi.push(N2_A + origin);
            }
        }

        if mask.contains(Tile::N3) {
            if flipped {
                semi.push(N3_V + origin);
            } else {
                semi.push(N3_A + origin);
            }
        }

        if !mask.contains(Tile::X2) {
            if mask.contains(Tile::L1) && !mask.contains(Tile::Y2) && !mask.contains(Tile::Y3) {
                paint_l1(&mut lines, origin);
            }
            if mask.contains(Tile::L2) && !mask.contains(Tile::Y1) && !mask.contains(Tile::Y3) {
                paint_l2(&mut lines, origin, flipped);
            }
            if mask.contains(Tile::L3) && !mask.contains(Tile::Y1) && !mask.contains(Tile::Y2) {
                paint_l3(&mut lines, origin, flipped);
            }

            if mask.contains(Tile::Y1) {
                paint_y1(&mut lines, &mut faint, origin, flipped);
            }

            if mask.contains(Tile::Y2) {
                paint_y2(&mut lines, &mut faint, origin, flipped);
            }

            if mask.contains(Tile::Y3) {
                paint_y3(&mut lines, &mut faint, origin, flipped);
            }
        }

        if mask.contains(Tile::C1) {
            let arc = if flipped { A1_V } else { A1_A } + origin;

            if rng.gen_range(0..2) == 0 {
                // Make the A1 go under the L1
                arcs.push(arc + Radius { r: W } + SegmentOffset { offset: 12 });

                farcs.push(arc + Radius { r: -W });
                farcs.push(arc + Radius { r: W } + SegmentOffset { offset: 6 });

                paint_l1(&mut lines, origin);
            } else {
                // Make the A1 go over the L1
                arcs.push(arc + Radius { r: -W });
                arcs.push(arc + Radius { r: W });

                if flipped {
                    push4(&mut lines, origin, L007, L008, L017, L018);
                    push2(&mut faint, origin, L021, L022);
                } else {
                    push4(&mut lines, origin, L005, L006, L042, L041);
                    push2(&mut faint, origin, L045, L046);
                }
            }
        }
        if mask.contains(Tile::C2) {
            let arc = if flipped { A2_V } else { A2_A } + origin;

            if rng.gen_range(0..2) == 0 {
                arcs.push(arc + Radius { r: W } + SegmentOffset { offset: 12 });
                farcs.push(arc + Radius { r: -W });
                farcs.push(arc + Radius { r: W } + SegmentOffset { offset: 6 });
                paint_l2(&mut lines, origin, flipped);
            } else {
                arcs.push(arc + Radius { r: -W });
                arcs.push(arc + Radius { r: W });

                if flipped {
                    push4(&mut lines, origin, L001, L002, L015, L016);
                    push2(&mut faint, origin, L019, L020);
                } else {
                    push4(&mut lines, origin, L025, L026, L040, L039);
                    push2(&mut faint, origin, L043, L044);
                }
            }
        }
        if mask.contains(Tile::C3) {
            let arc = if flipped { A3_V } else { A3_A } + origin;
            if rng.gen_range(0..2) == 0 {
                arcs.push(arc + Radius { r: W } + SegmentOffset { offset: 12 });
                farcs.push(arc + Radius { r: -W });
                farcs.push(arc + Radius { r: W } + SegmentOffset { offset: 6 });
                paint_l3(&mut lines, origin, flipped);
            } else {
                arcs.push(arc + Radius { r: -W });
                arcs.push(arc + Radius { r: W });
                if flipped {
                    push4(&mut lines, origin, L011, L012, L013, L014);
                    push2(&mut faint, origin, L023, L024);
                } else {
                    push4(&mut lines, origin, L035, L036, L037, L038);
                    push2(&mut faint, origin, L047, L048);
                }
            }
        }

        if mask.contains(Tile::X1) {
            arcs.push(if flipped { A1_V } else { A1_A } + origin + Radius { r: -W });
            arcs.push(if flipped { A2_V } else { A2_A } + origin + Radius { r: -W });
            arcs.push(if flipped { A3_V } else { A3_A } + origin + Radius { r: -W });
        }
        if mask.contains(Tile::X2) {
            match rng.gen_range(0..3) {
                // 0,1,2
                0 => {
                    paint_y1(&mut lines, &mut faint, origin, flipped);
                    if flipped {
                        push2(&mut lines, origin, L007, L008);
                    } else {
                        push2(&mut lines, origin, L005, L006);
                    }
                }
                1 => {
                    paint_y3(&mut lines, &mut faint, origin, flipped);
                    if flipped {
                        push2(&mut lines, origin, L011, L012);
                    } else {
                        push2(&mut lines, origin, L035, L036);
                    }
                }
                _ => {
                    paint_y2(&mut lines, &mut faint, origin, flipped);
                    if flipped {
                        push2(&mut lines, origin, L001, L002);
                    } else {
                        push2(&mut lines, origin, L025, L026);
                    }
                }
            }
        }

        if mask.contains(Tile::W1) {
            create_triangle(flipped).map(|line| lines.push(line + origin));
        }

        return Glyph {
            lines,
            faint,
            arcs,
            semi,
            fsemi,
            farcs,
            nets,
        };
    }

    pub fn max(&self) -> u32 {
        return self.nets.into_iter().max().unwrap_or_default();
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

#[cfg(test)]
mod tests {
    use crate::maze::tile::Tile;
    use crate::renders::glyph::Glyph;
    use crate::renders::primatives::Coordinate;

    #[test]
    fn zero_nets() {
        let title = Glyph::new(Coordinate { i: 1, j: 1 }, Tile::W1, 0);
        assert_eq!(title.nets, [0u32, 0u32, 0u32, 0u32, 0u32, 0u32]);
        assert_eq!(title.max(), 0u32);
    }

    #[test]
    fn a1_nets() {
        let title = Glyph::new(Coordinate { i: 1, j: 1 }, Tile::A1, 0);
        assert_eq!(title.nets, [0u32, 0u32, 0u32, 1u32, 1u32, 0u32]);
        assert_eq!(title.max(), 1u32);
    }

    #[test]
    fn x2_nets() {
        let title = Glyph::new(Coordinate { i: 1, j: 1 }, Tile::X2, 0);
        assert_eq!(title.nets, [1u32, 2u32, 3u32, 3u32, 2u32, 1u32]);
        assert_eq!(title.max(), 3u32);
    }

    #[test]
    fn a1_l2_nets() {
        let title = Glyph::new(Coordinate { i: 1, j: 1 }, Tile::A1 | Tile::L1, 0);
        assert_eq!(title.nets, [2u32, 0u32, 0u32, 1u32, 1u32, 2u32]);
        assert_eq!(title.max(), 2u32);
    }
}

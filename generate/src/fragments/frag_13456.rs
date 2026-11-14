use common::consts::FILL;
use common::types::format::trimf64;
use common::types::path::Path;
use common::types::point::{Point, PATH_WIDTH, S2, S3, S4, TILE_WIDTH};

/// Radius of the corner wall that turns between the two edge openings beside the
/// missing vertex (35/120 of a tile). Larger is a gentler bend with shorter straights.
pub const INNER_CORNER_RADIUS: f64 = 35.0 / TILE_WIDTH;

fn base(invert: bool) -> (String, String, Path) {
    let w = PATH_WIDTH;

    let pt1 = Point {
        x: 0.5f64 + -w,
        y: S3,
    };
    let pt2 = Point {
        x: 0.5f64 + -w,
        y: S4,
    };
    let pt3 = Point {
        x: 0.25f64 * (1. + (2. * w)),
        y: 0.25f64 * S2 * (1. + (2. * w)),
    };
    let pt4 = Point {
        x: 0.25f64 * (1. + ((-2.) * w)),
        y: 0.25f64 * S2 * (1. + ((-2.) * w)),
    };
    // The wall round vertex 2 (vertex 3 once reflected) is one corner: straight from
    // the edge 4 opening, a curve of INNER_CORNER_RADIUS, straight into the edge 6 opening.
    let corner = Point {
        x: 0.5 - w,
        y: pt4.y - (0.5 - w - pt4.x) * S4,
    };
    let along = INNER_CORNER_RADIUS * S4; // tangent length for the 60° turn
    let pt5 = Point {
        x: corner.x - along * S3,
        y: corner.y + along * 0.5,
    };
    let pt6 = Point {
        x: corner.x,
        y: corner.y - along,
    };
    let pt7 = Point {
        x: 0.5f64 + -w,
        y: 0.,
    };
    let pt8 = Point {
        x: 0.5f64 + w,
        y: 0.,
    };
    let pt9 = Point {
        x: 0.25f64 * (3. + ((-2.) * w)),
        y: 0.25f64 * S4 * (1. + ((-6.) * w)),
    };
    let pt10 = Point {
        x: 1. + ((-0.5f64) * w),
        y: (-0.5f64) * S2 * w,
    };
    let pt11 = Point {
        x: 0.5f64 * (2. + w),
        y: S3 * w,
    };
    let pt12 = Point {
        x: 0.25f64 * (3. + (2. * w)),
        y: 0.25f64 * S4 * (1. + (6. * w)),
    };
    let pt13 = Point {
        x: 0.25f64 * (3. + (2. * w)),
        y: 0.25f64 * S2 * (1. + ((-2.) * w)),
    };
    let pt14 = Point {
        x: 0.25f64 * (3. + ((-2.) * w)),
        y: 0.25f64 * S2 * (1. + (2. * w)),
    };
    let pt15 = Point {
        x: 0.5f64 + w,
        y: S4,
    };
    let pt16 = Point {
        x: 0.5f64 + w,
        y: S3,
    };

    let pts = Path::new(vec![
        pt1, pt2, pt3, pt4, pt5, pt6, pt7, pt8, pt9, pt10, pt11, pt12, pt13, pt14, pt15, pt16,
    ]);
    let pts = if invert { pts.invert() } else { pts };
    let r1s = trimf64(TILE_WIDTH * (1. / 6. - w));
    let rc = trimf64(TILE_WIDTH * INNER_CORNER_RADIUS);
    (r1s, rc, pts)
}

fn to_str(invert: bool, pts: Path, r1s: String, rc: String) -> String {
    let d1 = if invert { 0 } else { 1 };
    let pts = pts.str();
    format!(
        "<path fill='{fill}' d='M{a}L{b}A{r1s} {r1s} 0 0 {d1} {c}L{d}L{e}A{rc} {rc} 0 0 {d1} {f}L{g}L{h}A{r1s} {r1s} 0 0 {d1} {i}L{j}L{k}L{l}A{r1s} {r1s} 0 0 {d1} {m}L{n}A{r1s} {r1s} 0 0 {d1} {o}L{p}'/>
        <path stroke='black' d='M{a}L{b}A{r1s} {r1s} 0 0 {d1} {c}M{d}L{e}A{rc} {rc} 0 0 {d1} {f}L{g}M{h}A{r1s} {r1s} 0 0 {d1} {i}L{j}M{k}L{l}A{r1s} {r1s} 0 0 {d1} {m}M{n}A{r1s} {r1s} 0 0 {d1} {o}L{p}'/>",
        a = pts[0],
        b = pts[1],
        c = pts[2],
        d = pts[3],
        e = pts[4],
        f = pts[5],
        g = pts[6],
        h = pts[7],
        i = pts[8],
        j = pts[9],
        k = pts[10],
        l = pts[11],
        m = pts[12],
        n = pts[13],
        o = pts[14],
        p = pts[15],
        fill = FILL,
    )
}

pub fn n13456(invert: bool) -> String {
    let (r1s, rc, pts) = base(invert);
    to_str(!invert, pts, r1s, rc)
}

pub fn n12456(invert: bool) -> String {
    let (r1s, rc, pts) = base(invert);
    let pts = pts.reflect();
    to_str(invert, pts, r1s, rc)
}

pub fn n23456(invert: bool) -> String {
    let (r1s, rc, pts) = base(invert);
    let pts = pts.reflect().rotate_ccw(
        invert,
        vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
    );
    to_str(invert, pts, r1s, rc)
}

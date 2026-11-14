use common::consts::FILL;
use common::define_shape_family;
use common::macros::{identity, rotate_ccw, rotate_cw};
use common::types::format::trimf64;
use common::types::path::Path;
use common::types::point::{Point, PATH_WIDTH, S2, S3, S4, TILE_WIDTH};

/// Radius of the curve in each of the two long walls (the walls that run from one
/// vertex arm round to the next), as a fraction of the tile width. Each long wall is
/// straight, this one curve, then straight again.
const LONG_WALL_RADIUS: f64 = 35.0 / TILE_WIDTH;

fn unit(p: Point) -> Point {
    let length = (p.x * p.x + p.y * p.y).sqrt();
    p * (1.0 / length)
}

/// A wall that runs straight from `start` along `d_in`, then curves with radius
/// `radius` onto the straight line that reaches `end` along `d_out`. Returns the
/// points where the curve starts and ends.
fn rounded_corner(
    start: Point,
    d_in: Point,
    end: Point,
    d_out: Point,
    radius: f64,
) -> (Point, Point) {
    // Where the two straight lines meet: start + s * d_in = end - u * d_out.
    let cross = d_in.x * d_out.y - d_in.y * d_out.x;
    let gap = end - start;
    let s = (gap.x * d_out.y - gap.y * d_out.x) / cross;
    let corner = start + d_in * s;
    // The curve turns through the angle between the lines.
    let turn = (d_in.x * d_out.x + d_in.y * d_out.y)
        .clamp(-1.0, 1.0)
        .acos();
    let tangent = radius * (turn / 2.0).tan();
    let from_end = gap - d_in * s;
    let room = (from_end.x * from_end.x + from_end.y * from_end.y).sqrt();
    assert!(
        tangent <= s && tangent <= room,
        "frag_1235: LONG_WALL_RADIUS is too large for the wall"
    );
    (corner - d_in * tangent, corner + d_out * tangent)
}

fn base(invert: bool) -> (Vec<String>, Path) {
    let w = PATH_WIDTH;

    let pt1 = Point {
        x: 0.5f64 + -w,
        y: S3,
    };
    let pt2 = Point {
        x: 0.5f64 + -w,
        y: S4,
    };
    let pt4 = Point {
        x: 0.0833333f64 * (9. + ((-2.) * S2) + ((-6.) * w)),
        y: 0.0833333f64 * ((-2.) + (3. * S2) + (6. * S2 * w)),
    };
    let pt5 = Point {
        x: (-0.5f64) * w,
        y: S3 * w,
    };
    let pt6 = Point {
        x: 0.5f64 * w,
        y: (-0.5f64) * S2 * w,
    };
    let pt7 = Point {
        x: 0.0833333f64 * (9. + ((-2.) * S2) + (6. * w)),
        y: 0.0833333f64 * ((-2.) + (3. * S2) + ((-6.) * S2 * w)),
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

    // The two long walls keep their straight runs (pt1 -> pt2 into pt4 -> pt5, and
    // pt6 -> pt7 into pt9 -> pt10), joined by one curve each instead of two.
    let (a_in, a_out) =
        rounded_corner(pt1, unit(pt2 - pt1), pt5, unit(pt5 - pt4), LONG_WALL_RADIUS);
    let (b_in, b_out) = rounded_corner(
        pt6,
        unit(pt7 - pt6),
        pt10,
        unit(pt10 - pt9),
        LONG_WALL_RADIUS,
    );

    let pts = Path::new(vec![
        pt1, a_in, a_out, pt5, pt6, b_in, b_out, pt10, pt11, pt12, pt13, pt14, pt15, pt16,
    ]);

    let pts = if invert { pts.invert() } else { pts };
    let r1s = trimf64(TILE_WIDTH * (1. / 6. - w));
    let long = trimf64(TILE_WIDTH * LONG_WALL_RADIUS);
    (vec![r1s, long], pts)
}

fn svg(invert: bool, radii: Vec<String>, pts: Path) -> String {
    let d1 = if invert { 1 } else { 0 };
    let p = pts.str();
    let (r1s, long) = (&radii[0], &radii[1]);
    let wall_a = format!(
        "M{}L{}A{long} {long} 0 0 {d1} {}L{}",
        p[0], p[1], p[2], p[3]
    );
    let wall_b = format!(
        "M{}L{}A{long} {long} 0 0 {d1} {}L{}",
        p[4], p[5], p[6], p[7]
    );
    let short_a = format!("M{}L{}A{r1s} {r1s} 0 0 {d1} {}", p[8], p[9], p[10]);
    let short_b = format!("M{}A{r1s} {r1s} 0 0 {d1} {}L{}", p[11], p[12], p[13]);
    // The fill runs round the outline: each wall, then on to the next one.
    let fill = format!(
        "{wall_a}L{}L{}A{long} {long} 0 0 {d1} {}L{}L{}L{}A{r1s} {r1s} 0 0 {d1} {}L{}A{r1s} {r1s} 0 0 {d1} {}L{}",
        p[4], p[5], p[6], p[7], p[8], p[9], p[10], p[11], p[12], p[13]
    );
    format!(
        "<path fill='{FILL}' d='{fill}'/><path stroke='black' d='{wall_a}{wall_b}{short_a}{short_b}'/>"
    )
}

define_shape_family! {
    base: base,
    svg: svg,
    offset_rotations: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13],
    variants: {
        n1235: false, false => identity,
        n1234: false, false => rotate_ccw,
        n1236: false, false => rotate_cw,
    }
}

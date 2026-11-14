use common::consts::FILL;
use common::define_shape_family;
use common::macros::{
    identity, reflect, reflect_then_rotate_ccw, reflect_then_rotate_cw, rotate_ccw, rotate_cw,
};
use common::types::format::trimf64;
use common::types::path::Path;
use common::types::point::{Point, PATH_WIDTH, S2, S3, S4, TILE_WIDTH};

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
    let pt3 = Point {
        x: 0.25f64 * (1. + (2. * w)),
        y: 0.25f64 * S2 * (1. + (2. * w)),
    };
    let pt4 = Point {
        x: 0.25f64 * (1. + ((-2.) * w)),
        y: 0.25f64 * S2 * (1. + ((-2.) * w)),
    };
    let pt8 = Point {
        x: 1. + ((-0.5f64) * w),
        y: (-0.5f64) * S2 * w,
    };
    let pt9 = Point {
        x: 0.5f64 * (2. + w),
        y: S3 * w,
    };
    let pt10 = Point {
        x: 0.25f64 * (3. + (2. * w)),
        y: 0.25f64 * S4 * (1. + (6. * w)),
    };
    let pt11 = Point {
        x: 0.25f64 * (3. + (2. * w)),
        y: 0.25f64 * S2 * (1. + ((-2.) * w)),
    };
    let pt12 = Point {
        x: 0.25f64 * (3. + ((-2.) * w)),
        y: 0.25f64 * S2 * (1. + (2. * w)),
    };
    let pt13 = Point {
        x: 0.5f64 + w,
        y: S4,
    };
    let pt14 = Point {
        x: 0.5f64 + w,
        y: S3,
    };

    // The long wall runs straight from pt4 to pt8: the two ends lie on one line,
    // at the angle the arms meet them, so it needs no curve (it used to bulge into
    // the junction and back, which left a kink).
    let pts = Path::new(vec![
        pt1, pt2, pt3, pt4, pt8, pt9, pt10, pt11, pt12, pt13, pt14,
    ]);

    let pts = if invert { pts.invert() } else { pts };
    let r1s = trimf64(TILE_WIDTH * (1. / 6. - w));
    (vec![r1s], pts)
}

fn svg(invert: bool, radii: Vec<String>, pts: Path) -> String {
    let d1 = if invert { 0 } else { 1 };
    let r1s = &radii[0];
    let p = pts.str();
    format!(
        "<path fill='{FILL}' d='M{}L{}A{r1s} {r1s} 0 0 {d1} {}L{}L{}L{}L{}A{r1s} {r1s} 0 0 {d1} {}L{}A{r1s} {r1s} 0 0 {d1} {}L{}'/>\
        <path stroke='black' d='M{}L{}A{r1s} {r1s} 0 0 {d1} {}M{}L{}M{}L{}A{r1s} {r1s} 0 0 {d1} {}M{}A{r1s} {r1s} 0 0 {d1} {}L{}'/>",
        p[0], p[1], p[2], p[3], p[4], p[5], p[6], p[7], p[8], p[9], p[10],
        p[0], p[1], p[2], p[3], p[4], p[5], p[6], p[7], p[8], p[9], p[10],
    )
}

define_shape_family! {
    base: base,
    svg: svg,
    offset_rotations: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    variants: {
        n1345: false, true => identity,
        n1246: false, true => rotate_ccw,
        n2356: false, true => rotate_cw,
        n1245: false, false => reflect,
        n2346: false, false => reflect_then_rotate_ccw,
        n1356: false, false => reflect_then_rotate_cw,
    }
}

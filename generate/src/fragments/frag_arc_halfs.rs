use common::consts::FILL;
use common::define_shape_family;
use common::macros::{
    identity, reflect, reflect_then_rotate_ccw, reflect_then_rotate_cw, rotate_ccw, rotate_cw,
};
use common::types::format::trimf64;
use common::types::path::Path;
use common::types::point::{Point, PATH_WIDTH, S3, TILE_WIDTH};

fn base(invert: bool) -> (Vec<String>, Path) {
    let w = PATH_WIDTH;

    let r1 = 0.5 + w;
    let r2 = 0.5 - w;

    let a = Point {
        x: 0.25 - w / 2.0,
        y: 0.5 * S3 - w * S3,
    };

    let b = Point {
        x: 0.5,
        y: (S3 - r1),
    };
    let c = Point {
        x: 0.5,
        y: (S3 - r2),
    };

    let d = Point {
        x: 0.25 + w / 2.0,
        y: 0.5 * S3 + w * S3,
    };

    let r1 = trimf64(TILE_WIDTH * r1);
    let r2 = trimf64(TILE_WIDTH * r2);

    let pts = Path::new(vec![a, b, c, d]);
    let pts = if invert { pts.invert() } else { pts };
    (vec![r1, r2], pts)
}

fn svg(invert: bool, radii: Vec<String>, pts: Path) -> String {
    let (d1, d2) = if invert { (0, 1) } else { (1, 0) };
    let pts = pts.str();
    format!(
        "<path fill='{FILL}' d='M{a}A{r1} {r1} 0 0 {d1} {b}L{c}A{r2} {r2} 0 0 {d2} {d}'/>\
        <path stroke='black' d='M{a}A{r1} {r1} 0 0 {d1} {b}M{c}A{r2} {r2} 0 0 {d2} {d}'/>",
        a = pts[0],
        b = pts[1],
        c = pts[2],
        d = pts[3],
        r1 = radii[0],
        r2 = radii[1],
    )
}

define_shape_family! {
    base: base,
    svg: svg,
    offset_rotations: &[0, 1, 2, 3],
    variants: {
        a45_35_half: false, false => identity,
        a46_14_half: false, false => rotate_ccw,
        a56_26_half: false, false => rotate_cw,
        a45_24_half: false, true => reflect,
        a46_36_half: false, true => reflect_then_rotate_ccw,
        a56_15_half: false, true => reflect_then_rotate_cw,
    }
}

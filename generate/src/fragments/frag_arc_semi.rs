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
    let r1 = trimf64(TILE_WIDTH * (0.25 + w));
    let r2 = trimf64(TILE_WIDTH * (0.25 - w));

    let a = Point::new(0.5 - w / 2.0, S3 + S3 * w);
    let b = Point::new(0.75 + w / 2.0, S3 / 2.0 - S3 * w);
    let c = Point::new(0.75 - w / 2.0, S3 / 2.0 + S3 * w);
    let d = Point::new(0.5 + w / 2.0, S3 - S3 * w);

    let pts = Path::new(vec![a, b, c, d]);
    let pts = if invert { pts.invert() } else { pts };
    (vec![r1, r2], pts)
}

fn svg(invert: bool, radii: Vec<String>, pts: Path) -> String {
    let (d1, d2) = if invert { (0, 1) } else { (1, 0) };
    let pts = pts.str();
    format!(
        "<path stroke='black' fill='{fill}' d='M{a}A{r1} {r1} 0 0 {d1} {b}M{c}A{r2} {r2} 0 0 {d2} {d}'/>",
        a = pts[0],
        b = pts[1],
        c = pts[2],
        d = pts[3],
        fill = FILL,
        r1 = radii[0],
        r2 = radii[1],
    )
}

define_shape_family! {
    base: base,
    svg: svg,
    offset_rotations: &[0, 1, 2, 3],
    variants: {
        n45_14: false, false => identity,
        n45_15: false, true => reflect,
        n46_26: false, false => rotate_ccw,
        n46_24: false, true => reflect_then_rotate_ccw,
        n56_35: false, false => rotate_cw,
        n56_36: false, true => reflect_then_rotate_cw,
    }
}

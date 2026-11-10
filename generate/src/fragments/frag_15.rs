use common::consts::FILL;
use common::define_shape_family;
use common::macros::{
    identity, reflect, reflect_then_rotate_ccw, reflect_then_rotate_cw, rotate_ccw, rotate_cw,
};
use common::types::format::trimf64;
use common::types::path::Path;
use common::types::point::{Point, PATH_WIDTH, S2, S3, TILE_WIDTH};

fn base(invert: bool) -> (Vec<String>, Path) {
    let w = PATH_WIDTH;

    let r1 = 0.3333; // Adjustable

    let r2 = (1. + 2. * r1) / (4. + 12. * r1);

    let x1 = 0.5 - w;
    let x2 = (1. + 5. * r1 + 12. * r1 * r1 - w - 12. * r1 * w - 24. * r1 * r1 * w)
        / (2. * (1. + 6. * r1 + 12. * r1 * r1));
    let x3 = 0.75 + w / 2.0;

    let x4 = 0.75 - w / 2.0;
    let x5 = (1. + 5. * r1 + 12. * r1 * r1 + w + 12. * r1 * w + 24. * r1 * r1 * w)
        / (2. * (1. + 6. * r1 + 12. * r1 * r1));
    let x6 = 0.5 + w;

    let y1 = S3;
    let y2 = (-3. * (1. + 4. * r1) * (1. + 5. * r1 + 8. * r1 * r1 + w + 4. * r1 * w))
        / (2. * (-S2 - 4. * S2 * r1) * (1. + 6. * r1 + 12. * r1 * r1));
    let y3 = (0.5 - w) * S3;
    let y4 = (0.5 + w) * S3;
    let y5 = (-3. * (1. + 4. * r1) * (1. + 5. * r1 + 8. * r1 * r1 - w - 4. * r1 * w))
        / (2. * (-S2 - 4. * S2 * r1) * (1. + 6. * r1 + 12. * r1 * r1));
    let y6 = S3;

    let a = Point { x: x1, y: y1 };
    let b = Point { x: x2, y: y2 };
    let c = Point { x: x3, y: y3 };

    let d = Point { x: x4, y: y4 };
    let e = Point { x: x5, y: y5 };
    let f = Point { x: x6, y: y6 };

    let pts = Path::new(vec![a, b, c, d, e, f]);
    let pts = if invert { pts.invert() } else { pts };

    let r1s = trimf64(TILE_WIDTH * (r1 - w));
    let r2s = trimf64(TILE_WIDTH * (r2 + w));
    let r3s = trimf64(TILE_WIDTH * (r2 - w));
    let r4s = trimf64(TILE_WIDTH * (r1 + w));

    (vec![r1s, r2s, r3s, r4s], pts)
}

pub fn svg(invert: bool, radii: Vec<String>, pts: Path) -> String {
    let (d1, d2) = if invert { (0, 1) } else { (1, 0) };

    let pts = pts.str();
    format!(
        "<path fill='{fill}' d='M{a}A{r1} {r1} 0 0 {d2} {b}A{r2} {r2} 0 0 {d1} {c}L{d}A{r3} {r3} 0 0 {d2} {e}A{r4} {r4} 0 0 {d1} {f}'/><path stroke='black' d='M{a}A{r1} {r1} 0 0 {d2} {b}A{r2} {r2} 0 0 {d1} {c}M{d}A{r3} {r3} 0 0 {d2} {e}A{r4} {r4} 0 0 {d1} {f}'/>",
        a = pts[0],
        b = pts[1],
        c = pts[2],
        d = pts[3],
        e = pts[4],
        f = pts[5],
        r1=radii[0],
        r2=radii[1],
        r3=radii[2],
        r4=radii[3],
        fill = FILL,
    )
}

define_shape_family! {
    base: base,
    svg: svg,
    offset_rotations: &[0, 1, 2, 3, 4, 5],
    variants: {
        n15: false, false => identity,
        n24: false, false => rotate_ccw,
        n36: false, false => rotate_cw,
        n14: false, true => reflect,
        n26: false, true => reflect_then_rotate_ccw,
        n35: false, true => reflect_then_rotate_cw,
    }
}

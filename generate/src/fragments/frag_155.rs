use common::consts::FILL;
use common::define_shape_family;
use common::macros::{identity, reflect, reflect_then_rotate_ccw, reflect_then_rotate_cw, rotate_ccw, rotate_cw};
use common::types::format::trimf64;
use common::types::path::Path;
use common::types::point::{Point, PATH_WIDTH, S2, S3, TILE_WIDTH};

fn base(invert: bool) -> (Vec<String>, Path) {
    let w = PATH_WIDTH;

    let r = S2 - 3.0 / 2.0; // Sqrt[3] - (3/2)
    let o = 1f64 - S3; //  1 - Sqrt[3]/2

    let r1 = r + w;
    let r2 = r - w;

    let a = Point {
        x: 0.5 - w / 2.0,
        y: S3 + w * S3,
    };

    let b = a - Point {
        x: o * S3,
        y: o / 2.0,
    };

    let f = Point {
        x: 0.5 + w / 2.0,
        y: S3 - w * S3,
    };

    let e = f - Point {
        x: o * S3,
        y: o / 2.0,
    };

    let c = Point {
        x: 0.5,
        y: S3 - 0.5 - w,
    };

    let d = Point {
        x: 0.5,
        y: S3 - 0.5 + w,
    };

    let r1_str = trimf64(TILE_WIDTH * r1);
    let r2_str = trimf64(TILE_WIDTH * r2);

    let pts = Path::new(vec![a, b, c, d, e, f]);
    let pts = if invert { pts.invert() } else { pts };
    (vec![r1_str, r2_str], pts)
}

fn svg(invert: bool, radii: Vec<String>, pts: Path) -> String {
    let (d1, d2) = if invert { (0, 1) } else { (1, 0) };
    let pts = pts.str();
    format!(
        "<path fill='{fill}' d='M{a}L{b}A{r1} {r1} 0 0 {d1} {c}L{d}A{r2} {r2} 0 0 {d2} {e}L{f}'/>\
         <path stroke='black' d='M{a}L{b}A{r1} {r1} 0 0 {d1} {c}M{d}A{r2} {r2} 0 0 {d2} {e}L{f}'/>",
        a = pts[0],
        b = pts[1],
        c = pts[2],
        d = pts[3],
        e = pts[4],
        f = pts[5],
        fill = FILL,
        r1 = radii[0],
        r2 = radii[1],
    )
}

define_shape_family! {
    base: base,
    svg: svg,
    offset_rotations: &[0, 1, 2, 3, 4, 5],
    variants: {
        a144: false, false => identity,
        a266: false, false => rotate_ccw,
        a355: false, false => rotate_cw,
        a155: false, true => reflect,
        a244: false, true => reflect_then_rotate_ccw,
        a366: false, true => reflect_then_rotate_cw,
    }
}

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

    let r1 = 0.25 + w;
    let r2 = 0.25 - w;

    let x1 = 0.5f64 - w;
    let x2 = 0.5f64 + w;
    let x3 = 0.75f64;

    let y1 = S3;
    let y2 = S3 - r1;
    let y3 = S3 - r2;

    let a = Point { x: x1, y: y1 };
    let b = Point { x: x3, y: y2 };
    let c = Point { x: x3, y: y3 };
    let d = Point { x: x2, y: y1 };

    let pts = Path::new(vec![a, b, c, d]);
    let pts = if invert { pts.invert() } else { pts };

    let r1s = trimf64(r1 * TILE_WIDTH);
    let r2s = trimf64(r2 * TILE_WIDTH);
    let r3s = trimf64(PATH_WIDTH * TILE_WIDTH);

    (vec![r1s, r2s, r3s], pts)
}

fn svg(invert: bool, radii: Vec<String>, pts: Path) -> String {
    let (d1, d2, d3) = if invert { (0, 1, 0) } else { (1, 0, 1) };
    let pts = pts.str();

    format!(
        "<path stroke='black' fill='{fill}' d='\
         M{a}A{r1} {r1} 0 0 {d1} {b}\
         A{r3} {r3} 0 0 {d3} {c}\
         A{r2} {r2} 0 0 {d2} {d}\'/>",
        a = pts[0],
        b = pts[1],
        c = pts[2],
        d = pts[3],
        r1 = radii[0],
        r2 = radii[1],
        r3 = radii[2],
        fill = FILL
    )
}

define_shape_family! {
    base: base,
    svg: svg,
    offset_rotations: &[0, 1, 2, 3],
    variants: {
        n15_5: false, false => identity,
        n24_4: false, false => rotate_ccw,
        n36_6: false, false => rotate_cw,
        n14_4: false, true => reflect,
        n26_6: false, true => reflect_then_rotate_ccw,
        n35_5: false, true => reflect_then_rotate_cw,
    }
}

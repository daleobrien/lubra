use common::consts::FILL;
use common::define_shape_family_1r;
use common::macros::{identity, reflect, reflect_then_rotate_ccw, reflect_then_rotate_cw, rotate_ccw, rotate_cw};
use common::types::format::trimf64;
use common::types::path::Path;
use common::types::point::{Point, PATH_WIDTH, S3, TILE_WIDTH};

fn base(invert: bool) -> (String, Path) {
    let w = PATH_WIDTH;
    let h = PATH_WIDTH / 2.;
    let r = w / 2.;
    let x1 = 0.5 - w;
    let x2 = 0.5 + w;
    let y1 = S3;
    let y2 = S3 - h;

    let a = Point { x: x1, y: y1 };
    let b = Point { x: x1, y: y2 };
    let c = Point { x: x2, y: y2 };
    let d = Point { x: x2, y: y1 };

    let mut pts = Path::new(vec![a, b, c, d]);
    if invert {
        pts = pts.invert();
    }

    let r_str = trimf64(TILE_WIDTH * r);
    (r_str, pts)
}

fn svg(invert: bool, pts: &Path, r: &str) -> String {

    let d1 = if invert { 0 } else { 1 };
    let pts_str = pts.clone().str();
    format!(
        "<path stroke='black' fill='{fill}' d='M {a} L {b} A {r} {r} 0 0 {d1} {c} L {d}'/>",
        a = pts_str[0],
        b = pts_str[1],
        c = pts_str[2],
        d = pts_str[3],
        fill = FILL,
        r = r,
    )
}

define_shape_family_1r! {
    base: base,
    svg: svg,
    variants: {
        n1: false => identity,
        n2: false => rotate_ccw,
        n3: false => rotate_cw,
        n4: true => reflect_then_rotate_cw,
        n5: true => reflect_then_rotate_ccw,
        n6: true => reflect,
    }
}

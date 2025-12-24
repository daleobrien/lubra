use common::types::path::Path;
use common::types::point::{Point, PATH_WIDTH, S3};

use common::consts::FILL;
use common::define_shape_family;
use common::macros::{identity, rotate_ccw, rotate_cw};

fn base(invert: bool) -> (String, String, Path) {
    let w = Point {
        x: PATH_WIDTH,
        y: 0f64,
    };
    let a = Point { x: 0.5, y: S3 };
    let b = Point { x: 0.5, y: 0.0 };
    let pts  = Path::new(vec![a - w, b - w, b + w, a + w]).diff(invert);

    return ("".to_string(),"".to_string(), pts);
}

fn svg(_invert: bool, _r1: String, _r2: String, pts: Path) -> String {
  let pts = pts.str();
    return format!(
        "<path fill='{fill}' d='M{a}l{b}l{c}l{d}z '/> \
         <path stroke='black' d='M{a}l{b}m{c}l{d}z'/>",
        a = pts[0],
        b = pts[1],
        c = pts[2],
        d = pts[3],
        fill = FILL,
    );
}

define_shape_family! {
    base: base,
    svg: svg,
    offset_rotations: &[0,4],
    variants: {
        n16: false => identity,
        n25: false => rotate_ccw,
        n34: false => rotate_cw,
    }
}

// pub fn n16(invert: bool) -> String {
//     return svg(TILE_WIDTH * base(invert));
// }
//
// pub fn n25(invert: bool) -> String {
//     let pts = TILE_WIDTH * base(invert).rotate_ccw(invert, vec![0, 4]);
//     return svg(pts);
// }
//
// pub fn n34(invert: bool) -> String {
//     let pts = TILE_WIDTH * base(invert).rotate_cw(invert, vec![0, 4]);
//     return svg(pts);
// }

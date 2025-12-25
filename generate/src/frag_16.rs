use common::types::path::Path;
use common::types::point::{Point, PATH_WIDTH, S3};

use common::consts::FILL;
use common::define_shape_family;
use common::macros::{identity, rotate_ccw, rotate_cw};

fn base(invert: bool) -> (Vec<String>, Path) {
    let w = Point {
        x: PATH_WIDTH,
        y: 0f64,
    };
    let a = Point { x: 0.5, y: S3 };
    let b = Point { x: 0.5, y: 0.0 };
    let pts  = Path::new(vec![a - w, b - w, b + w, a + w]).diff(invert);

    return (vec![], pts);
}

fn svg(_invert: bool, _radii: Vec<String>, pts: Path) -> String {
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
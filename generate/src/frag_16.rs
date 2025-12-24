use common::types::path::Path;
use common::types::point::{Point, PATH_WIDTH, S3, TILE_WIDTH};

use common::consts::FILL;

fn base(invert: bool) -> Path {
    let w = Point {
        x: PATH_WIDTH,
        y: 0f64,
    };
    let a = Point { x: 0.5, y: S3 };
    let b = Point { x: 0.5, y: 0.0 };

    return Path::new(vec![a - w, b - w, b + w, a + w]).diff(invert);
}

fn svg(path: Path) -> String {
    let s = path.trim();
    return format!(
        "<path fill='{fill}' d='M{a}l{b}l{c}l{d}z '/> \
         <path stroke='black' d='M{a}l{b}m{c}l{d}z'/>",
        a = s[0],
        b = s[1],
        c = s[2],
        d = s[3],
        fill = FILL,
    );
}

pub fn n16(invert: bool) -> String {
    return svg(TILE_WIDTH * base(invert));
}

pub fn n25(invert: bool) -> String {
    let pts = TILE_WIDTH * base(invert).rotate_ccw(invert, vec![0, 4]);
    return svg(pts);
}

pub fn n34(invert: bool) -> String {
    let pts = TILE_WIDTH * base(invert).rotate_cw(invert, vec![0, 4]);
    return svg(pts);
}

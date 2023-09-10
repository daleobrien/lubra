use common::types::path::Path;
use common::types::point::{Point, PATH_WIDTH, S3, TILE_WIDTH};

const FILL: &str = "white";

fn generate_lx(invert: bool) -> Path {
    let w = Point {
        x: PATH_WIDTH,
        y: 0f64,
    };
    let a = Point { x: 0.5, y: S3 };
    let b = Point { x: 0.5, y: 0.0 };

    return Path::new(vec![a - w, b - w, b + w, a + w]).diff(invert);
}

fn l_a_to_b(path: Path) -> String {
    let s = path.trim();
    return format!(
        "<path fill=\\\"{FILL}\\\" d=\\\"M{a}l{b}l{c}l{d}z \\\"/> \
         <path stroke=\\\"black\\\" d=\\\"M{a}l{b}m{c}l{d}z\\\"/>",
        a = s[0],
        b = s[1],
        c = s[2],
        d = s[3]
    );
}

pub fn l16(invert: bool) -> String {
    return l_a_to_b(TILE_WIDTH * generate_lx(invert));
}

pub fn l25(invert: bool) -> String {
    let pts = TILE_WIDTH * generate_lx(invert).rotate_ccw(invert, vec![0, 4]);
    return l_a_to_b(pts);
}

pub fn l34(invert: bool) -> String {
    let pts = TILE_WIDTH * generate_lx(invert).rotate_cw(invert, vec![0, 4]);
    return l_a_to_b(pts);
}

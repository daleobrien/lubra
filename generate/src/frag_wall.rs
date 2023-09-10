use common::types::path::Path;
use common::types::point::{Point, S3, TILE_WIDTH};

fn triangle() -> Path {
    let a = Point { x: 0.0, y: 0.0 };
    let b = Point { x: 0.5, y: S3 };
    let c = Point { x: 1.0, y: 0.0 };
    Path::new(vec![a, b, c])
}

pub fn l_wall(invert: bool) -> String {
    let mut t = triangle();
    if invert {
        t = t.invert();
    }
    t = TILE_WIDTH * t;
    let s = t.clone().diff(false).trim();

    return format!(
        "<path fill-opacity=\\\"0.0\\\" stroke-opacity=\\\"0.1\\\" stroke=\\\"red\\\" d=\\\"M{a}l{b}l{c}z\\\" />",
        a = s[0],
        b = s[1],
        c = s[2],
    );
}

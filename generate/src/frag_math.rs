use common::types::format::trimf64;
use common::types::path::Path;
use common::types::point::{Point, PATH_WIDTH, S3, TILE_WIDTH};
use std::f64::consts::FRAC_PI_3;

pub fn polar(p: Point, radius: f64, angle: f64) -> Point {
    let (a, b) = f64::sin_cos(angle);
    return Point {
        x: p.x + radius * b,
        y: p.y + radius * a,
    };
}

pub fn ax(index: usize, invert: bool) -> (Path, String) {
    //

    let [start, end, r, y] = match (invert, index) {
        // Small Arc on Node 1
        (false, 1) => [0.0, 3.0, PATH_WIDTH, S3],
        (true, 1) => [3.0, 0.0, PATH_WIDTH, 0.0],

        // Inner Arc on Node 1 (Node 4-5)
        (false, 2) => [4.0, 5.0, (0.5 - PATH_WIDTH), S3],
        (true, 2) => [1.0, 2.0, (0.5 - PATH_WIDTH), 0.0],

        // Outer Arc on Node 1 (Node 4-5)
        (false, 3) => [5.0, 4.0, (0.5 + PATH_WIDTH), S3],
        (true, 3) => [2.0, 1.0, (0.5 + PATH_WIDTH), 0.0],

        // Small Arc on Node 6
        (false, 6) => [3.0, 0.0, PATH_WIDTH, 0.0],
        (true, 6) => [0.0, 3.0, PATH_WIDTH, S3],
        _ => panic!("Invalid index"),
    };
    let x = 0.5;
    let a = polar(Point { x, y }, r, start * FRAC_PI_3);
    let b = polar(Point { x, y }, r, end * FRAC_PI_3);

    let r = trimf64(TILE_WIDTH * r);
    return (Path::new(vec![a, b]), r);
}

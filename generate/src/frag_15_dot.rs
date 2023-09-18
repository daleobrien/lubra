use common::consts::FILL;
use common::types::format::trimf64;
use common::types::path::Path;
use common::types::point::{Point, PATH_WIDTH, S3, TILE_WIDTH};

fn a_base(invert: bool) -> (f64, f64, Path) {
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
    return (r1, r2, pts);
}

pub fn a15_5(invert: bool) -> String {
    let (r1, r2, pts) = a_base(invert);
    return a_str(invert, pts.str(), r1, r2);
}

pub fn a14_4(invert: bool) -> String {
    let (r1, r2, pts) = a_base(invert);
    let pts = pts.reflect();
    return a_str(!invert, pts.str(), r1, r2);
}

fn a_str(invert: bool, pts: Vec<String>, r1: f64, r2: f64) -> String {
    let (d1, d2, d3) = if invert { (0, 1, 0) } else { (1, 0, 1) };

    let string = format!(
        " <path stroke=\\\"black\\\" fill=\\\"{fill}\\\" d=\\\"\
           M {a} A {r1} {r1} 0 0 {d1} {b} \
                 A {r3} {r3} 0 0 {d3} {c} \
                 A {r2} {r2} 0 0 {d2} {d} \
          \\\" />",
        a = pts[0],
        b = pts[1],
        c = pts[2],
        d = pts[3],
        r1 = trimf64(r1 * TILE_WIDTH),
        r3 = trimf64(PATH_WIDTH * TILE_WIDTH),
        r2 = trimf64(r2 * TILE_WIDTH),
        fill = FILL
    );
    return string;
}

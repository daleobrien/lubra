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

    let r = 1.0f64 / 3.0f64; //

    let a = Point {
        x: 0.5f64 - w,
        y: S3,
    };

    let f = Point {
        x: 0.5f64 + w,
        y: S3,
    };

    let b = Point {
        x: 0.5f64 + r + (r + w) / 2f64,
        y: S3 - S3 * (r + w),
    };

    let e = Point {
        x: 0.5f64 + r + (r - w) / 2f64,
        y: S3 - S3 * (r - w),
    };

    let c = Point {
        x: 1.5f64 + w / 2.0,
        y: S3 - S3 * w,
    };

    let d = Point {
        x: 1.5f64 - w / 2.0,
        y: S3 + S3 * w,
    };

    let pts = Path::new(vec![a, b, c, d, e, f]);
    let pts = if invert { pts.invert() } else { pts };

    let r1 = trimf64((r + w) * TILE_WIDTH);
    let r2 = trimf64((r - w) * TILE_WIDTH);

    (vec![r1, r2], pts)
}

fn svg(invert: bool, radii: Vec<String>, pts: Path) -> String {
    let pts = pts.str();
    let (d1, d2) = if invert { (0, 1) } else { (1, 0) };

    format!(
        "<path fill='{fill}' d='\
           M{a}A{r1} {r1} 0 0 {d1} {b}\
                L{c}L{d}\
            L{e}A{r2} {r2} 0 0 {d2} {f}'/>\
          <path stroke='black' d='\
           M{a}A{r1} {r1} 0 0 {d1} {b}\
                L{c}M{d}\
            L{e}A{r2} {r2} 0 0 {d2} {f}'/>",
        a = pts[0],
        b = pts[1],
        c = pts[2],
        d = pts[3],
        e = pts[4],
        f = pts[5],
        d1 = d1,
        d2 = d2,
        r1 = radii[0],
        r2 = radii[1],
        fill = FILL
    )
}

define_shape_family! {
    base: base,
    svg: svg,
    offset_rotations: &[0, 1, 2, 3, 4, 5],
    variants: {
        n15_25: false, false => identity,
        n24_34: false, false => rotate_ccw,
        n36_16: false, false => rotate_cw,
        n14_34: false, true => reflect,
        n26_16: false, true => reflect_then_rotate_ccw,
        n35_25: false, true => reflect_then_rotate_cw,
    }
}

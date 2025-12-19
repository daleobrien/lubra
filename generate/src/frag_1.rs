use common::consts::FILL;
use common::types::format::trimf64;
use common::types::path::Path;
use common::types::point::{Point, PATH_WIDTH, S3, TILE_WIDTH};

fn a1_base(invert: bool) -> (String, Path) {
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

fn a1_str(invert: bool, pts: &Path, r: &str) -> String {
    let d1 = if invert { 0 } else { 1 };
    let pts_str = pts.str();
    format!(
        r#"<path  stroke="black" fill="{fill}" d="\
      M {a} L {b} \
      A {r} {r} 0 0 {d1} {c} \
      L {d} \
      \" />"#,
        a = pts_str[0],
        b = pts_str[1],
        c = pts_str[2],
        d = pts_str[3],
        fill = FILL,
        r = r,
    )
}

fn n_base(invert: bool, transform: fn(Path, bool) -> Path) -> String {
    let (r, pts) = a1_base(invert);
    let transformed_pts = transform(pts, invert);
    a1_str(invert, &transformed_pts, &r)
}

pub fn n1(invert: bool) -> String {
    n_base(invert, |pts, _| pts)
}

pub fn n2(invert: bool) -> String {
    n_base(invert, |pts, invert| {
        pts.rotate_ccw(invert, vec![0, 1, 2, 3])
    })
}

pub fn n3(invert: bool) -> String {
    n_base(invert, |pts, invert| {
        pts.rotate_cw(invert, vec![0, 1, 2, 3])
    })
}

pub fn n4(invert: bool) -> String {
    n_base(!invert, |pts, invert| {
        let reflected_pts = pts.reflect();
        reflected_pts.rotate_cw(invert, vec![0, 1, 2, 3])
    })
}

pub fn n5(invert: bool) -> String {
    n_base(!invert, |pts, invert| {
        let reflected_pts = pts.reflect();
        reflected_pts.rotate_ccw(invert, vec![0, 1, 2, 3])
    })
}

pub fn n6(invert: bool) -> String {
    n_base(!invert, |pts, _| pts.reflect())
}

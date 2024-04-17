use common::consts::FILL;
use common::types::format::trimf64;
use common::types::path::Path;
use common::types::point::{Point, PATH_WIDTH, S2, S3, S4, S5, TILE_WIDTH};

fn base(invert: bool) -> (String, String, String, Path) {
    let w = PATH_WIDTH;

    let pt1 = Point { x:0.5f64+((-1.)*w), y:S3};
    let pt2 = Point { x:0.5f64+((-1.)*w), y:S4};
    let pt3 = Point { x:0.25f64*(1.+(2.*w)), y:0.25f64*S2*(1.+(2.*w))};
    let pt4 = Point { x:0.25f64*(1.+((-2.)*w)), y:0.25f64*S2*(1.+((-2.)*w))};
    let pt5 = Point { x:0.333333f64, y:(-0.166667f64)+S4+((-1.)*w)};
    let pt6 = Point { x:0.0833333f64*(8.+((-1.)*S2)+((-6.)*S2*w)), y:0.0833333f64*(1.+(6.*w))};
    let pt7 = Point { x:0.5f64+((-1.)*w), y:0.};
    let pt8 = Point { x:0.5f64+w, y:0.};
    let pt9 = Point { x:0.25f64*(3.+((-2.)*w)), y:0.25f64*S4*(1.+((-6.)*w))};
    let pt10 = Point { x:1.+((-0.5f64)*w), y:(-0.5f64)*S2*w};
    let pt11 = Point { x:0.5f64*(2.+w), y:S3*w};
    let pt12 = Point { x:0.0833333f64*(7.+(6.*w)), y:0.0833333f64*S4*(5.+(18.*w))};
    let pt13 = Point { x:0.5f64+w, y:0.666667f64*S4};
    let pt14 = Point { x:0.5f64+w, y:S3};

    let pts = Path::new(vec![pt1, pt2, pt3, pt4, pt5, pt6, pt7, pt8, pt9, pt10, pt11, pt12, pt13, pt14]);


    let pts = Path::new(vec![pt1, pt2, pt3, pt4, pt5, pt6, pt7, pt8, pt9, pt10, pt11, pt12, pt13, pt14]);

    let pts = if invert { pts.invert() } else { pts };
    let r1s = trimf64(TILE_WIDTH * (1. / 6. - w));
    let r1l = trimf64(TILE_WIDTH * (1. / 6. + w));
    let r3 = trimf64(TILE_WIDTH * (S5 * w - 1. / 6.));
    return (r1s, r1l, r3, pts);
}

fn to_str(invert: bool, pts: Path, r1s: String, r1l: String, r3: String) -> String {
    let (d1, d2) = if invert { (0, 1) } else { (1, 0) };
    let pts = pts.str();
    return format!(
        "<path fill=\\\"{fill}\\\" d=\\\"\
      M {a} \
      L {b} \
      A {r1s} {r1s} 0 0 {d1} {c} \
      L {d} \
      A {r1l} {r1l} 0 0 {d2} {e} \
      A {r3} {r3} 0 0 {d1} {f} \
      A {r1l} {r1l} 0 0 {d2} {g} \
      L {h} \
      A {r1s} {r1s} 0 0 {d1} {i} \
      L {j} \
      L {k} \
      L {l} \
      A {r1s} {r1s} 0 0 {d1} {m} \
      L {n} \
      \\\"/>\
      <path stroke=\\\"black\\\" d=\\\"\
      M {a} \
      L {b} \
      A {r1s} {r1s} 0 0 {d1} {c} \
      M {d} \
      A {r1l} {r1l} 0 0 {d2} {e} \
      A {r3} {r3} 0 0 {d1} {f} \
      A {r1l} {r1l} 0 0 {d2} {g} \
      M {h} \
      A {r1s} {r1s} 0 0 {d1} {i} \
      L {j} \
      M {k} \
      L {l} \
      A {r1s} {r1s} 0 0 {d1} {m} \
      L {n} \
      \\\"/>",
        a = pts[0],
        b = pts[1],
        c = pts[2],
        d = pts[3],
        e = pts[4],
        f = pts[5],
        g = pts[6],
        h = pts[7],
        i = pts[8],
        j = pts[9],
        k = pts[10],
        l = pts[11],
        m = pts[12],
        n = pts[13],
        fill = FILL,
    );
}

pub fn n1346(invert: bool) -> String {
    let (r1s, r1l, r3,pts) = base(invert);
    return to_str(!invert, pts, r1s, r1l, r3);
}

pub fn n1236(invert: bool) -> String {
    let (r1s, r1l, r3, pts) = base(invert);
    let pts = pts.reflect();
    return to_str(invert, pts, r1s, r1l, r3);
}


pub fn n2345(invert: bool) -> String {
    let (r1s, r1l,r3, pts) = base(invert);
    let pts = pts.rotate_cw(invert, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13]);
    return to_str(!invert, pts, r1s, r1l,r3);
}

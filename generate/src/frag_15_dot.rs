fn a_base(invert: bool) -> (String, String, Path) {
    let w = PATH_WIDTH;

    let r1 = trimf64(TILE_WIDTH * (0.25 + w));
    let r2 = trimf64(TILE_WIDTH * (0.25 - w));
    let r3 = trimf64(TILE_WIDTH);

    let x1 = 0.5 - w;
    let x2 = 1.0 + w;

    let y = S3;
    let a = Point { x: x1, y };
    let b = Point { x: x2, y };
    let c = Point { x: x3, y };
    let d = Point { x: x4, y };

    let pts = Path::new(vec![a, b, c, d]);
    let pts = if invert { pts.invert() } else { pts };
    return (r1, r2, pts);
}

pub fn a15_45(invert: bool) -> String {
    let (r1, r2, pts) = a_base(invert);
    return a_str(invert, pts, r1, r2);
}

fn a_str(invert: bool, pts: Vec<String>, r1: f64, r2: f64) -> String {
    let (d1, d2) = if invert { (0, 1) } else { (1, 0) };

    return format!(
        "<path fill=\\\"{fill}\\\" d=\\\"\
           M {a} A {r1_l} {r1_l} 0 0 {d1} {b} \
                 A {r2_s} {r2_s} 0 0 {d2} {c} \
           L {d} A {r2_l} {r2_l} 0 0 {d1} {e} \
                 A {r1_s} {r1_s} 0 0 {d2} {f} \
          \\\" />\
          <path stroke=\\\"black\\\" d=\\\"\
           M {a} A {r1_l} {r1_l} 0 0 {d1} {b} \
                 A {r2_s} {r2_s} 0 0 {d2} {c} \
           M {d} A {r2_l} {r2_l} 0 0 {d1} {e} \
                 A {r1_s} {r1_s} 0 0 {d2} {f} \
          \\\" />",
        a = pts[0],
        b = pts[1],
        c = pts[2],
        d = pts[3],
        e = pts[4],
        f = pts[5],
        r1 = trimf64(r1 * TILE_WIDTH),
        r2 = trimf64(r2 * TILE_WIDTH),
        r3 = trimf64(TILE_WIDTH),
        fill = FILL
    );
}

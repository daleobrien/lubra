use crate::types::{diff, Point, P1, P6, PATH_WIDTH, S3, SCALE, SVG_FILL, SVG_LINE};
use std::f64::consts::FRAC_PI_6;
use std::ops::{Mul, Sub};

fn wrap_format_with_function(name: &str, svg: String) -> String {
    format!(
        "pub fn {}(offset: Point) -> String \n{{\n  writeln!(\"{}\", x=offset.x, y=offset.y)\n}}",
        name, svg
    )
}

fn generate_Lx() -> Vec<Point> {
    let w = Point {
        x: PATH_WIDTH,
        y: 0f64,
    };
    return diff(vec![P1 - w, P6 - w, P6 + w, P1 + w]);
}

fn trimf64(num: f64) -> String {
    let space = if num < 0f64 { "" } else { " " };
    let formatted = format!("{space}{:.*}", 4, num);
    let trimmed = formatted.trim_end_matches('0').trim_end_matches('.');
    return trimmed.to_string();
}

fn trim(point: Point) -> String {
    return format!("{} {}", trimf64(point.x), trimf64(point.y));
}

fn generate_Lx_String(pts: Vec<Point>) -> String {
    // TODO: Put {{x}} {{y}} back in

    let s = pts.iter().map(|p| trim(*p)).collect::<Vec<_>>();
    return format!(
        "<path {SVG_FILL} d=\"M0 0m{a}l{b}l{c}l{d}z\"/><path {SVG_LINE} d=\"M0 0m{a}l{b}m{c}l{d}z\"/>\n",
        a = s[0],
        b = s[1],
        c = s[2],
        d = s[3],
    );
}

fn generate_L16_fn() -> String {
    return generate_Lx_String(generate_Lx());
}

fn generate_L25_fn() -> String {
    let pts = generate_Lx()
        .iter()
        .enumerate()
        .map(|(i, p)| p.rotate_cw(i, vec![0, 4]))
        .collect::<Vec<_>>();
    return generate_Lx_String(pts);
}

fn generate_L34_fn() -> String {
    let pts = generate_Lx()
        .iter()
        .enumerate()
        .map(|(i, p)| p.rotate_ccw(i, vec![0, 4]))
        .collect::<Vec<_>>();
    return generate_Lx_String(pts);
}

pub fn polar(p: Point, radius: f64, angle: f64) -> Point {
    let (a, b) = f64::sin_cos(angle);
    return Point {
        x: p.x + radius * b,
        y: p.y + radius * a,
    };
}

fn generate_Ax(index: usize) -> [Point; 3] {
    //
    let [start, end, r, y] = match index {
        0 => [
            12.0 * FRAC_PI_6,
            18.0 * FRAC_PI_6,
            PATH_WIDTH * SCALE,
            S3 * SCALE,
        ],
        1 => [18.0 * FRAC_PI_6, 12.0 * FRAC_PI_6, PATH_WIDTH * SCALE, 0f64],
        2 => [
            8.0 * FRAC_PI_6,
            10.0 * FRAC_PI_6,
            (0.5 - PATH_WIDTH) * SCALE,
            S3 * SCALE,
        ],
        3 => [
            10.0 * FRAC_PI_6,
            8.0 * FRAC_PI_6,
            (0.5 + PATH_WIDTH) * SCALE,
            S3 * SCALE,
        ],
        _ => panic!("Invalid index"),
    };
    let x = 0.5 * SCALE;
    let a = polar(Point { x, y }, r, start);
    let b = polar(Point { x, y }, r, end);
    return [a, b, Point { x: r, y: r }];
}

fn generate_A1_or_6_fn(index: usize) -> String {
    let [a, b, r] = generate_Ax(index).map(|p| trim(p));
    return format!("<path d=\"M {a} A {r} 0 0 0  {b} \"/>",);
}

fn generate_A1_fn() -> String {
    return generate_A1_or_6_fn(0);
}

fn generate_A6_fn() -> String {
    return generate_A1_or_6_fn(1);
}

fn generate_A2_to_3_fn() -> String {
    // With this and others, I don't need to draw a shape, just a really fat line!
    let [a, b, r1] = generate_Ax(2).map(|p| trim(p));
    let [c, d, r2] = generate_Ax(3).map(|p| trim(p));
    return format!(
        "<path {SVG_FILL} d=\"M {a} A {r1} 0 0 1  {b} L {c} A {r2} 0 0 0  {d} z\"/>
         <path {SVG_LINE} d=\"M {a} A {r1} 0 0 1  {b} \"/>\
         <path {SVG_LINE} d=\"M {c} A {r2} 0 0 0  {d} \"/>",
    );
}

fn generate_A2_or_5_fn(index: usize) -> String {
    let [a, b, r] = generate_Ax(index);
    let pts = [
        trim(a.rotate_cw(0, vec![0])),
        trim(b.rotate_cw(0, vec![0])),
        trim(r),
    ];
    return format!(
        "<path d=\"M{a}A{r} 0 0 0 {b}\"/>",
        a = pts[0],
        b = pts[1],
        r = pts[2]
    );
}
fn generate_A2_fn() -> String {
    return generate_A2_or_5_fn(0);
}
fn generate_A4_fn() -> String {
    return generate_A2_or_5_fn(1);
}

fn generate_A3_or_5fn(index: usize) -> String {
    let [a, b, r] = generate_Ax(index);
    let pts = [
        trim(a.rotate_ccw(0, vec![0])),
        trim(b.rotate_ccw(0, vec![0])),
        trim(r),
    ];
    return format!(
        "<path d=\"M{a}A{r} 0 0 0{b}\"/>",
        a = pts[0],
        b = pts[1],
        r = pts[2]
    );
}
fn generate_A3_fn() -> String {
    return generate_A3_or_5fn(0usize);
}
fn generate_A5_fn() -> String {
    return generate_A3_or_5fn(1usize);
}

pub fn generate_all_shapes() {
    //println!("{}", wrap_format_with_function("L1", generate_L1_fn()));
    //println!("{}", wrap_format_with_function("L2", generate_L2_fn()));
    // println!("{}", generate_L16_fn());
    // println!("{}", generate_L25_fn());
    // println!("{}", generate_L34_fn());
    // println!("{}", generate_A1_fn());
    // println!("{}", generate_A2_fn());
    // println!("{}", generate_A3_fn());
    // println!("{}", generate_A4_fn());
    // println!("{}", generate_A5_fn());
    // println!("{}", generate_A6_fn());
    println!("{}", generate_A2_to_3_fn());
}

#[cfg(test)]
mod tests {
    use crate::types::SCALE;
    use approx::assert_ulps_eq;
    use std::vec;

    extern crate approx;

    #[test]
    fn test_rotate() {
        use super::*;
        let p = SCALE * Point { x: 1f64, y: 0f64 };
        let r = p.rotate_ccw(0, vec![0]);
        assert_ulps_eq!(r.x, SCALE * 0.5f64);
        assert_ulps_eq!(r.y, SCALE * S3);

        let q = r.rotate_cw(0, vec![0]);
        assert_ulps_eq!(q.x, SCALE * 1f64);
        assert_ulps_eq!(q.y, SCALE * 0f64, epsilon = 0.0000000000001f64);
    }
}

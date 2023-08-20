use crate::types::{diff, Point, P1, P6, PATH_WIDTH, S3, SCALE};
use std::collections::HashMap;
use std::f64::consts::FRAC_PI_6;
use std::fs::File;
use std::io::Write;
use std::ops::{Mul, Sub};
/*

Could I use a use statement within the SVG, and have the objects under a white block?

It can work, only if I don't set the stroke/fill on the source <g...>, I can set that at the svg level
 though, so it's transparent unless I set the colour in the "use" it.
 */
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

fn generate_Lx_String(pts: Vec<Point>) -> [String; 2] {
    let s = pts.iter().map(|p| trim(*p)).collect::<Vec<_>>();
    return [
        format!("M{a}l{b}l{c}l{d}z", a = s[0], b = s[1], c = s[2], d = s[3]),
        format!("M{a}l{b}m{c}l{d}z", a = s[0], b = s[1], c = s[2], d = s[3]),
    ];
}

fn generate_L16_fn() -> [String; 2] {
    return generate_Lx_String(generate_Lx());
}

fn generate_L25_fn() -> [String; 2] {
    let pts = generate_Lx()
        .iter()
        .enumerate()
        .map(|(i, p)| p.rotate_cw(i, vec![0, 4]))
        .collect::<Vec<_>>();
    return generate_Lx_String(pts);
}

fn generate_L34_fn() -> [String; 2] {
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

fn generate_A1_or_6_fn(index: usize) -> [String; 2] {
    let [a, b, r] = generate_Ax(index).map(|p| trim(p));
    return [format!("M {a} A {r} 0 0 0 {b}"), format!("")];
}

fn generate_A1_fn() -> [String; 2] {
    return generate_A1_or_6_fn(0);
}

fn generate_A6_fn() -> [String; 2] {
    return generate_A1_or_6_fn(1);
}

fn generate_A4_to_5_fn() -> [String; 2] {
    // With this and others, I don't need to draw a shape, just a really fat line!
    let [a, b, r1] = generate_Ax(2);
    let [c, d, r2] = generate_Ax(3);
    let [a, b, c, d, r1, r2] = [a, b, c, d, r1, r2].map(|p| trim(p));

    return [
        format!("M {a} A {r1} 0 0 1 {b} L {c} A {r2} 0 0 0 {d} z"),
        format!("M {a} A {r1} 0 0 1 {b} M {c} A {r2} 0 0 0 {d}"),
    ];
}

fn generate_A4_to_6_fn() -> [String; 2] {
    // With this and others, I don't need to draw a shape, just a really fat line!
    let [a, b, r1] = generate_Ax(2);
    let [c, d, r2] = generate_Ax(3);
    let [a, b, c, d] = [a, b, c, d].map(|p| p.rotate_cw(0, vec![0]));
    let [a, b, c, d, r1, r2] = [a, b, c, d, r1, r2].map(|p| trim(p));

    return [
        format!("M {a} A {r1} 0 0 1  {b} L {c} A {r2} 0 0 0  {d} z "),
        format!("M {a} A {r1} 0 0 1  {b} M {c} A {r2} 0 0 0  {d} ",),
    ];
}

fn generate_A5_to_6_fn() -> [String; 2] {
    // With this and others, I don't need to draw a shape, just a really fat line!
    let [a, b, r1] = generate_Ax(2);
    let [c, d, r2] = generate_Ax(3);
    let [a, b, c, d] = [a, b, c, d].map(|p| p.rotate_ccw(0, vec![0]));
    let [a, b, c, d, r1, r2] = [a, b, c, d, r1, r2].map(|p| trim(p));

    return [
        format!("M {a} A {r1} 0 0 1 {b} L {c} A {r2} 0 0 0  {d} z"),
        format!("M {a} A {r1} 0 0 1 {b} M {c} A {r2} 0 0 0 {d}"),
    ];
}

fn generate_A2_or_5_fn(index: usize) -> [String; 2] {
    let [a, b, r] = generate_Ax(index);
    let [a, b] = [a, b].map(|p| p.rotate_cw(0, vec![0]));
    let [a, b, r] = [a, b, r].map(|p| trim(p));
    return [format!("M {a} A {r} 0 0 0 {b}"), format!("")];
}
fn generate_A2_fn() -> [String; 2] {
    return generate_A2_or_5_fn(0);
}
fn generate_A4_fn() -> [String; 2] {
    return generate_A2_or_5_fn(1);
}

fn generate_A3_or_5fn(index: usize) -> [String; 2] {
    let [a, b, r] = generate_Ax(index);
    let [a, b] = [a, b].map(|p| p.rotate_ccw(0, vec![0]));
    let [a, b, r] = [a, b, r].map(|p| trim(p));
    return [format!("M{a}A{r} 0 0 0{b}",), format!("")];
}
fn generate_A3_fn() -> [String; 2] {
    return generate_A3_or_5fn(0usize);
}
fn generate_A5_fn() -> [String; 2] {
    return generate_A3_or_5fn(1usize);
}

fn wrap(name: &str, paths: [String; 2]) -> String {
    let a = format!(
        "<g id=\"{name}_fill\"><path d=\"{fill}\"/></g>",
        fill = paths[0]
    );

    if paths[1].is_empty() {
        return a;
    }
    let b = format!(
        "<g id=\"{name}_line\"><path d=\"{line}\"/></g>",
        line = paths[1]
    );

    return format!("{}\n{}", a, b);
}

pub fn generate_all_shapes() {
    let mut lines: HashMap<usize, String> = HashMap::new();
    let mut fills: HashMap<usize, String> = HashMap::new();

    for (key, [line, fill]) in [
        (0b100_001, generate_L16_fn()),
        (0b010_010, generate_L25_fn()),
        (0b001_100, generate_L34_fn()),
        (0b000_001, generate_A1_fn()),
        (0b000_010, generate_A2_fn()),
        (0b000_100, generate_A3_fn()),
        (0b001_000, generate_A4_fn()),
        (0b010_000, generate_A5_fn()),
        (0b100_000, generate_A6_fn()),
        (0b011_000, generate_A4_to_5_fn()),
        (0b01_0100, generate_A4_to_6_fn()),
        (0b11_0000, generate_A5_to_6_fn()),
    ] {
        lines.insert(key, line);
        if !fill.is_empty() {
            fills.insert(key, fill);
        }
    }

    // Change this to a look up function rather than a class with a HashMap.
    let mut file = File::create("../lib/src/renders/fragment.rs").expect("File Creation failed");
    let _ = file.write(b"/* This file is generated. */\n");
    let _ = file.write(b"use std::collections::HashMap;\n\n");

    let _ = write!(file, "pub const U: f64 = {SCALE}f64;\n",);
    let _ = write!(file, "pub const M: f64 = {m}f64;\n", m = SCALE / 2.0);
    let _ = file.write(b"// U * f64::sqrt(3.0) / 2.0;\n");
    let _ = write!(file, "pub const H: f64 = {m}f64;\n\n", m = S3 * SCALE);

    let _ = file.write(b"pub struct Fragment {\n");
    let _ = file.write(b"    strokes: HashMap<u8, String>,\n");
    let _ = file.write(b"    fills: HashMap<u8, String>,\n");
    let _ = file.write(b"}\n\n");

    let _ = file.write(b"impl Fragment {\n\n");
    let _ = file.write(b"    pub fn get(&self, key: &u8) -> [Option<&String>; 2] {\n");
    let _ = file.write(b"        return [self.strokes.get(key), self.fills.get(key)];\n");
    let _ = file.write(b"    }\n");

    let _ = file.write(b"\n    pub fn new() -> Fragment {\n");
    let _ = file.write(b"        let mut strokes:HashMap<u8, String> = HashMap::new();\n");
    let _ = file.write(b"        let mut fills:HashMap<u8, String> = HashMap::new();\n\n");
    let _ = file.write(b"\n");

    for (key, value) in lines {
        let _ = write!(
            file,
            "        strokes.insert(0b{:06b}, \"{}\".to_string());\n",
            key, value
        );
    }
    for (key, value) in fills {
        let _ = write!(
            file,
            "        fills.insert(0b{:06b}, \"{}\".to_string());\n",
            key, value
        );
    }
    let _ = file.write(b"\n        return Fragment {\n");
    let _ = file.write(b"             strokes: strokes,\n");
    let _ = file.write(b"             fills: fills,\n");
    let _ = file.write(b"        };\n");
    let _ = file.write(b"    }\n");
    let _ = file.write(b"}\n");

    let _ = file.flush();
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

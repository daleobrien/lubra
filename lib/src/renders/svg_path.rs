use crate::renders::primatives::{Line, Segment};

pub struct Path {
    stroke: String,
    stroke_width: f64,
    path: String,
    opacity: f64,
}

const PI_6:f64 = 2.0 * std::f64::consts::FRAC_PI_6;

const SMALL_ANGLES_FROM_SEGMENT:[[f64; 2]; 7] = [
   [0f64 ,0f64],
   [0.0 * PI_6, 1.0 * PI_6],
   [1.0 * PI_6, 2.0 * PI_6],
   [2.0 * PI_6, 3.0 * PI_6],
   [3.0 * PI_6, 4.0 * PI_6],
   [4.0 * PI_6, 5.0 * PI_6],
   [5.0 * PI_6, 6.0 * PI_6]];

 const LARGE_ANGLES_FROM_SEGMENT:[[f64; 2]; 7] = [
    [0f64 ,0f64],
    [0.0 * PI_6, 3.0 * PI_6],
    [1.0 * PI_6, 4.0 * PI_6],
    [2.0 * PI_6, 5.0 * PI_6],
    [3.0 * PI_6, 6.0 * PI_6],
    [4.0 * PI_6, 7.0 * PI_6],
    [5.0 * PI_6, 8.0 * PI_6]];

impl Path {
    pub fn new(stroke: String, stroke_width: f64, opacity: f64) -> Path {
        Path {
            stroke: stroke,
            stroke_width: stroke_width,
            path: String::new(),
            opacity: opacity,
        }
    }

    pub fn to_string(&self) -> String {
        let d = &self.path;
        let colour = &self.stroke;
        let w = &self.stroke_width;
        let opacity = &self.opacity;

        if d.is_empty() {
            return "".to_string();
        }
        return format!("<path stroke-opacity=\"{opacity}\" stroke-width=\"{w}\" stroke-linecap=\"round\" stroke=\"{colour}\" d=\"{d}\"/>",
                       opacity = opacity, w = w, colour = colour, d = d);
    }

    fn polar(&self, x: f64, y: f64, radius: f64, angle: f64) -> (f64, f64) {
        let (a, b) = f64::sin_cos(angle);
        return (x + (radius * b), y + (radius * a));
    }

    pub fn line(&mut self, line: &Line) {
        let path = format!(
            "M {} {} l {} {} z ",
            line.start.x, line.start.y, line.delta.x, line.delta.y
        );
        self.path.push_str(&path);
    }
    // https://developer.mozilla.org/en-US/docs/Web/SVG/Tutorial/Paths
    pub fn arc(&mut self, arc: &Segment) {
        let ccw = "1";
        let x = arc.centre.x;
        let y = arc.centre.y;
        let r = arc.radius;
        let segment = arc.segment;

        let [start, end] = SMALL_ANGLES_FROM_SEGMENT[segment % 7];

        let (a, b) = self.polar(x, y, r, start);
        let (c, d) = self.polar(x, y, r, end);
        self.path.push_str(&format!(
            "M {a} {b} A {r} {r} 0 0 {ccw} {c} {d} ",
            a = a,
            b = b,
            c = c,
            d = d,
            r = r,
            ccw = ccw,
        ));
    }

    pub fn semi(&mut self, arc: &Segment) {
        let ccw = "1";
        let x = arc.centre.x;
        let y = arc.centre.y;
        let r = arc.radius;
        let segment = arc.segment;

        assert!(segment <= 6 && segment > 0);

        let [start, end] = LARGE_ANGLES_FROM_SEGMENT[&segment % 7];
        let (a, b) = self.polar(x, y, r, start);
        let (c, d) = self.polar(x, y, r, end);
        self.path.push_str(&format!(
            "M {a} {b} A {r} {r} 0 0 {ccw} {c} {d} ",
            a = a,
            b = b,
            c = c,
            d = d,
            r = r,
            ccw = ccw,
        ));
    }
}

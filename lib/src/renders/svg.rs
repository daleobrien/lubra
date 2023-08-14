use crate::renders::_arc::Arc;
use crate::renders::_line::Line;
use crate::renders::_rect::Rect;
use crate::renders::_segment::Segment;

pub struct Path {
    stroke: String,
    stroke_width: f64,
    path: String,
    opacity: f64,
}

impl Path {
    pub fn new(stroke: String, stroke_width: f64, opacity: f64) -> Path {
        Path {
            stroke,
            stroke_width,
            path: String::new(),
            opacity,
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

    pub fn line(&mut self, line: &Line) {
        self.path.push_str(&line.to_string());
    }

    pub fn rect(&mut self, rect: &Rect) {
        self.path.push_str(&rect.to_string());
    }

    pub fn arc(&mut self, arc: &Arc) {
        self.path.push_str(&arc.to_string());
    }

    pub fn semi(&mut self, arc: &Segment) {
        self.path.push_str(&arc.to_string());
    }
}

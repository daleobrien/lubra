use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::io::prelude::*;

pub struct Path {
    stroke: String,
    stroke_width: f64,
    path: String,
    opacity: f64,
    S: f64,
}

impl Path {
    pub fn new(stroke: String, stroke_width: f64, opacity: f64) -> Path {
        let s = 30.0;

        Path {
            stroke: stroke,
            stroke_width: stroke_width,
            path: String::new(),
            opacity: opacity,
            S: s,
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
        return format!("<path stroke-opacity=\"{opacity}\" stroke-width=\"{w}\" stroke=\"{colour}\" d=\"{d}\"/>",
                       opacity = opacity, w = w, colour = colour, d = d);
    }

    pub fn append(&mut self, path: &str) {
        self.path.push_str(path);
    }
    pub fn s0(&mut self, x: f64, y: f64) {
        self.path.push_str(&format!("M{},{}", x, y));
    }
}

pub struct RenderSvg {
    pub style: Option<String>,
    paths: HashMap<String, Path>,
    width: f64,
    height: f64,
}

impl RenderSvg {
    pub fn new() -> Self {
        let mut paths: HashMap<String, Path> = HashMap::new();

        let path = Path::new("black".to_string(), 1.0, 1.0);

        paths.insert("key".to_string(), path);

        let mut obj = Self {
            style: None,
            paths: paths,
            width: 100.0,
            height: 100.0,
        };
        obj.add_to_path("key", "M 0 0 h 100 v 100 h -100 z");
        return obj;
    }

    pub fn add_to_path(&mut self, key: &str, text: &str) {
        let path = self.paths.get_mut(key).unwrap();
        path.append(text);
    }
    // Path commands
    // https://github.com/bodoni/svg/blob/master/src/node/element/path/data.rs
    pub fn render(self, fileName: String) -> Result<(), Box<dyn Error>> {
        let mut svg = String::new();

        let w = self.width.to_string();
        let h = self.height.to_string();

        let view_box = format!("viewBox=\"0 0 {w} {h}\" ", w = w, h = h);

        svg.push_str(&format!("<svg id=\"maze\" xmlns=\"http://www.w3.org/2000/svg\" {view_box} width=\"{w}px\" height=\"{h}px\" fill-opacity=\"0\" stroke=\"black\">", w = w, h = h, view_box = view_box));

        for (_key, path) in &self.paths {
            svg.push_str(&path.to_string());
        }
        svg.push_str("</svg>");

        fs::write(fileName, svg.as_bytes())?;
        Ok(())
    }
}

use crate::renders::registry::StringRegistry;
use common::types::point::{Point, TILE_HEIGHT, TILE_WIDTH};
use std::fmt;

pub struct Svg {
    width: f64,
    height: f64,
    layers: [String; 6], // Layers, fill & stroke, from bottom to top.
    glyphs: StringRegistry,
}

impl fmt::Display for Svg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Svg {
    pub fn new(width: f64, height: f64) -> Svg {
        Svg {
            width,
            height,
            layers: [
                String::new(),
                String::new(),
                String::new(),
                String::new(),
                String::new(),
                String::new(),
            ],
            glyphs: StringRegistry::new(),
        }
    }

    pub fn add_fragment(&mut self, fragment: &str, index: Point, layer_index: usize) {
        // let name = fragment;
        // println!("adding fragment: {}", layer_index);
        let name = self.glyphs.add(fragment);

        self.layers[layer_index].push_str(&format!(
            "\n  <use href=\"#{name}\" x=\"{x}\" y=\"{y}\"/>",
            x = index.x * TILE_WIDTH / 2.0,
            y = index.y * TILE_HEIGHT,
        ));
    }

    pub fn to_string(&self) -> String {
        let mut s = String::new();

        let margin = 15f64;
        s.push_str(&format!(
            "<svg \
               viewBox=\"{m} {m} {w} {h}\" \
               width=\"{w}px\" height=\"{h}px\" \
               xmlns=\"http://www.w3.org/2000/svg\" \
               stroke-width=\"3\" \
               fill-opacity=\"0.3\" \
               fill=\"transparent\" \
               stroke-linecap=\"round\" \
               stroke=\"transparent\" \
               >",
            m = -margin,
            w = self.width + margin * 2f64,
            h = self.height + margin * 2f64
        ));

        s.push_str(&format!("\n<defs>"));
        for (name, content) in self.glyphs.iter() {
            s.push_str(&format!(
                "\n  <g id=\"{name}\">{content}</g>",
                name = *name,
                content = content
            ));
        }
        s.push_str(&format!("\n</defs>\n"));

        // s.push_str(&format!(
        //     "<g transform=\"translate(0,{h}) scale(1, -1)\">",
        //     h = self.height - margin * 2f64
        // ));
        for layer in &self.layers {
            s.push_str(&layer.to_string());
        }
        //
        // s.push_str("</g>");
        s.push_str("\n</svg>");
        return s;
    }
}

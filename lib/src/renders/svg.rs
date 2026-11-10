use std::fmt;

use common::types::point::{Point, TILE_HEIGHT, TILE_WIDTH};

use crate::renders::registry::StringRegistry;

pub struct Svg {
    width: f64,
    height: f64,
    layers: [String; 6], // Layers, fill & stroke, from bottom to top.
    glyphs: StringRegistry,
}

impl fmt::Display for Svg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string(false))
    }
}

impl Svg {
    pub fn new(width: f64, height: f64) -> Svg {
        Svg {
            width,
            height,
            layers: std::array::from_fn(|_| String::new()),
            glyphs: StringRegistry::new(),
        }
    }

    pub fn add_fragment(&mut self, fragment: &str, index: Point, layer_index: usize) {
        // Sometimes we don't need to draw anything
        if fragment.is_empty() {
            return;
        }

        let name = self.glyphs.add(fragment);

        self.layers[layer_index].push_str(&format!(
            "\n  <use href='#{name}' x='{x:.2}' y='{y:.2}'/>",
            x = index.x * TILE_WIDTH / 2.0,
            y = index.y * TILE_HEIGHT,
        ));
    }

    pub fn add_text(&mut self, text: &str, position: Point, layer_index: usize) {
        // Add a text element at the specified position
        // Note: The coordinate system will be flipped by the main SVG transform
        self.layers[layer_index].push_str(&format!(
            "<text x='{x:.2}' y='{y:.2}' font-size='12' fill='red' font-family='monospace' text-anchor='middle' dominant-baseline='middle' transform='scale(1,-1) translate(0,{flip:.2})'>{text}</text>",
            x = position.x,
            y = position.y,
            flip = -2.0 * position.y, // Compensate for the flip transform
            text = text
        ));
    }

    pub fn to_string(&self, add_background_mask: bool) -> String {
        let mut s = String::new();

        let style = if add_background_mask {
            "style='background-color:lightblue;'"
        } else {
            ""
        };

        let margin = 15f64;
        s.push_str(&format!(
            "<svg viewBox='{m} {m} {w} {h}' width='{w}px' height='{h}px' xmlns='http://www.w3.org/2000/svg' stroke-width='{s}' fill-opacity='0.8' {style} stroke-opacity='1' fill='transparent' stroke-linecap='butt' stroke='transparent'>",
            m = -margin, w = self.width + margin * 2f64, h = self.height + margin * 2f64,
            s = TILE_WIDTH/15.0,
        ));

        s.push_str("<defs>");
        for (name, content) in self.glyphs.iter() {
            s.push_str(&format!(
                "<g id='{name}'>{content}</g>",
                name = *name,
                content = content
            ));
        }
        s.push_str("</defs>");

        s.push_str(&format!(
            "<g transform='translate(0,{h}) scale(1, -1)'>",
            h = self.height - margin * 2f64
        ));
        for layer in &self.layers {
            s.push_str(&layer.to_string());
        }
        //
        s.push_str("</g></svg>");
        s
    }
}

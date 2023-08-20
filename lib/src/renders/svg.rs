use crate::renders::_point::Point;
use std::collections::HashSet;
use std::fmt;

pub struct Svg {
    width: f64,
    height: f64,
    layers: [String; 12], // Layers, Fill, then stroke, from bottom to top.
    glyph_sections: HashSet<String>,
}

// impl AsRef<String> for Svg {
//     fn as_ref(&self) -> String {
//         return format!("{}", self);
//     }
// }

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
                String::new(),
                String::new(),
                String::new(),
                String::new(),
                String::new(),
                String::new(),
            ],
            glyph_sections: HashSet::new(),
        }
    }

    pub fn add_tile_fragment(
        &mut self,
        name: String,
        fragment: String,
        index: Point,
        layer_index: usize,
    ) {
        self.glyph_sections.insert(fragment.clone());

        self.layers[layer_index].push_str(&format!(
            "<use href=\"#{name}\" x=\"{x}\" y=\"{y}\")\">",
            x = index.x * 120.0,
            y = index.y * 103.0,
        ));
    }

    pub fn to_string(&self) -> String {
        let mut s = String::new();

        s.push_str(&format!(
            "<svg \
               viewBox=\"0 0 {w} {h}\" \
               width=\"{w}px\" height=\"{h}px\" \
               xmlns=\"http://www.w3.org/2000/svg\"\
               stroke-width=\"3\" \
               fill-opacity=\"0.03\" \
               fill=\"black\" \
               stroke-linecap=\"round\" \
               stroke=\"black\" >\
               >",
            w = self.width,
            h = self.height
        ));

        // // These are all the glyphs, ... tile fragments.
        // s.push_str(" <g id=\"glyphs\" stroke=\"transparent\" fill=\"transparent\">");
        // for content in &self.glyph_sections {
        //     s.push_str(&content.to_string());
        // }
        // s.push_str("</g>");
        //
        // // For each tile fragment, work the the layers
        // for layer in &self.layers {
        //     s.push_str(&layer.to_string());
        // }
        //
        // s.push_str("</svg>");
        return s;
    }
}

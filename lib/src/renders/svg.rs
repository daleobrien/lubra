use crate::renders::_point::Point;
use crate::renders::registry::StringRegistry;
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
        println!("adding fragment: {}", layer_index);
        let name = self.glyphs.add(fragment);

        self.layers[layer_index].push_str(&format!(
            "\n  <use href=\"#{name}\" x=\"{x}\" y=\"{y}\"/>",
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
               xmlns=\"http://www.w3.org/2000/svg\" \
               stroke-width=\"3\" \
               fill-opacity=\"0.03\" \
               fill=\"black\" \
               stroke-linecap=\"round\" \
               stroke=\"black\" \
               >",
            w = self.width,
            h = self.height
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

        for layer in &self.layers {
            s.push_str(&layer.to_string());
        }
        //
        s.push_str("</svg>");
        return s;
    }
}

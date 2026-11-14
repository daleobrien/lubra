use std::fmt;

use common::types::point::{Point, TILE_HEIGHT, TILE_WIDTH};

use crate::renders::registry::StringRegistry;

pub struct Svg {
    width: f64,
    height: f64,
    layers: [String; 6], // Layers, fill & stroke, from bottom to top.
    glyphs: StringRegistry,
    caption: Option<String>,
}

/// Height of the caption strip above the maze, in SVG units.
const CAPTION_HEIGHT: f64 = 28.0;

/// Radius of the rings drawn by `add_ring`.
const RING_RADIUS: f64 = 16.0;

/// Escapes text for use inside an SVG element.
fn escape_xml(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('\'', "&apos;")
        .replace('"', "&quot;")
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
            caption: None,
        }
    }

    /// A line of text drawn above the maze, in a strip added to the top of the image.
    pub fn set_caption(&mut self, caption: Option<String>) {
        self.caption = caption;
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

    /// Adds an unfilled ring centred on `(x, y)` (drawing units), to mark a spot.
    pub fn add_ring(&mut self, x: f64, y: f64, colour: &str, layer_index: usize) {
        self.layers[layer_index].push_str(&format!(
            "<circle cx='{x:.1}' cy='{y:.1}' r='{RING_RADIUS}' stroke='{colour}' stroke-width='3'/>"
        ));
    }

    pub fn add_text(&mut self, text: &str, position: Point, layer_index: usize) {
        self.add_coloured_text(text, position, layer_index, "red");
    }

    /// Adds text made of coloured parts, centred on `position`: each `(text, colour)`
    /// pair is drawn in its own colour.
    pub fn add_coloured_parts(
        &mut self,
        parts: &[(String, &str)],
        position: Point,
        layer_index: usize,
    ) {
        let spans: String = parts
            .iter()
            .map(|(text, colour)| format!("<tspan fill='{colour}'>{}</tspan>", escape_xml(text)))
            .collect();
        self.layers[layer_index].push_str(&format!(
            "<text x='{x:.2}' y='{y:.2}' font-size='12' font-family='monospace' text-anchor='middle' dominant-baseline='middle' transform='scale(1,-1) translate(0,{flip:.2})'>{spans}</text>",
            x = position.x,
            y = position.y,
            flip = -2.0 * position.y,
        ));
    }

    /// Adds `text` centred on `position`, in `colour` (any SVG colour).
    pub fn add_coloured_text(
        &mut self,
        text: &str,
        position: Point,
        layer_index: usize,
        colour: &str,
    ) {
        // Note: The coordinate system will be flipped by the main SVG transform
        self.layers[layer_index].push_str(&format!(
            "<text x='{x:.2}' y='{y:.2}' font-size='12' fill='{colour}' font-family='monospace' text-anchor='middle' dominant-baseline='middle' transform='scale(1,-1) translate(0,{flip:.2})'>{text}</text>",
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
        let caption_height = if self.caption.is_some() {
            CAPTION_HEIGHT
        } else {
            0.0
        };
        s.push_str(&format!(
            "<svg viewBox='{m} {top} {w} {h}' width='{w}px' height='{h}px' xmlns='http://www.w3.org/2000/svg' stroke-width='{s}' fill-opacity='0.8' {style} stroke-opacity='1' fill='transparent' stroke-linecap='butt' stroke='transparent'>",
            m = -margin, top = -margin - caption_height, w = self.width + margin * 2f64,
            h = self.height + margin * 2f64 + caption_height,
            s = TILE_WIDTH/15.0,
        ));
        if let Some(caption) = &self.caption {
            s.push_str(&format!(
                "<text x='0' y='{y}' font-size='16' fill='#333' font-family='monospace' dominant-baseline='middle'>{text}</text>",
                y = -margin - caption_height / 2.0,
                text = escape_xml(caption),
            ));
        }

        s.push_str("<defs>");
        for (name, content) in self.glyphs.iter() {
            s.push_str(&format!("<g id='{name}'>{content}</g>"));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_caption_adds_a_strip_above_the_maze() {
        let mut svg = Svg::new(100.0, 50.0);
        let plain = svg.to_string(false);
        assert!(plain.contains("viewBox='-15 -15 130 80'"), "{plain}");
        assert!(!plain.contains("<text"));

        svg.set_caption(Some("lubra --seed 1 -W 'a b' <x> & y".to_string()));
        let captioned = svg.to_string(false);
        assert!(
            captioned.contains("viewBox='-15 -43 130 108'"),
            "{captioned}"
        );
        assert!(
            captioned.contains(">lubra --seed 1 -W &apos;a b&apos; &lt;x&gt; &amp; y</text>"),
            "{captioned}"
        );
    }
}

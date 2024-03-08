use common::consts::IN;
use common::types::point::Point;
use lib::renders::svg::Svg;

pub fn add_indexes(key: u32, svg: &mut Svg) {
    add_text(svg, "③".to_string(), 1f64, 0f64, key);
    add_text(svg, "⑤".to_string(), 2f64, 0f64, key);
    add_text(svg, "①".to_string(), 3f64, 0f64, key);
    add_text(svg, "④".to_string(), 4f64, 0f64, key);
    add_text(svg, "②".to_string(), 5f64, 0f64, key);

    add_text(svg, "⑤".to_string(), 0.5f64, 0.5f64, key);
    add_text(svg, "⑥".to_string(), 1.5f64, 0.5f64, key);
    add_text(svg, "④".to_string(), 2.5f64, 0.5f64, key);
    add_text(svg, "⑤".to_string(), 3.5f64, 0.5f64, key);
    add_text(svg, "⑥".to_string(), 4.5f64, 0.5f64, key);
    add_text(svg, "④".to_string(), 5.5f64, 0.5f64, key);

    add_text(svg, "①".to_string(), 0f64, 1f64, key);
    add_text(svg, "④".to_string(), 1f64, 1f64, key);
    add_text(svg, "②".to_string(), 2f64, 1f64, key);
    add_text(svg, "⑥".to_string(), 3f64, 1f64, key);
    add_text(svg, "③".to_string(), 4f64, 1f64, key);
    add_text(svg, "⑤".to_string(), 5f64, 1f64, key);
    add_text(svg, "①".to_string(), 6f64, 1f64, key);

    add_text(svg, "⑤".to_string(), 0.5f64, 1.5f64, key);
    add_text(svg, "⑥".to_string(), 1.5f64, 1.5f64, key);
    add_text(svg, "④".to_string(), 2.5f64, 1.5f64, key);
    add_text(svg, "⑤".to_string(), 3.5f64, 1.5f64, key);
    add_text(svg, "⑥".to_string(), 4.5f64, 1.5f64, key);
    add_text(svg, "④".to_string(), 5.5f64, 1.5f64, key);

    add_text(svg, "③".to_string(), 1f64, 2f64, key);
    add_text(svg, "⑤".to_string(), 2f64, 2f64, key);
    add_text(svg, "①".to_string(), 3f64, 2f64, key);
    add_text(svg, "④".to_string(), 4f64, 2f64, key);
    add_text(svg, "②".to_string(), 5f64, 2f64, key);

    add_text(svg, "⑥".to_string(), 1.5f64, 2.5f64, key);
    add_text(svg, "④".to_string(), 2.5f64, 2.5f64, key);
    add_text(svg, "⑤".to_string(), 3.5f64, 2.5f64, key);
    add_text(svg, "⑥".to_string(), 4.5f64, 2.5f64, key);

    add_text(svg, "②".to_string(), 2f64, 3f64, key);
    add_text(svg, "⑥".to_string(), 3f64, 3f64, key);
    add_text(svg, "③".to_string(), 4f64, 3f64, key);
}

fn add_text(svg: &mut Svg, n: String, x: f64, y: f64, key: u32) {
    let invert = key & IN == IN;
    let y = if invert { 3f64 - y } else { y };
    svg.add_fragment(
        &format!("<text y=\"3.7\" dominant-baseline=\"middle\" text-anchor=\"middle\" font-size=\"43px\" fill=\"black\" transform=\"scale(1, -1)\">{n}</text>"),
        Point { x, y },
        0,
    );
}

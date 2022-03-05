use lib::renders::render_svg::RenderSvg;

fn main() {
    let g = RenderSvg::new();
    g.render("test.svg".to_string());
}

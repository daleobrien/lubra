use lib::renders::render_svg::RenderSvg;

fn main() {
    let mut g = RenderSvg::new();
    g.display_basic_tiles();
    let result = g.render("test.svg".to_string());

    println!("{:?}", result.unwrap());
}

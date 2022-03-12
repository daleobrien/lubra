use lib::renders::render_svg::RenderSvg;

fn main() {
    let g = RenderSvg::new();
    let result = g.render("test.svg".to_string());

    println!("{:?}", result.unwrap());
}

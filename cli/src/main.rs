use lib::renders::render_svg::RenderSvg;

fn main() {
    let mut g = RenderSvg::new();
    g.display_one_tile();
    let result_g = g.render("test-one-tile.svg".to_string());

    let mut h = RenderSvg::new();
    h.display_all_tiles();
    let result_h = h.render("test-all.svg".to_string());

    println!("{:?} {:?}", result_h.unwrap(), result_g.unwrap());
}

use lib::renders::render_svg::RenderSvg;

fn main() {
    let mut g = RenderSvg::new();
    g.display_one_tile();
    let result = g.render("test-one-tile.svg".to_string());
    //g.display_all_tiles();
    //let result = g.render("test-all.svg".to_string());

    println!("{:?}", result.unwrap());
}

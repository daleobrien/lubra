use lib::{renders::render_svg::RenderSvg};

fn main() {
    let mut g = RenderSvg::new();
    g.display_one_tile();
    let result_g = g.render("test-one-tile.svg".to_string());

    // println!(
    //     "Finished {:?}",
    //     result_g.unwrap()
    // );
    let mut h = RenderSvg::new();
    h.display_all_tiles();
    let result_h = h.render("test-all.svg".to_string());

    let mut f = RenderSvg::new();
    f.display_random_grid();
    let result_f = f.render("test-random.svg".to_string());

    println!(
        "Finished {:?} {:?} {:?}",
        result_h.unwrap(),
        result_g.unwrap(),
        result_f.unwrap()
    );

    //walk();
}

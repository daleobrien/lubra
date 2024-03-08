use itertools::Itertools;
use std::io::Write;

mod add_nodes;
mod create_svgs;
mod create_tiles;
mod create_typst;
mod draw_walls;

fn main() {
    println!("Loading fragments and generating svg for each fragment");
    create_svgs::create_svgs();
    create_typst::create_typst_doc();
    create_tiles::create_tiles();
}

mod add_nodes;
mod create_svgs;
mod create_tiles;
mod create_typst;
mod draw_walls;
mod variants;

fn main() {
    println!("Loading fragments and generating svg for each fragment");
    create_svgs::create_svgs();
    create_tiles::create_tiles();
    create_typst::create_typst_doc("svgs", "fragments.typ", 4);
    create_typst::create_typst_doc("tiles", "tiles_grid.typ", 8);
}

mod add_nodes;
mod create_svgs;
mod create_tiles;
mod create_typst;
mod draw_walls;
#[rustfmt::skip]
#[allow(clippy::all)]
mod empty;
#[rustfmt::skip]
#[allow(clippy::all)]
mod variants;

fn main() {
    create_svgs::create_svgs();
    create_tiles::create_tiles();
    create_typst::create_typst_doc("svgs", "fragments.typ", 4, "1");
    create_typst::create_typst_doc("tiles", "tiles_grid.typ", 7, "2");
}

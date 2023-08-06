use lib::maze::maze::Maze;

fn main() {
    // let mut g = Maze::new();
    // g.create_one_tile_grid();
    // let result_g = g.render("test-one-tile.svg".to_string());
    //
    // // println!(
    // //     "Finished {:?}",
    // //     result_g.unwrap()
    // // );
    // let mut h = Maze::new();
    // h.create_all_title_in_grid();
    // let result_h = h.render("test-all.svg".to_string());

    // const C: usize = 54; // 9, 54;
    // const R: usize = 36; // 6, 36;

    //let maze = Maze::new(18, 6, true, None);
    let maze = Maze::new(6, 3, true, Some(101));
    //let maze = Maze::new(6, 3, true, None);
    let result_f = maze.render("test-random.svg".to_string());

    println!("Finished creating Random maze {:?}", result_f.unwrap());
}

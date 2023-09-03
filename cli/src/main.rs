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

    // const C: usize = 54; // 9, 54;
    // const R: usize = 36; // 6, 36;

    // for i in 100..110 {
    //     let maze = Maze::new(18, 6, true, Some(i));
    //     let name = format!("random-{}.svg", i);
    //     let result_f = maze.render(name.to_string());
    //     println!("Finished creating Random maze {:?}", result_f.unwrap());
    // }

    // // Great test case here
    // let maze = Maze::new(3, 2, true, Some(102));
    // let result_f = maze.render("test-smoosh.svg".to_string());
    // println!("Finished creating Random maze {:?}", result_f.unwrap());

    // Single tile
    let maze = Maze::new(18, 6, true, Some(2));
    //let maze = Maze::new(1, 2, true, Some(2));
    let result_f = maze.render("test-single.svg".to_string());
    println!("Finished creating Random maze {:?}", result_f.unwrap());
}

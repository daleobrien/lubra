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
    //
    // for i in 1..50 {
    //     let maze = Maze::new(18, 9, true, Some(i));
    //     let name = format!("test/random-{}.svg", i);
    //     let result_f = maze.render(name.to_string());
    //     println!("Finished creating Random maze {:?}", result_f.unwrap());
    // }
    //
    // // Great test case here
    // let maze = Maze::new(3, 2, true, true, Some(102));
    // let result_f = maze.render("test-smoosh.svg".to_string());
    // println!("Finished creating Random maze {:?}", result_f.unwrap());

    //let maze = Maze::new(18, 9, true,true, Some(1));
    let maze = Maze::new(2, 2, true, true, Some(1));
    //let maze = Maze::new(6, 3, true, Some(5)); // ERROR
    //let maze = Maze::new(1, 2, true, Some(5));
    let result_f = maze.render("test-single.svg".to_string());
    println!("Finished creating Random maze {:?}", result_f.unwrap());
}

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
    for i in 1..50 {
        let maze = Maze::new(3, 2, true, true, Some(i));
        let name = format!("test/random-{}.svg", i);
        let result_f = maze.render(name.to_string());
        println!("Finished creating Random maze {:?}", result_f.unwrap());
    }

    //let maze = Maze::new(4, 3, true, true, Some(1));
    let maze = Maze::new(2, 2, true, true, Some(1));
    let result_f = maze.render("test-small.svg".to_string());
    println!("Finished creating Random maze {:?}", result_f.unwrap());

    let maze = Maze::new(54, 36, true, true, Some(1));
    let result_f = maze.render("Maze Version X.svg".to_string());
    println!("Finished creating Random maze {:?}", result_f.unwrap());
}

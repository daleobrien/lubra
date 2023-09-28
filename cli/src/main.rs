use lib::maze::maze::Maze;

fn main() {
    let seed = 13;
    let maze = Maze::new(3, 2, true, true, Some(seed));
    let name = format!("sample-{}.svg", seed);
    let result_f = maze.render(name.to_string());
    println!("Finished creating Random maze {:?}", result_f.unwrap());

    // const C: usize = 54; // 9, 54;
    // const R: usize = 36; // 6, 36;
    //
    // for i in 1..50 {
    //     let maze = Maze::new(3, 2, true, true, Some(i));
    //     let name = format!("test/random-{}.svg", i);
    //     let result_f = maze.render(name.to_string());
    //     println!("Finished creating Random maze {:?}", result_f.unwrap());
    // }
    //
    // let maze = Maze::new(C, R, true, true, Some(1));
    // let result_f = maze.render("Maze Version X.svg".to_string());
    // println!("Finished creating Random maze {:?}", result_f.unwrap());
    //
    // let maze = Maze::new(2, 2, true, true, Some(1));
    // let result_f = maze.render("test-small.svg".to_string());
    // println!("Finished creating Random maze {:?}", result_f.unwrap());
}

use clap::Parser;
use lib::maze::maze::Maze;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // #[arg(short = 'd', long, default_value_t = false)]
    // document: bool,
}

fn main() {
    // TODO: use args ...

    // Large maze
    let maze = Maze::new(36, 19, true, true, Some(1));
    let name = format!("sample-large.svg");
    let _ = maze.render(name.to_string());

    // Lots of small mazes
    for i in 1..50 {
        let maze = Maze::new(5, 2, true, true, Some(i));
        let name = format!("test/random-{}.svg", i);
        let _ = maze.render(name.to_string());
    }

    // One small maze
    // 43,44
    let maze = Maze::new(5, 2, true, true, Some(43));
    //let maze = Maze::new(5, 2, true, true, Some(11));
    let name = format!("sample.svg");
    let _ = maze.render(name.to_string());

    // //
    // for i in 1..50 {
    //     let maze = Maze::new(4, 3, true, true, Some(i));
    //     let name = format!("test/random-{}.svg", i);
    //     let result_f = maze.render(name.to_string());
    //     println!("Finished creating Random maze {:?}", result_f.unwrap());
    // }

    // const C: usize = 54; // 9, 54;
    // const R: usize = 36; // 6, 36;
    // let maze = Maze::new(C, R, true, true, Some(1));
    // let result_f = maze.render("Maze Version X.svg".to_string());
    // println!("Finished creating Random maze {:?}", result_f.unwrap());
    //
    // let maze = Maze::new(2, 2, true, true, Some(1));
    // let result_f = maze.render("test-small.svg".to_string());
    // println!("Finished creating Random maze {:?}", result_f.unwrap());
}

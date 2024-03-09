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

    let seed = 15;
    let maze = Maze::new(8, 8, true, true, Some(seed));
    let name = format!("sample.svg");
    let result_f = maze.render(name.to_string());
    println!("Finished creating Random maze {:?}", result_f.unwrap());

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

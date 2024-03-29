use clap::Parser;

use lib::maze::maze::FileType::SVG;
use lib::maze::maze::Maze;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // #[arg(short = 'd', long, default_value_t = false)]
    // document: bool,
}

fn main() {
    // TODO: use args ...

    // // Large maze
    // let maze = Maze::new(70, 60, true, true, Some(1));
    // let name = format!("sample-large.svg");
    // let _ = maze.render(name.to_string(), SVG);
    //
    // // Lots of small mazes
    // for i in 1..105 {
    //     let maze = Maze::new(5, 4, true, true, Some(i));
    //     let name = format!("test/random-{:02}", i);
    //     let _ = maze.render(name.to_string(), PNG);
    // }

    // One small maze
    let mut maze = Maze::new(5, 4, true, true, Some(3));
    let name = format!("sample-0");
    let _ = maze.render(name.to_string(), SVG);

    // Reduce step
    maze.reduce_nets();
    let name = format!("sample-1");
    let _ = maze.render(name.to_string(), SVG);
}

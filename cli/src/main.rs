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
    // let mut maze = Maze::new(70, 60, true, false, Some(1));
    // let name = format!("1-sample-large");
    // let _ = maze.render(name.to_string(), SVG);
    //
    // maze.reduce_nets(4);
    // let name = format!("2-sample-large-reduced");
    // let _ = maze.render(name.to_string(), SVG);
    //
    // let mut maze = Maze::new(70, 60, true, true, Some(1));
    // let name = format!("3-sample-large-with-grid");
    // let _ = maze.render(name.to_string(), SVG);
    //
    // maze.reduce_nets(4);
    // let name = format!("4-sample-large-with-grid-reduced");
    // let _ = maze.render(name.to_string(), SVG);

    // Lots of small mazes
    // for i in 1..105 {
    //     let maze = Maze::new(5, 4, true, true, Some(i));
    //     let name = format!("test/random-{:02}", i);
    //     let _ = maze.render(name.to_string(), PNG);
    // }

    // One small maze
    let mut maze = Maze::new(5, 4, true, true, Some(2));
    let name = format!("5-sample-normal");
    let _ = maze.render(name.to_string(), SVG);

    maze.remove_cycles();
    let name = format!("6-sample-remove-cycles");
    let _ = maze.render(name.to_string(), SVG);

    // Reduce step
    maze.reduce_nets(4);
    let name = format!("7-sample-reduce");
    let _ = maze.render(name.to_string(), SVG);
}

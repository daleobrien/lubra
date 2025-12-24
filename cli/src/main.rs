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
    // let _ = maze.render(name.to_string(), PNG);
    //
    // maze.reduce_nets(4);
    // let name = format!("2-sample-large-reduced");
    // let _ = maze.render(name.to_string(), PNG);
    //
    // let mut maze = Maze::new(70, 60, true, true, Some(1));
    // let name = format!("3-sample-large-with-grid");
    // let _ = maze.render(name.to_string(), PNG);
    //
    // maze.reduce_nets(4);
    // let name = format!("4-sample-large-with-grid-reduced");
    // let _ = maze.render(name.to_string(), PNG);

    // let start = Instant::now();
    // println!("Generating nets ... ");

    let mut maze = Maze::new(100, 64, true, true, true, Some(0));
    maze.reduce_nets(4);
    let name = format!("100-by-64-sample-normal-00");
    let _ = maze.render(name.to_string(), SVG);

    let mut maze = Maze::new(10, 7, true, true, true, Some(10));
    maze.reduce_nets(10);
    let name = format!("50-by-32-sample-normal-03");
    let _ = maze.render(name.to_string(), SVG);

    // Small test maze with node numbers for easy verification
    // let mut maze = Maze::new(5, 5, false, true, true, Some(0));
    // let name = format!("test-maze-with-node-numbers");
    // let _ = maze.render(name.to_string(), SVG);

    // for i in 1..101 {
    //     let mut maze = Maze::new(3, 3, true, false, Some(i));
    //     maze.reduce_nets(4);
    //     let name = format!("small/{i:03}");
    //     let _ = maze.render(name.to_string(), SVG);
    // }
    //
    // // println!("render took {:?}", start.elapsed());
    // // let start = Instant::now();
    //
    // maze.remove_cycles();
    // let name = format!("6-sample-remove-cycles-03");
    // let _ = maze.render(name.to_string(), SVG);
    //
    // //println!("Reducing nets ... ");
    // // Reduce step
    // maze.reduce_nets(4);
    // // println!("reducing took {:?}", start.elapsed());
    // // let start = Instant::now();
    //
    // let name = format!("7-sample-reduce-03");
    // let _ = maze.render(name.to_string(), SVG);
    // //println!("render took {:?}", start.elapsed());

    // // Lots of small mazes
    // for i in 1..105 {
    //     let mut maze = Maze::new(5, 4, true, true, Some(i));
    //     let name = format!("test/random-{:02}", i);
    //     let _ = maze.render(name.to_string(), PNG);
    // }
}

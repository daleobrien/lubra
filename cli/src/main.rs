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

    let seed = 3;
    // for n in 3..5 {
    //     for m in 3..5 {
    //         let mut maze = Maze::new(n, m, true, true, true, Some(seed));
    //         maze.reduce_nets(0);
    //         maze.remove_cycles();
    //         let name = format!("{n}-by-{m}-sample-normal-{seed}");
    //         let _ = maze.render(name.to_string(), SVG);
    //     }
    // }
  let n = 4;
  let m = 3;
  let mut maze = Maze::new(n, m, true, true, true, Some(seed));
  maze.reduce_nets(0);
  maze.remove_cycles();
  let name = format!("{n}-by-{m}-sample-normal-{seed}");
  let _ = maze.render(name.to_string(), SVG);

    // let seed = 3;
    // let mut maze = Maze::new(10, 7, true, true, true, Some(seed));
    // //  let mut maze = Maze::new(10, 7, true, true, true, None);
    // maze.reduce_nets(10);
    // let name = format!("50-by-32-sample-normal-{seed}.1");
    // let _ = maze.render(name.to_string(), SVG);
    // maze.remove_cycles();
    // let name = format!("50-by-32-sample-normal-{seed}.2");
    // let _ = maze.render(name.to_string(), SVG);

    // for i in 1..101 {
    //     let mut maze = Maze::new(20, 14, true, false, false, Some(i));
    //     maze.reduce_nets(0);
    //     maze.remove_cycles();
    //     let name = format!("small/{i:03}");
    //     let _ = maze.render(name.to_string(), SVG);
    // }
}

use clap::Parser;
use lib::maze::maze::{FileType, Maze};
use std::time::Instant;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Output filename (without extension)
    #[arg(short, long, value_name = "FILE")]
    output: Option<String>,

    /// Width of maze (number of columns)
    #[arg(short = 'W', long, default_value_t = 4)]
    width: usize,

    /// Height of maze (number of rows)
    #[arg(short = 'H', long, default_value_t = 3)]
    height: usize,

    /// Random seed for reproducible generation
    #[arg(short, long, value_name = "SEED")]
    seed: Option<u64>,

    /// Show tile boundaries
    #[arg(short = 'B', long, default_value_t = false)]
    boundaries: bool,

    /// Show node numbers
    #[arg(short = 'N', long, default_value_t = false)]
    show_node_numbers: bool,

    /// Show each tile's nets in blue beside its coordinates, e.g. `12,5`
    #[arg(short = 'T', long, default_value_t = false)]
    show_tile_nets: bool,

    /// Randomize the grid (set false for empty grid)
    #[arg(short = 'R', long, default_value_t = true)]
    randomise: bool,

    /// Output format
    #[arg(short = 'F', long, value_enum, default_value_t = Format::Svg)]
    format: Format,

    /// Draw background mask
    #[arg(short, long, default_value_t = false)]
    background: bool,

    /// Enable debug output
    #[arg(short, long, default_value_t = false)]
    debug: bool,

    /// Number of times to reduce nets (0 to skip)
    #[arg(long, default_value_t = 1)]
    reduce: usize,

    /// Remove cycles from the maze
    #[arg(long, default_value_t = true)]
    remove_cycles: bool,

    /// Remove dead ends up to this many tiles long (0 keeps them all)
    #[arg(long, default_value_t = 2)]
    trim_dead_ends: usize,

    /// Grow tracks into blank areas; new side branches are at least this many
    /// tiles long (0 leaves blank areas alone)
    #[arg(long, default_value_t = 3)]
    fill: usize,

    /// Leave tracks that are drawn touching each other as they are
    #[arg(long, default_value_t = false)]
    keep_touching: bool,
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum Format {
    Svg,
    Png,
}

impl From<Format> for FileType {
    fn from(format: Format) -> Self {
        match format {
            Format::Svg => FileType::SVG,
            Format::Png => FileType::PNG,
        }
    }
}

/// The command line this run was started with, e.g. `lubra --seed 1 -W 20 -H 12`.
/// Arguments containing spaces or quotes are single-quoted, as a shell would need.
fn command_line() -> String {
    let quote = |arg: String| {
        if arg.is_empty() || arg.contains(|c: char| c.is_whitespace() || "'\"$`\\".contains(c)) {
            format!("'{}'", arg.replace('\'', "'\\''"))
        } else {
            arg
        }
    };
    std::iter::once("lubra".to_string())
        .chain(std::env::args().skip(1).map(quote))
        .collect::<Vec<_>>()
        .join(" ")
}

fn main() {
    let args = Args::parse();
    let total_start = Instant::now();

    // Set up debug output if requested
    if args.debug {
        println!("Debug mode enabled");
    }

    let maze_start = Instant::now();
    let mut maze = Maze::new(
        args.width,
        args.height,
        args.randomise,
        args.boundaries,
        args.show_node_numbers,
        args.seed,
    );
    let maze_creation_time = maze_start.elapsed();
    if args.debug {
        println!("Maze creation took: {:?}", maze_creation_time);
    }

    // Apply net reduction if requested
    if args.reduce > 0 {
        let reduce_start = Instant::now();
        maze.reduce_nets(args.reduce);
        let reduce_time = reduce_start.elapsed();
        if args.debug {
            println!("Net reduction took: {:?}", reduce_time);
        }
    }

    // Remove cycles if requested
    if args.remove_cycles {
        let cycles_start = Instant::now();
        maze.remove_cycles(args.debug);
        let cycles_time = cycles_start.elapsed();
        if args.debug {
            println!("Cycle removal took: {:?}", cycles_time);
        }
    }

    // Trim short dead ends if requested
    if args.trim_dead_ends > 0 {
        let trim_start = Instant::now();
        maze.trim_dead_ends(args.trim_dead_ends);
        if args.debug {
            println!("Dead-end trimming took: {:?}", trim_start.elapsed());
        }
    }

    // Prune side branches that are drawn touching another track
    if !args.keep_touching {
        let separate_start = Instant::now();
        let left = maze.separate_touching_tracks();
        if args.debug {
            println!(
                "Separating touching tracks took: {:?} ({left} touching pairs left)",
                separate_start.elapsed()
            );
        }
    }

    // Grow tracks into blank areas if requested
    if args.fill > 0 {
        let fill_start = Instant::now();
        maze.fill_blank_areas(args.fill);
        if args.debug {
            println!("Blank-area filling took: {:?}", fill_start.elapsed());
        }
    }

    // Generate output filename
    let filename = if let Some(output) = args.output {
        output
    } else {
        format!("{}-by-{}-sample-normal", args.width, args.height)
    };

    // Render the maze
    let file_type: FileType = args.format.clone().into();

    if args.debug {
        println!(
            "Rendering maze {}x{} with seed {:?}",
            args.width, args.height, args.seed
        );
        println!(
            "Output: {}.{}",
            filename,
            match args.format {
                Format::Svg => "svg",
                Format::Png => "png",
            }
        );
        println!(
            "... settings: boundaries={}, show_node_numbers={}, show_tile_nets={}, randomise={}, background={}",
            args.boundaries,
            args.show_node_numbers,
            args.show_tile_nets,
            args.randomise,
            args.background
        );
    }

    maze.set_show_tile_nets(args.show_tile_nets);
    // With any debug output on, write the command line above the maze, so a drawing
    // can be reproduced from the image alone.
    if args.debug || args.boundaries || args.show_node_numbers || args.show_tile_nets {
        maze.set_caption(Some(command_line()));
    }
    let render_start = Instant::now();
    let result = maze.render_with_options(filename, file_type, args.background, args.debug);
    let render_time = render_start.elapsed();
    let total_time = total_start.elapsed();

    match result {
        Ok(_) => {
            if args.debug {
                println!("Rendering took: {:?}", render_time);
            }

            println!("Total execution time: {:?}", total_time);
            if args.debug {
                println!("Maze rendered successfully");
            }
        }
        Err(e) => {
            eprintln!("Error rendering maze: {}", e);
            if args.debug {
                println!("Total execution time before error: {:?}", total_time);
            }
            std::process::exit(1);
        }
    }
}

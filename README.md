# Lubra

A Rust generator for **triangle-tile mazes**: winding tracks drawn on a grid of
triangles, with tracks that pass over and under one another. Every maze is a
single tree, so there is exactly one route between any two points, from the
entrance (bottom left) to the exit (top right).

<p align="center">
  <img src="docs/sample-ink-solved.svg" alt="A hand-drawn triangle-tile maze with its solution pencilled in" width="560">
</p>

## Quick start

```bash
cargo build --release
cd cli
cargo run --release -- --seed 1 -W 20 -H 12 -o maze      # writes maze.svg
cargo run --release -- --seed 1 -W 20 -H 12 -S -o maze   # with its solution
cargo run --release -- --seed 1 -W 20 -H 12 -P -o maze   # plain, even walls
cargo run --release -- -W 20 -H 12 -F png -o maze       # a new maze, as PNG
```

`-W` and `-H` are in columns and rows of tiles; about 1.7 columns per row looks
square. The same `--seed` always gives the same maze. Run with `--help` to see
every option. The Makefile wraps the common tasks: run `make` to list them.

## Hand-drawn ink

Mazes are drawn by hand, in brown ink, by default: every wall wobbles slightly
and the pen's width wanders, after the ink style of
[maze-rs](https://github.com/daleobrien/maze-rs). The wobble is seeded from the
maze's seed, so a maze always comes out the same. Add `-P` (`--plain`) for
even black walls.

`-S` (`--solution`) adds the route from the entrance to the exit: pencilled in
red over the ink, gone over twice, and lightly where it passes under another
track; with `-P` it is a plain red line. The sample above was made with
`make sample`, which also writes the maze without its solution.

No two tiles look alike in ink, so tiles can't be reused the way plain ones
are: expect files several times bigger (`lib/src/renders/ink.rs`).

## Track width

The tracks are `PATH_WIDTH` (in `common/src/types/point.rs`) wide on each side
of their middle, as a fraction of a tile: 0.09 by default, and anything up to
0.15 works. After changing it, run `make width`: it regenerates the fragments
and re-measures how tracks meet, which decides where they cross and keeps them
apart. Wider tracks touch more often.

## Debugging

`make cli` draws a fixed maze with the debugging overlay: tile outlines (`-B`),
node numbers (`-N`), each tile's nets (`-T`) and rings round any kinks in the
tracks (`-K`), with each net in its own colour and the command line written
above the maze. `--kink-stats N` counts the kinks over `N` seeded mazes.

## How it works

The workspace has four crates:

- `lib`: generates, reduces and renders the maze.
- `cli`: the command line.
- `generate`: writes the SVG fragment for every tile shape, and the lookup
  tables, into `lib`.
- `document`: draws every fragment for the design notes in `typst/`.

The design notes (`typst/main.typ`, `make doc`) explain the tiles, the passes
that turn random tiles into one maze, and how each fragment is drawn.
`WARP.md` is a shorter guide to the code.

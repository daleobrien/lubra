# Debugging

[Back to the README](../README.md)

`make cli` draws a fixed maze with the debugging overlay:

- tile outlines (`-B`),
- node numbers (`-N`),
- each tile's nets (`-T`),
- rings round any kinks in the tracks (`-K`: red for a corner, purple for a
  tight wiggle),

with each net in its own colour and the command line written above the maze.
`CLI_ARGS` in the Makefile holds those arguments; change the seed there, or on
the command line, to see another maze.

`make maze` builds the command line and times one larger maze with the overlay
(`MAZE_ARGS`).

`--kink-stats N` builds `N` seeded mazes and lists which fragments their kinks
fall in, most common first:

```bash
make cli CLI_ARGS="--seed 1 -W 20 -H 12 --kink-stats 50"
```

# Settings: track width and crossings

[Back to the README](../README.md)

Two constants shape every maze. Change one, then rebuild.

## Track width

The tracks are `PATH_WIDTH` (in `common/src/types/point.rs`) wide on each side
of their middle, as a fraction of a tile: 0.12 by default, and anything from
0.09 to 0.15 works. After changing it, run `make width`: it regenerates the
fragments and re-measures how tracks meet, which decides where they cross and
keeps them apart. Wider tracks touch more often.

`make width` needs `rsvg-convert` and Python with numpy, Pillow and OpenCV.

## Crossings

How often one track passes under another is `CROSSING_LEVEL` (in
`lib/src/maze/maze.rs`, or `Maze::set_crossing_level`): 4 by default.

Tidying a maze takes out tracks drawn against each other, and with them most
crossings, as a crossing's tracks often touch a neighbour. From 0 to 1 the level
is the share of crossings kept: tidying takes out the other track instead, and
gives up a kept crossing only when nothing else clears the touch. Above 1 every
crossing is kept and tiles with crossing tracks are also picked that many times
as often.

Over 20 mazes of 24 by 12 tiles:

| Level | Crossings per 100 tiles | Tracks left touching (over the 20) |
| ----- | ----------------------- | ---------------------------------- |
| 0     | 0.3                     | 137                                |
| 0.5   | 2                       | 123                                |
| 1     | 4                       | 105                                |
| 2     | 7                       | 79                                 |
| 4     | 10                      | 48                                 |
| 10    | 13.5                    | 33                                 |

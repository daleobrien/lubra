#import "template.typ": *
#import "helper.typ": *

#import "@preview/algo:0.3.6": algo, i, d, comment, code
#import "@preview/cetz:0.4.2"

#show: project.with(title: "Lubra")
#set heading(numbering: "1.1")

// ---------------------------------------------------------------------------
// Geometry helpers for the diagrams. They mirror the Rust code:
//   - tile size/shape:      common/src/types/point.rs
//   - node positions/labels: lib/src/renders/node_numbers.rs
//   - neighbour offsets:     common/src/consts.rs (K0..K5, NODE_TO_COORD)
// Units: one tile is 1 wide and sqrt(3)/2 high; tile (i, j) starts at x = i/2.
// ---------------------------------------------------------------------------

#let H = calc.sqrt(3) / 2

// Physical node positions inside an upright tile, in node order 1..6.
#let local-nodes = ((0.5, H), (0, 0), (1, 0), (0.25, H / 2), (0.75, H / 2), (0.5, 0))

#let is-up(i, j) = calc.rem(i + j, 2) == 0

#let tile-nodes(x0, y0, up) = local-nodes.map(p => {
  let y = if up { p.at(1) } else { H - p.at(1) }
  (x0 + p.at(0), y0 + y)
})

#let node-pos(i, j) = tile-nodes(i / 2, j * H, is-up(i, j))

#let centroid(i, j) = {
  let y = if is-up(i, j) { H / 3 } else { 2 * H / 3 }
  (i / 2 + 0.5, j * H + y)
}

// Logical node number printed at each physical node position, by column i mod 3
// (NUMBERS in node_numbers.rs).
#let node-labels = (
  ("1", "2", "3", "4", "5", "6"),
  ("3", "1", "2", "5", "6", "4"),
  ("2", "3", "1", "6", "4", "5"),
)

#let K0 = ((0, -1), (-2, 1), (2, 1), (-1, 0), (1, 0), (0, 1), (-1, -1), (1, -1), (-2, 0), (-1, 1), (2, 0), (1, 1))
#let K1 = ((-2, -1), (2, -1), (0, 1), (0, -1), (-1, 0), (1, 0), (-1, -1), (-2, 0), (2, 0), (1, -1), (1, 1), (-1, 1))
#let K2 = ((2, 1), (0, -1), (-2, 1), (1, 0), (0, 1), (-1, 0), (2, 0), (1, 1), (-1, -1), (1, -1), (-2, 0), (-1, 1))
#let K3 = ((0, 1), (-2, -1), (2, -1), (-1, 0), (1, 0), (0, -1), (1, 1), (-1, 1), (-1, -1), (-2, 0), (2, 0), (1, -1))
#let K4 = ((-2, 1), (2, 1), (0, -1), (0, 1), (-1, 0), (1, 0), (-2, 0), (-1, 1), (2, 0), (1, 1), (-1, -1), (1, -1))
#let K5 = ((2, -1), (0, 1), (-2, -1), (1, 0), (0, -1), (-1, 0), (2, 0), (1, -1), (1, 1), (-1, 1), (-1, -1), (-2, 0))
#let node-to-coord = (K3, K4, K5, K0, K1, K2)
#let neighbour-names = ("1", "2", "3", "4", "5", "6", "1L", "1R", "2L", "2R", "3L", "3R")

#let draw-nodes(pts, labels) = {
  import cetz.draw: *
  for (k, p) in pts.enumerate() {
    circle(p, radius: 0.085, fill: white, stroke: 0.5pt + black)
    content(p, text(size: 7pt, labels.at(k)))
  }
}

#let draw-triangle(pts, ..style) = {
  import cetz.draw: *
  line(pts.at(0), pts.at(1), pts.at(2), close: true, ..style)
}

= Overview

Lubra generates mazes on a grid of triangular tiles and renders them as SVG
(or PNG). Every tile has six _nodes_ and holds zero or more _nets_: sets of
nodes joined by a path drawn inside the tile. Nodes are shared with
neighbouring tiles, so nets on adjacent tiles join up into long winding
corridors. The maze is built by placing random tiles that agree with their
neighbours, optionally merging the resulting nets, and then drawing each net
from a table of pre-generated SVG path fragments.

The project is a Cargo workspace:

#table(
  columns: (auto, 1fr),
  table.header([*Crate*], [*Purpose*]),
  [`common`], [Shared types and constants: bit flags (`consts.rs`), tile orientation (`types/bits.rs`), coordinates, points and paths, `NodeCombinations`, and the `define_shape_family!` macro.],
  [`lib`], [The maze itself (`maze/`, with the `Maze` methods split across `maze/maze/`) and the renderer (`renders/`). Generated tables live in `maze/generated/` and `renders/generated/`.],
  [`generate`], [One binary that writes the generated tables: `generate svg`, `generate nets`, or both when run with no argument.],
  [`cli`], [The `lubra` binary: generates, reduces, de-cycles and renders one maze (see @cli).],
  [`document`], [Produces the SVGs and the generated Typst files used by this document.],
)

Everything is driven from the `Makefile` at the workspace root (run `make`
to list the targets). Each step runs from its crate's directory, so output
files land where the code expects them. Pipelines run their steps in order:

#table(
  columns: (auto, 1fr),
  table.header([*Target*], [*Runs*]),
  [`make generate`], [`gen` (SVG fragments and net tables), `cli`, then `document`.],
  [`make both`], [`gen-svg`, then `cli`.],
  [`make doc`], [`gen-svg`, `document`, then `typst` (this document and the catalogues).],
  [`make full`], [`gen-nets`, `gen-svg`, `cli`, then `document`.],
  [`make maze`], [Builds the CLI and times one run with `MAZE_ARGS` (default `--seed 3 -W 40 -H 30 -B -N --debug`), e.g. `make maze MAZE_ARGS="--seed 7 -W 20 -H 20"`.],
)

Single steps:

#table(
  columns: (auto, 1fr),
  table.header([*Target*], [*Runs*]),
  [`make gen`], [`generate` with no argument: SVG fragments and net tables.],
  [`make gen-svg`], [`generate svg`: fragments, `ADJ_CHECKS` and the `document` helper tables.],
  [`make gen-nets`], [`generate nets`: the small and large net/mass lookup tables.],
  [`make cli`], [The CLI with `CLI_ARGS` (default `--seed 1 -W 20 -H 12 -B -N -T -K`: the same square-looking maze every run, with tile boundaries, node numbers, tile nets and kink rings).],
  [`make sample`], [The maze shown in the README (`SAMPLE_ARGS`), written to `docs/sample-ink.svg`.],
  [`make document`], [The `document` crate (SVGs and generated Typst files).],
  [`make typst`], [`typst compile` of `shapes.typ`, `tiles.typ` and `main.typ`.],
  [`make build`], [`cargo build --workspace`.],
  [`make test`], [`cargo test --workspace`.],
)

`CARGO` and `TYPST` can be overridden, e.g. `make doc TYPST=/opt/typst`.

= The tile

A tile is an equilateral triangle. Nodes 1, 2 and 3 are its vertices and
nodes 4, 5 and 6 are the midpoints of its edges: 4 lies between 1 and 2,
5 between 1 and 3, and 6 between 2 and 3. Tiles alternate between _normal_
and _inverted_ (flipped vertically, flag `IN`).

#figure(
  cetz.canvas(length: 3cm, {
    import cetz.draw: *
    let up = tile-nodes(0, 0, true)
    let down = tile-nodes(1.6, 0, false)
    draw-triangle(up)
    draw-triangle(down)
    draw-nodes(up, ("1", "2", "3", "4", "5", "6"))
    draw-nodes(down, ("1", "2", "3", "4", "5", "6"))
    content((0.5, -0.3), [normal])
    content((2.1, -0.3), [inverted (`IN`)])
  }),
  caption: [Node numbering of a tile as it is drawn.],
)

`TILE_WIDTH` is 120 SVG units, `TILE_HEIGHT` is $120 dot sqrt(3) / 2$, and
tracks are `PATH_WIDTH` $= 0.09$ tile widths wide. Some arcs (for example
the vertex arc 2–3) bulge past their tile's edge. At this width they stay
clear of the neighbouring tile's opening, so a track that passes over another
never leaves a seam where it crosses a tile boundary (see @rendering).

== Grid and orientation

Tile $(i, j)$ is in column $i$ and row $j$. Its origin is at
$x = i dot W / 2$, $y = j dot H$, so neighbouring triangles in a row overlap
by half a width. Each tile has one of six orientations:

- rotation $r = i mod 3$ (none, or one of the two 120° rotations);
- inverted when $i + j$ is odd.

Inside the maze, nodes are numbered _logically_: a node shared by
several tiles has the same number in all of them. This makes building
nets across tiles simple, but it means a tile's logical node 1 is
not necessarily its top vertex. Before drawing, `transform_tile`
(`common/src/types/bits.rs`) rotates the bits and sets `IN` where needed to
convert from logical to drawn numbering:

#table(
  columns: 5,
  align: center,
  table.header([*Tile*], [*$r$*], [*Inverted*], [*Logical `N1` →*], [*Logical `N4` →*]),
  [(0, 0)], [0], [no], [`N1`], [`N4`],
  [(1, 0)], [1], [yes], [`IN|N2`], [`IN|N6`],
  [(2, 0)], [2], [no], [`N3`], [`N5`],
  [(0, 1)], [0], [yes], [`IN|N1`], [`IN|N4`],
  [(1, 1)], [1], [no], [`N2`], [`N6`],
  [(2, 1)], [2], [yes], [`IN|N3`], [`IN|N5`],
)

The pattern repeats every three columns. The diagram below shows the logical
numbers on part of the grid, the same labels the CLI draws when
`show_node_numbers` is set:

#figure(
  cetz.canvas(length: 2.3cm, {
    import cetz.draw: *
    for j in range(2) {
      for col in range(6) {
        draw-triangle(node-pos(col, j), stroke: 0.6pt + gray)
      }
    }
    for j in range(2) {
      for col in range(6) {
        draw-nodes(node-pos(col, j), node-labels.at(calc.rem(col, 3)))
        content(centroid(col, j), text(size: 6pt, fill: gray, "(" + str(col) + "," + str(j) + ")"))
      }
    }
  }),
  caption: [Logical node numbers. Shared nodes have the same number in every tile that touches them.],
)

== Surrounding tiles

Each tile has 12 relevant neighbours:

- three _edge_ neighbours, across nodes 4, 5 and 6;
- three _opposite_ neighbours, which touch only at a vertex (1, 2 or 3),
  directly across from it;
- six _flanking_ neighbours (`1L`, `1R`, `2L`, `2R`, `3L`, `3R`), which
  also touch a vertex but sit beside it.

A path through a vertex always continues into the opposite tile. An edge
node always continues into the edge neighbour. `NODE_TO_COORD[k]` gives the
$(Delta i, Delta j)$ of all 12 neighbours, in the order 1–6, 1L, 1R, 2L, 2R,
3L, 3R, with $k = (i + 3 (j mod 2)) mod 6$. Because nodes are numbered
logically, entry $n$ is always the tile across logical node $n$, whatever
the tile's orientation.

#figure(
  {
    let ci = 4
    let cj = 2
    cetz.canvas(length: 2.1cm, {
      import cetz.draw: *
      let k = calc.rem(ci + 3 * calc.rem(cj, 2), 6)
      let neighbours = node-to-coord.at(k).map(o => (ci + o.at(0), cj + o.at(1)))
      for j in range(cj - 1, cj + 2) {
        for col in range(ci - 3, ci + 4) {
          let shade = if (col, j) == (ci, cj) {
            luma(210)
          } else if neighbours.contains((col, j)) {
            rgb("#dbe9f6")
          } else {
            none
          }
          draw-triangle(node-pos(col, j), fill: shade, stroke: 0.6pt + gray)
        }
      }
      for (n, c) in neighbours.enumerate() {
        content(centroid(c.at(0), c.at(1)), text(size: 8pt, weight: "bold", neighbour-names.at(n)))
      }
      draw-nodes(node-pos(ci, cj), node-labels.at(calc.rem(ci, 3)))
    })
  },
  caption: [The 12 surrounding tiles of tile (4, 2), computed from `NODE_TO_COORD`. Index constants are `ADJ_N1` … `ADJ_N3R` in `lib/src/maze/adjacent.rs`.],
)

= Bit encoding

A net is a `usize` bit mask (`common/src/consts.rs`):

#table(
  columns: (auto, auto, 1fr),
  table.header([*Bits*], [*Constants*], [*Meaning*]),
  [0–5], [`N1`–`N6`], [Nodes of the net in this tile. Mask `NL`.],
  [6–11], [`A1`–`A6`], [First neighbouring net that is drawn as part of this fragment (see @adjacent).],
  [12–17], [`B1`–`B6`], [Second neighbouring net.],
  [18–29], [], [Room for two more neighbouring nets. `NX` masks bits 0–29.],
  [30], [`IN`], [Tile is drawn inverted.],
  [31], [`WA`], [Wall (tile outline) fragment.],
)

The neighbouring-net groups are kept in canonical order by
`sort_upper_bits`, which sorts the 6-bit groups above bit 5 by their lowest
set node. A `Tile` (`lib/src/maze/tile.rs`) holds a list of nets that
must not share nodes; `bits` is their union.

Fragment and SVG names follow the bits: a prefix `n` (normal) or `i`
(inverted), the nodes, then `-a` and `-b` for the neighbouring nets. For
example, `n14-a34` is the net 1–4 drawn together with the net 3–4 of the
tile across node 4.

= Generating a maze

`Maze::new(columns, rows, randomise, boundaries, show_node_numbers, seed)`
creates an empty grid. If `randomise` is set it calls `create_random_grid`
(`maze/maze/grid_generation.rs`):

#algo(
  title: "CreateRandomGrid",
  parameters: ("columns", "rows", "seed"),
)[
  place a dead end (`N5`) at $(0, 0)$ as the start\
  place a dead end (`N5`, `N6` or `N4`, by column count) at the last tile as the end\
  order $<-$ every tile $(i, j)$, shuffled with ChaCha8(seed)\
  for each unplaced $(i, j)$ in order:#i\
    $S <-$ the 12 surrounding tiles of $(i, j)$\
    mask $<-$ `get_surrounding_tile_connections`($S$)\
    nets $<-$ `random_tile`(mask, seed, small table)\
    if mask $!= 0$ and nets is empty:#i\
      nets $<-$ `random_tile`(mask, seed, large table)#d\
    place nets at $(i, j)$\
    seed $<-$ seed $+ 1$#d\
  build the net list from the tiles (`rebuild_nets`)
]

The tiles are visited in random order so that no direction is favoured.
With `seed = None`, the generator is seeded from the thread RNG instead.

== Constraint mask

`get_surrounding_tile_connections` (`lib/src/maze/adjacent.rs`) encodes one
decimal digit per node, with node $n$ at $10^(n - 1)$:

#table(
  columns: (auto, 1fr),
  table.header([*Digit*], [*Meaning*]),
  [0], [The node must not be used: it is outside the grid, the neighbour has already been placed without it, or (for a vertex) a flanking tile already uses it.],
  [1], [The node must be used, because the neighbour (edge or opposite tile) already connects to it.],
  [2], [Free: the neighbour has not been placed yet.],
)

For example, mask `211` means nodes 1 and 2 must be connected, node 3 is
free, and nodes 4–6 are closed.

== Lookup tables and weighting

`generate_nets` precomputes, for every mask, all valid sets of nets and
their weights ("mass"). It writes four functions:

- `nets_small.rs` / `mass_small.rs`: a reduced set of net shapes with no
  dead ends. It uses edge-to-edge lines (4–5, 4–6, 5–6), vertex-to-edge
  lines, vertex-to-vertex lines and all twenty 3-way junctions.
- `nets_large.rs` / `mass_large.rs`: every combination of 1- to 6-node
  nets, including single-node dead ends. Used only when the small table has
  no option for a constrained tile.

Each is a `match` on the mask that returns `&'static` slices of constant data,
so a lookup allocates nothing and needs only a small stack frame.

`NodeCombinations` lists every set of non-overlapping nets. Each set is
registered under every mask it satisfies, with each used node marked as
either 1 or 2. Its mass (`get_mass` in `common/src/bit_utils.rs`) is

$ "mass" = n^2 dot product_("net") w("net"), quad n = |"nets"|, quad w = cases(4 & "3 nodes", 3 & "4 nodes", 2 & "otherwise") $

Squaring $n$ favours tiles that hold several separate nets, which gives more
tracks crossing over each other. The higher weights for 3- and 4-node nets
give more branching. An empty tile (mass 0) is chosen only when it is the
only option. To change the look of the mazes, adjust `get_mass` or the shape
list in `generate/src/main.rs`, then run `make gen-nets`. Each mass list is in the same order
as its list of net sets (entry $n$ weighs net set $n$); a test checks every
mask against `get_mass`. `random_tile` (`maze/random.rs`) picks
an entry with a Walker alias table (`weighted_rand`) and shuffles the nets to
vary the render order.

== Tracking and reducing nets

The maze-wide `Net` is a list of `Nodes`, each a set of `Node { i, j, node }`
positions (`maze/net.rs`). It is only an index of what the tiles already say,
so it is never edited directly: `rebuild_nets` (`maze/maze/net_operations.rs`)
recomputes it from the tiles once all tiles are placed, and again after every
later change. Two
node positions are in the same net when they share a tile net, or when they
are the same node seen from the tiles either side of it. The result is
ordered by the first tile that uses each net, so it is deterministic.

`reduce_nets(lower_limit)` (`maze/maze/net_reduction.rs`) joins the nets
into one:

+ rebuild the net list and remove every net with at most `lower_limit`
  node positions from the tiles (`remove_short_nets`; 0 keeps them all);
+ repeat passes until one net is left, or a pass joins nothing. Each pass
  uses the first of these that joins anything:
  - *shared tile*: merge two tile nets of different nets that pass through
    the same tile into one tile net;
  - *free node*: open a node that is unused on both sides between
    neighbouring tiles of different nets, and add it to a tile net on each
    side. A vertex is only opened if none of its flanking tiles uses it;
  - *empty tile*: give an empty tile a two-node net joining tile nets of
    different nets on two of its sides.
+ rebuild the net list after every pass.

Within a pass a union-find over the nets allows each pair of nets to be
joined only once, so joins never create a loop. Candidates are shuffled with
the maze's seed, so a seeded maze always reduces the same way.

== Removing cycles

`remove_cycles(debug)` (`maze/maze/remove_cycles.rs`) turns the maze into a
tree, with exactly one route between any two points, without splitting it.
It treats the maze as a graph with two kinds of vertex, node positions and
tile nets, and two kinds of join:

- a _crossing_ join links a node position to the same node in the tile
  across it. Each position has at most one partner, so these joins alone can
  never form a loop;
- a _tile_ join links a tile net to each of its node positions. Treating the
  tile net as a hub, rather than joining its nodes to each other, means a
  junction is not mistaken for a loop.

The joins are added to a union-find in Kruskal order: all crossing joins
first, then the tile joins shuffled with the maze's seed. A tile join whose
two ends are already connected would close a loop, so it is cut instead.
That node is detached from its tile net and kept as a single-node net, which
is drawn as a dead end. Cutting a join that closes a loop never disconnects
anything, so the number of nets doesn't change.

== Trimming dead ends

Because the maze ends up as a tree, every junction needs a branch that stops
somewhere, so the number of dead ends always tracks the number of junctions.
Many of those branches are only a tile or two long, because a loop is often
cut right beside a junction. They are drawn as stubs: a track that stops just
past a node, or a hook curled into a capped finger.

`trim_dead_ends(max_length)` (`maze/maze/dead_ends.rs`) removes them. A dead
end is a chain of tile nets from a leaf (joined to one other tile net) through
plain two-way tile nets to a junction (joined to three or more). Chains of at
most `max_length` tile nets are deleted and the junction gives up that branch.

- All chains are measured before any are removed, so a junction that loses
  its short branches is not then trimmed away itself.
- Chains that include the entrance tile $(0, 0)$ or the exit tile are kept.

This is a trade-off, not a free improvement: trimming removes branching. Over
ten 30 × 20 mazes, junctions per 100 tiles go from 35 untrimmed to 22 at
`max_length` 1 and 15 at 2.

== Separating touching tracks

Tile placement only checks that nodes line up, so two tracks that are not
joined can be drawn with their walls against each other, on one tile or
across neighbouring tiles. `separate_touching_tracks`
(`maze/maze/touching.rs`) removes those touches. It runs after trimming and
before filling.

How two tile nets meet is a lookup in `maze/maze/fill_clearance.rs`, for a
tile net on a tile and another on the same tile or one of the twelve around it:

- _clear_: at least 2 units of white space between the drawings;
- _crossing_: one drawing cuts the other in two, so one track passes over the
  other. Crossings are kept;
- _touching_: anything else.

The tables are measured from the rendered fragments by `tools/clearance.py`
(the steps are in the script) and must be measured again when fragment
geometry changes. Each tile net is measured alone, so a track drawn as a
cross-tile fragment (see @adjacent) can sit slightly differently from its
measurement.

Each round of the pass:

+ takes one tile net out of every touching pair, the one with fewer tracks
  joined to it, and has its neighbours give up the shared nodes;
+ joins the pieces that were beyond it back on with the joins of `reduce_nets`,
  allowing only joins whose reshaped tile nets touch nothing
  (`rejoin_nets_tidily`).

Tile nets on the route from the entrance to the exit are never removed, so a
pair with both tile nets on the route stays touching. After the last round,
pieces that could not be joined back on are removed. The pass returns the
number of touching pairs left; over eight 20 × 12 mazes it took about 100
touching pairs down to between 2 and 9.

== Filling blank areas

After reduction, cycle removal and trimming, parts of the grid are left with
no track through them. `fill_blank_areas(min_length)` (`maze/maze/fill.rs`)
grows new dead-end branches into that space.

- A branch starts at a tile net with one or two nodes and wanders from tile to
  tile through free nodes, for up to six tiles. Later branches can fork off
  earlier ones.
- Existing dead ends are extended first. A branch off a two-node tile net makes
  a new junction, so it is only kept if it reaches `min_length` tiles; shorter
  ones are undone, which keeps stubs out.
- A branch only enters a tile that holds at most two tile nets, and only where
  its track is drawn clear of every track it is not joined to, on the same
  tile and the twelve around it. So filling adds no touching tracks.
- About one step in three, a plain two-node track may instead pass over one
  other plain track, so filled areas get crossings too. A branch never ends on
  a crossing.
- When nothing more grows, each dead end that is stuck looks for the shortest
  route to an empty tile, of at most `MAX_REACH` (6) tiles, passing other plain
  tracks at most `MAX_CROSSINGS` (3) times (`reach_gaps`). At each crossing it
  goes over or under at random: a tile's nets are drawn in order, so putting
  the new one first sends it under. Growing and reaching alternate until
  neither adds anything.
- A branch is attached at one end only, so the maze stays a single loop-free
  tree.

Reaching adds little: over 40 seeded 20 × 12 mazes it filled 29 of 343 empty
tiles. After growing, most dead ends cannot move because the track they would
add touches another, and 87% of the empty tiles sit beside a plain track that
could only reach them with a side branch shorter than `min_length`. The
side-branch length is what decides how full the maze is. The default is
`--fill 2`, which halves the blank pockets inside the maze compared with 3;
`--fill 1` removes nine in ten, at the cost of one-tile stubs.

The "drawn clear" test uses the tables described under _Separating touching
tracks_.

= Rendering <rendering>

`Maze::render_with_options(file_name, file_type, background, debug)`
(`maze/maze/render.rs`) produces an `Svg` (`lib/src/renders/svg.rs`) and saves
it as SVG or, via `resvg`, as PNG. `render` is the same with no background
and no debug output.


+ For each tile and each of its nets, `generate_hidden_and_visible_nets`
  decides which fragment to draw (see @adjacent). The result is a
  `CataloguedNet` holding _visible_ and _hidden_ coordinate/net pairs.
+ Every visible net that is not also hidden is converted to drawn
  orientation with `transform_tile` and looked up in `MAP_PATHS`
  (`lib/src/renders/generated/fragment.rs`). Missing or empty fragments are skipped.
+ With `boundaries`, each tile also gets its `WA` (outline) fragment. With
  `show_node_numbers`, `add_node_numbers` labels every node and tile. With
  `background`, the SVG gets a light-blue background.

Each distinct fragment is stored once in a `StringRegistry` and emitted as
a `<g>` in `<defs>`, in the order fragments were first used, so a seeded
maze always produces a byte-identical file. Every placement is a `<use href>` positioned at
$(i dot W / 2, j dot H)$. The y axis is flipped so that row 0 is at the bottom.

Tracks may pass over one another. Each fragment is a white fill (80% opaque)
followed by its black walls, so the track drawn later passes over and the one
beneath shows through faintly as grey. All fragments go on one layer, in tile
order. This only looks right because `PATH_WIDTH` keeps every crossing inside
a single tile; at 0.11 the over/under order could flip at a tile boundary and
leave a black seam across a corridor.

== Entrance and exit markers

The entrance (the first tile) and the exit (the last tile) each hold a single
dead end at an edge. `is_terminal(i, j)` identifies them, and `reduce_nets` and
`fill_blank_areas` never extend their track or route another through them.

They are drawn as markers instead of the plain dead end:
`terminal_fragment` (`lib/src/renders/terminal.rs`) builds the fragment at
render time. The track runs from the edge towards the tile's centre, each wall
leaves it on a small blend curve and joins a ring around the centre, and a red
dot sits in the middle of the ring.

== Adjacent (complex) fragments <adjacent>

Some nets end on an edge node and continue in the neighbouring tile. Drawing
each half separately would leave a visible seam, especially with arcs. So
for those nets, `find_best_path` checks `ADJ_CHECKS`
(`lib/src/maze/generated/nets_adjacent.rs`), which lists, for a net, the nets a given
neighbour may hold. If the neighbour contains a matching net:

- the two nets are combined into one key: this tile's nodes in bits 0–5 and
  the neighbour's net in `A`/`B`;
- if `MAP_PATHS` has that key, the combined fragment is drawn here and the
  neighbour's net is added to the hidden set.

`generate_svg` also registers the mirrored key (the neighbour's point of
view) as an empty fragment, so that it is never drawn twice. The complete
fragment and tile catalogues are in `shapes.pdf` and `tiles.pdf`. The
families involving the arcs 4–5, 4–6 and 5–6 are shown at the end of this
document.

= Generated code

#figure(
  table(
    columns: (auto, auto, 1fr),
    align: (left + horizon, left + horizon, left + horizon),
    stroke: none,
    inset: (x: 6pt, y: 5pt),
    table.hline(stroke: 0.8pt),
    table.header([*Target*], [*File*], [*Contents*]),
    table.hline(stroke: 0.5pt),
    table.cell(rowspan: 2)[`make gen-nets`],
    [`maze/generated/` \ `nets_small.rs` \ `nets_large.rs`],
    [For each constraint mask, the candidate sets of nets.],
    [`maze/generated/` \ `mass_small.rs` \ `mass_large.rs`],
    [The weight (mass) of each candidate.],
    table.hline(stroke: 0.4pt + luma(170)),
    table.cell(rowspan: 3)[`make gen-svg`],
    [`renders/generated/` \ `fragment.rs`],
    [`MAP_PATHS`: perfect-hash map from net bits to SVG fragment, including the empty (mirrored) and wall entries.],
    [`maze/generated/` \ `nets_adjacent.rs`],
    [`ADJ_CHECKS`: neighbouring nets that can be merged into a complex fragment.],
    [`document/src/` \ `variants.rs` \ `empty.rs`],
    [Fragment variants per node set, and the empty keys, used by the `document` crate.],
    table.hline(stroke: 0.8pt),
  ),
  caption: [Files written by the generator. Paths are under `lib/src/` unless shown otherwise.],
)

`make gen` runs both. The table generators are in `generate/src/generators/`
(`lookups.rs`, `svg_fragments.rs`, `path_adj.rs`).

All generated files are checked in. Regenerate them only after changing the
tile logic or the fragment geometry.

== Fragment geometry

Each `generate/src/fragments/frag_*.rs` file describes one shape, in tile-width units,
as a list of points plus a function that turns them into SVG path data. Most
of these files now use `define_shape_family!` (`common/src/macros/`), which
derives all six rotations and reflections from one base shape:

```rust
define_shape_family! {
    base: base,
    svg: svg,
    offset_rotations: &[0, 1, 2, /* ... */],
    variants: {
        n1345: false, true => identity,
        n1246: false, true => rotate_ccw,
        n2356: false, true => rotate_cw,
        n1245: false, false => reflect,
        n2346: false, false => reflect_then_rotate_ccw,
        n1356: false, false => reflect_then_rotate_cw,
    }
}
```

The two booleans flip `invert` for the base points and for the SVG (arc sweep
direction). `offset_rotations` controls how each point is rotated, so arc
centres can be treated differently from end points. A few shapes
(`frag_123`, `frag_13456`, `frag_arcs`, `frag_arc_long`,
`frag_complex_arcs`) are still written out by hand. `frag_wall` draws the tile
outline.

=== The S-curve

Six cross-tile combinations (`45/14/35`, `45/15/24`, `46/14/26`, `46/24/36`,
`56/15/36`, `56/26/35`, in normal and inverted tiles) carry a track from one
vertex of a tile to another. They used to be a tight loop glued to a long arc
with a short straight between them, and in six catalogue tiles the loop touched
the vertex-to-edge track beside it. `frag_s_curve` now draws them as one S: a
tight curve, then a wide curve the other way, tangent where they meet.

Both ends keep the position and direction of the old pieces, so the
neighbouring tiles are unchanged. That fixes everything but one radius:
`S_CURVE_TIGHT_RADIUS` sets the tight curve (to the middle of the track, as a
fraction of the tile width), and the wide curve follows from it. A larger tight
curve makes the wide one smaller and brings the track closer to its neighbour:

#table(
  columns: 3,
  table.header([Tight curve], [Wide curve], [White gap]),
  [22], [64], [15],
  [24], [60], [12],
  [*26* (default)], [*56*], [*9*],
  [28], [52], [6],
  [30], [48], [3],
)

Radii and gaps are in drawing units (a tile edge is 120); the gap is to the
vertex-to-edge track on the same tile. Above about 30 they touch. After
changing it, run `make gen-svg`.

`frag_15_35` (`15/35`, `24/14`, `36/26`) uses the same construction for a track
from one vertex to the next one round, crossing into the neighbouring tile
through the middle of the edge between them. It used to be a double S of four
curves. Both its ends point the same way, so the only S that crosses at the
edge's midpoint is two equal curves of half a tile width (radius 60); that
radius is fixed by the geometry, not a setting.

=== The two-vertex junction

`frag_135` (`135`, `124`, `236`) joins two vertices of a tile and drops to the
middle of the edge between them. It used to be two curves merged, which left a
double hump in the top wall. It is now one arch: the track lifts out of each
vertex on a side curve (`SIDE_RADIUS`, 60, turning `SIDE_TURN`, 20 degrees),
crosses over on a top curve (about 59), and the drop meets the top curve with a
rounded corner of `CORNER_RADIUS` (13, the wall's own radius) on each side. The
track keeps its full width everywhere.

The bottom edge leaves little room: the drop and its walls take the middle, and
the corners must fit above the edge. About 13 is the largest corner these side
settings allow; a larger value stops the generator with a message. Lifting the
sides more raises the arch, which allows rounder corners but brings it closer
to tracks in the neighbouring tiles.

=== The four-way junction

`frag_1235` (`1234`, `1235`, `1236`) joins three vertices and one edge. Each of
its two long walls, which run from one vertex arm round to the next, used to
have a kink: a tight 9-unit corner followed by a 31-unit curve back the other
way. Each is now straight, one curve of `LONG_WALL_RADIUS` (35), then straight
again; the generator stops with a message if the curve no longer fits. The
short walls by the edge opening are unchanged. The curve rounds the inside of
the junction, so it comes no closer to tracks in neighbouring tiles.

=== The four-way junction with a straight side

`frag_1345` (`1345`, `1246`, `2356`, `1245`, `2346`, `1356`) joins two vertices
and two edges. Its long wall runs from one edge opening to the far vertex, and
both ends lie on one straight line at the angle the arms meet them. It used to
bulge into the junction and back (curve, tight hump, curve), which left a kink;
it is now straight. A smoother hump into the junction always pinched the track
below its full width, and a bow outwards crossed into the neighbouring tile
near an edge opening, so straight is the only kink-free wall that fits.

=== The five-way junction

`frag_13456` (`13456`, `12456`, `23456`) joins two vertices and all three
edges, five arms in all. The wall between the two edge openings beside the missing vertex used to
bulge into the junction and back (curve, tight hump, curve), which left a kink.
It is now one corner: straight, one curve of `INNER_CORNER_RADIUS` (35), then
straight into the other opening, turning the same way as the junction's other
corners. It only moves towards the junction, so the track keeps its full width
and comes no closer to neighbouring tiles.

=== The four-way junction with three edges

`frag_1456` (`1456`, `2456`, `3456`) joins one vertex and all three edges. The
two walls that run from a side opening to the opening opposite the vertex used
to curve one way and then tightly back (a 31-unit curve into a 9-unit corner).
Each is now straight, one corner of `SIDE_CORNER_RADIUS` (35), then straight,
turning the same way as the two corners beside the vertex arm. The new walls
sit further out than the old ones, so the junction is roomier; the measured
clearance tables did not change.

=== The four-way junction with two edges side by side

`frag_1346` (`1346`, `1256`, `2345`) joins two vertices and two edges that
share a corner of the tile. The wall between the two edge openings used to
curve, hump back the other way and curve again; it is now straight, one corner
of `EDGE_CORNER_RADIUS` (35), straight, as in `frag_1456`. The long wall
between the two vertex arms had a tight 9-unit corner; it now turns on
`LONG_CORNER_RADIUS` (60), which bows it out towards the tile's edge and away
from the junction. Both changes only make the junction roomier. Over 60 seeded
mazes its nearest neighbouring track stayed 12 units of white away, the same
as before.

=== The crescent

`frag_crescent` (`56/15/26`, `45/24/35`, `46/14/36`) draws a track from one
vertex round the far side of the tile to the next vertex, through the other two
edge openings. It used to be two long arcs joined by a short straight, which
left a corner halfway. All four points lie on a circle centred on the edge
between the two vertices, so it is now one semicircle of half a tile width.
`CRESCENT_RADIUS` (0.5) is its one setting: a smaller radius keeps the ends
where they are and joins two curves of that radius with a straight. The
semicircle reaches further over the edge openings, so other tracks that cross
it (as before) cross it a little further out; over 80 seeded 20 × 12 mazes the
closest track that does not cross it was 6.7 units of white away.

=== The vertex S

Where a track passes through a vertex from an edge opening in one tile to an edge
opening in the tile opposite, it used to be two hooks that met at the vertex
with opposite turns, which left a kink. The two openings point the same way and
sit side by side, so a smooth track has to turn a half-circle one way and a
half-circle back. `frag_vertex_s` (`15/14`, `24/26`, `36/35`, in normal and
inverted tiles) draws it as one S of two half-circles meeting at the vertex;
`FIRST_HALF_RADIUS` (30) sets the first, and the two add up to 60.

One tile draws the S and the tile opposite gets a blank fragment (added by the
generator), so it is drawn once. Its `ADJ_CHECKS` rules check the tile across
the vertex.

The half-circles swing into the tiles flanking the vertex, so the renderer only
keeps an S when:

- it is drawn clear of every net round it that it does not join, from the
  measured `VERTEX_S_CLEAR` table (`tools/clearance.py` measures it with the
  other tables);
- none of the flanking tiles draws, or is drawn by, a cross-tile fragment, whose
  shapes the table does not cover;
- no other S shares a tile round its vertex.

Otherwise it and its partner are drawn as the two hooks. Over 100 seeded 20 × 12
mazes this left 5 S-curves, none touching another track.

=== The corner hook

Where a track runs into a vertex from one tile and out of it into a dead end in
the tile opposite, it used to be two pieces: a hook into the vertex and a curl
out of it. Their turns went opposite ways, which left a bump in the walls.
`frag_corner_hook` (`14/14/4`, `15/15/5`, `24/24/4`, `26/26/6`, `35/35/5`,
`36/36/6`, in normal and inverted tiles) draws it as one track that only turns
one way: it arrives through the opposite tile's edge, makes one U-turn round the
vertex (`U_TURN_RADIUS`, about 35), then runs straight towards the dead-end node
and stops short of it with a round cap (`STOP_SHORT`, 22 to the middle of the
cap).

It is the first cross-tile fragment that reaches a tile across a vertex: its
`ADJ_CHECKS` entry checks the tile opposite the vertex (neighbour index 0-2) as
well as the tile across the edge, and is tried first. The opposite tile's net
can also be drawn by a cross-tile fragment from its other side, so the renderer
works out every tile's fragments, and any tile whose fragment across a vertex
claims a net that another cross-tile fragment also draws is drawn the usual way
instead, with its partner (`corner_hook_conflicts` in `maze/maze/render.rs`). Over 40 seeded 20 × 12
mazes it was used 11 times, with at least 8.5 units of white to other tracks.

=== The hook fork

Inside a junction, a track that comes in through a vertex and leaves by the
edge opening beside it turns 120° on a 20-unit centreline. That is the tightest
curve in the maze, and no redraw of the one tile can loosen it, because the
turn already ends exactly at the opening. Where the tile across that edge
carries on along the arc round the same vertex, the track hooks out of the
vertex and then orbits it.

`frag_hook_fork` draws the hook and the arc as one half-circle of radius 30
from the vertex to the arc's far opening, the same curve the plain `45/14`
family uses. What is left of the junction is the plain track from the vertex
to its third node: straight to the opposite edge, or the one-tile-wide arc to
another vertex. The two tracks fork at the vertex, and the point where they
separate is rounded with `JOIN_RADIUS` (9.2).

The arc's tile draws the fragment and hides the junction. There are 18 keys
(`46/245`, `56/236`, ...): each arc with each junction made of its vertex, one
of its edges and a third node. Junctions that hook to both edges beside the
vertex (`145`, `246`, `356`) are left out, since two arc tiles could claim them.
The rules come after the existing ones in `ADJ_CHECKS`, so a plain hook on the
arc's other side still wins.

The new shape stays inside the space the two nets already took up, so it is
used only when every other net on the two tiles was drawn clear of the net it
replaces, not crossing it (`find_best_path` checks the measured same-tile
table). Over 100 seeded 20 × 12 mazes it was used about 6 times per maze, with
at least 7.6 units of white to every other track. For the same reason it does
not count as a cross-tile fragment when a vertex S checks its flanking tiles;
otherwise a hook fork beside an S would turn the S back into two hooks.

=== The vertex sweep

A wide arc from one vertex to the next can carry on through the second vertex
into a hook round to the edge beside it. When both turn the same way the hook,
drawn on its own, still swings out before it turns, which leaves a wiggle after
the arc. `frag_vertex_sweep` (`14/12`, `15/13`, `24/12`, `26/23`, `35/13`,
`36/23`) draws the two as one sweep that only turns one way: an arc of radius
150 that ends a quarter of a tile past the vertex, then an arc of 30 into the
edge opening. The ends are where they were, and the radii follow from that.

The hook's tile draws it and hides the arc in the tile opposite the vertex. The
sweep passes beside the vertex and bulges further than the two shapes it
replaces, so it is treated like the vertex S: its six keys follow the three S
keys in `VERTEX_S_KEYS`, `tools/clearance.py` measures all nine, and the
renderer keeps a sweep only where that table says it is clear, no flanking tile
is part of another cross-tile fragment, and no S or other sweep shares a tile
round the vertex (an S always wins). Its rule comes last for the hook, so a
tile across the hook's edge that already draws the hook keeps doing so.

It is rare: over 100 seeded 20 × 12 mazes the pattern occurred 208 times and
the sweep was kept 11 times, with at least 4 units of white to other tracks.

=== Finding kinks <kinks>

`renders/kinks.rs` measures kinks in the maze as drawn. Every wall is straight
lines and circular arcs, so along a wall the heading $theta(s)$ is known
everywhere and the curvature $kappa = dif theta slash dif s$ is 0 on a line and
$plus.minus 1 slash r$ on an arc. The walls of all drawn fragments are joined
end to end where their ends meet (at the openings), cut at the round caps of
dead ends, and each joined wall is checked for:

- a *corner*: the heading jumps by more than `BREAK_ANGLE` (8°) where two
  pieces meet;
- a *wiggle*: the wall turns one way and then back the other with at most
  `KINK_STRAIGHT` (25) of straight between, the tighter curve under
  `KINK_RADIUS` (15) and the other under `KINK_WIDE_RADIUS` (35). Two
  back-to-back bends of a 20-unit centreline have wall radii 9.2 and 30.8,
  which is every hump and double hook redrawn by hand so far.

A wall that bends one way only has no kink, however tight. A junction's rounded
corner running into an ordinary arc is not one either, and the S-curve
fragments (15.2 or more) pass. The two walls of a track kink together, so kinks
within 30 units of each other count once.

`-K` (`--show-kinks`) rings each kink on the maze (red for a corner, purple for
a wiggle) and adds the number to the caption. `--kink-stats N` builds $N$ seeded
mazes instead of drawing one and prints the kinks by fragment, or by pair of
fragments where the kink is at a join. Over 100 seeded 20 × 12 mazes there were
9.3 kinks per maze: 68% inside `1456`/`2456`/`3456`, 28% inside
`1256`/`1346`/`2345`, and 3% at joins between fragments. Redrawing the first
family (below) left 2.8 per maze, and redrawing the second left none.

== Drawing by hand in ink

Mazes are drawn as if by hand, after maze-rs's ink style
(`renders/ink.rs`). Every fragment is flattened to pieces of at most 6 units,
each point is nudged up to 2 units by a smooth noise field, and the walls are
drawn in brown ink with a pen whose width wanders between three widths. The
noise is seeded from the maze's seed, so a maze always comes out the same, and
depends only on where a point is on the page, so walls that meet at a tile edge
still meet after the wobble. The entrance and exit dots are pencilled in red.

Tracks pass over one another, so each fragment's inside is painted white
(wobbled the same way) before its walls: the track drawn later hides the walls
of the one beneath, which reads as a break in the ink, as in maze-rs. Tile
outlines (`-B`) and the other debugging aids stay plain. No two tiles look
alike, so nothing is reused through `<use>`: an inked maze is several times
bigger than a plain one. `-P` (`--plain`) draws the even black walls instead,
which is what the library does unless `Maze::set_ink` is called.

= Documentation pipeline

`make document` runs the `document` crate; `make doc` also regenerates the
fragments first and compiles the Typst files afterwards:

+ `create_svgs` writes one SVG per non-empty, non-wall `MAP_PATHS` entry to
  `typst/svgs/`, drawn with the surrounding tile outlines and faint node
  numbers.
+ `create_tiles` enumerates combinations of two or more nets (with all their
  adjacent variants), keeps those that don't reuse a vertex, and writes each
  distinct drawing to `typst/tiles/NNNN.svg`.
+ `create_typst_doc` generates `fragments.typ` (4 columns) and
  `tiles_grid.typ` (7 columns), which are included by `shapes.typ` and
  `tiles.typ`.

Don't edit `fragments.typ` or `tiles_grid.typ` by hand. This file, `more.typ`,
`helper.typ` and `template.typ` are maintained by hand.

= Command line <cli>

`lubra` (`cli/src/main.rs`) builds one maze, then runs `reduce_nets`,
`remove_cycles`, `trim_dead_ends`, `separate_touching_tracks` and
`fill_blank_areas`, then renders it. It writes `<W>-by-<H>-sample-normal.svg`
(or `.png`) unless `--output` is given, and prints the total time.
With `--debug` or any debug overlay (`-B`, `-N`, `-T`) on, it also writes the
command line above the maze (e.g. `lubra --seed 1 -W 20 -H 12 -B -N -T`), so a
drawing can be reproduced from the image alone.
It also colours each net's track walls: nets in one tile get different colours,
neighbouring tiles start on different colours, and the `-T` labels use the same
colour for each net (`lib/src/renders/net_colours.rs`).

#table(
  columns: (auto, auto, 1fr),
  table.header([*Option*], [*Default*], [*Meaning*]),
  [`-W`, `--width`], [4], [Columns.],
  [`-H`, `--height`], [3], [Rows.],
  [`-s`, `--seed`], [random], [Seed for reproducible mazes.],
  [`-o`, `--output`], [], [Output file name, without extension.],
  [`-F`, `--format`], [`svg`], [`svg` or `png`.],
  [`-B`, `--boundaries`], [off], [Draw tile outlines.],
  [`-N`, `--show-node-numbers`], [off], [Label nodes and tiles.],
  [`-T`, `--show-tile-nets`], [off], [Write each tile's nets in blue beside its coordinates, e.g. `12,5` (nodes 1–2 joined, a dead end at 5); above them in tiles that point up, below in tiles that point down.],
  [`-R`, `--randomise`], [on], [Fill the grid with random tiles.],
  [`-b`, `--background`], [off], [Light-blue background.],
  [`-d`, `--debug`], [off], [Timings and debug output.],
  [`--reduce`], [1], [`lower_limit` passed to `reduce_nets`; 0 skips reduction.],
  [`--remove-cycles`], [on], [Run `remove_cycles`.],
  [`--trim-dead-ends`], [2], [Remove dead ends up to this many tiles long; 0 keeps them all.],
  [`--keep-touching`], [off], [Skip `separate_touching_tracks`.],
  [`--fill`], [2], [Grow tracks into blank areas; new side branches are at least this many tiles long. 0 leaves blank areas alone.],
  [`-P`, `--plain`], [off], [Even black walls instead of hand-drawn ink.],
  [`-K`, `--show-kinks`], [off], [Ring every kink and count them in the caption.],
  [`--kink-stats`], [], [Build this many seeded mazes and print their kinks by fragment, instead of drawing.],
)

= Current limitations

- `reduce_nets` stops with more than one net if no shared tile, free node or
  empty tile can join the rest. This hasn't happened in testing: every
  seeded maze in the tests ends as one net.

= Arc fragment families

Normal (`n`) then inverted (`i`) variants of the 4–5, 4–6 and 5–6 arcs, alone
and combined with the neighbouring nets they merge with.

#include "more.typ"

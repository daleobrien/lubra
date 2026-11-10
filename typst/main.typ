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
  [`make cli`], [The CLI with its default options.],
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
paths are `PATH_WIDTH` $= 0.11$ tile widths wide, which is narrow enough
that fragments in neighbouring tiles never overlap.

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
    place nets at $(i, j)$ and add them to the global net list\
    seed $<-$ seed $+ 1$#d
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
  lines, vertex-to-vertex lines and the two 3-ways 1–2–3 and 4–5–6.
- `nets_large.rs` / `mass_large.rs`: every combination of 1- to 6-node
  nets, including single-node dead ends. Used only when the small table has
  no option for a constrained tile.

`NodeCombinations` lists every set of non-overlapping nets. Each set is
registered under every mask it satisfies, with each used node marked as
either 1 or 2. Its mass is

$ "mass" = |"nets"| dot product_("net") w("net"), quad w = cases(20 & "2 nodes", 5 & "3 nodes", 2 & "4 nodes", 1 & "otherwise") $

so simple corridors are strongly favoured, and an empty tile (mass 0) is
chosen only when it is the only option. `random_tile` (`maze/random.rs`) picks
an entry with a Walker alias table (`weighted_rand`) and shuffles the nets to
vary the render order.

== Tracking and reducing nets

As tiles are placed, `update_network` (`maze/maze/net_operations.rs`) keeps
the maze-wide `Net`: a list of `Nodes`, each a set of `Node { i, j, node }`
positions (`maze/net.rs`). A new tile net either starts a new entry, extends
one, or merges several that it bridges.

`reduce_nets(lower_limit)` (`maze/maze/net_reduction.rs`) tries to join the
disconnected nets. It does nothing if there is at most one net. Otherwise:

+ sort the nets by size and remove every net with at most `lower_limit`
  nodes, deleting it from the tiles (`remove_short_nets`);
+ find pairs of nets that pass through the same tile (`get_intersections`),
  skipping pairs already linked through another net;
+ for each pair: if a shared tile holds exactly two nets, replace them with
  one combined net and join the two nets. Otherwise, in every shared tile,
  pick a random node of each net (`select_random_element`) and combine the
  tile nets that contain them;
+ drop the nets that were merged away;
+ if more than one net is left, use `find_adjacent_tile_pairs` to find
  neighbouring tiles of the first two nets and open the node between them
  (`get_connecting_node_mask`), joining those two nets.

The neighbour search (`maze/maze/surrounding_coordinates.rs`) tries three
tiers in turn: tiles in both nets, tiles in one net next to the other, and
empty tiles next to both.

== Removing cycles

`remove_cycles(debug)` (`maze/maze/remove_cycles.rs`) turns each net into a
tree, so there is only one route between any two points:

+ build a node adjacency map for every net;
+ find cycles with a depth-first search and normalise them, so the same
  loop found from different starting points is counted once;
+ for each cycle, remove its second-to-last node from the net. In the tile,
  that node is split off its tile net and kept as a single-node net (a dead
  end);
+ repeat until no cycles remain (at most 100 passes);
+ split any net that is now disconnected into its connected components.

= Rendering

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
a `<g>` in `<defs>`. Every placement is a `<use href>` positioned at
$(i dot W / 2, j dot H)$. The SVG has six layers, drawn bottom to top, and
the y axis is flipped so that row 0 is at the bottom.

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

`lubra` (`cli/src/main.rs`) builds one maze, then runs `reduce_nets` and
`remove_cycles`, then renders it. It writes `<W>-by-<H>-sample-normal.svg`
(or `.png`) unless `--output` is given, and prints the total time.

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
  [`-R`, `--randomise`], [on], [Fill the grid with random tiles.],
  [`-b`, `--background`], [off], [Light-blue background.],
  [`-d`, `--debug`], [off], [Timings and debug output.],
  [`--reduce`], [1], [`lower_limit` passed to `reduce_nets`; 0 skips reduction.],
  [`--remove-cycles`], [on], [Run `remove_cycles`.],
)

= Current limitations

- `reduce_nets` only connects the first two remaining nets in its final
  step. The code also marks `remove_short_nets` and the complex merge (which
  doesn't update the net list) as unfinished.
- Fragments are drawn on one layer, so nets that extend into a neighbouring
  tile can overlap in the wrong order (a TODO in `render.rs`).

= Arc fragment families

Normal (`n`) then inverted (`i`) variants of the 4–5, 4–6 and 5–6 arcs, alone
and combined with the neighbouring nets they merge with.

#include "more.typ"

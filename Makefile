# Build, generation and documentation tasks for the Lubra workspace.
# Each crate is run from its own directory, so output files land where the code
# expects them.

CARGO ?= cargo
TYPST ?= typst

# Arguments for `make cli`. Width is in columns of half-tiles (60 units each)
# and height in rows (about 104 units each), so W ~ 1.73 * H looks square.
# -B draws the tile boundaries, -N the node numbers, -T each tile's nets and -K
# rings the kinks (the debug overlay); drop them for a clean maze, e.g.
# make cli CLI_ARGS="-W 20 -H 12". Mazes are drawn in ink; add -P for plain walls.
# The fixed seed makes every run draw the same maze; change it to see another.
CLI_ARGS ?= --seed 1 -W 20 -H 12 -B -N -T -K

# Arguments for `make sample`: the maze shown in the README, without the debug
# overlay. Drawn in ink, like every maze unless -P (--plain) is given; it is
# written once as is and once with its solution (-S).
SAMPLE_ARGS ?= --seed 1 -W 20 -H 12

# Arguments for `make maze`, e.g. make maze MAZE_ARGS="--seed 7 -W 20 -H 20"
MAZE_ARGS ?= --seed 3 -W 40 -H 30 -B -N --debug

.DEFAULT_GOAL := help
# Pipeline prerequisites must run in order.
.NOTPARALLEL:
.PHONY: help generate both maze doc full width \
	gen gen-svg gen-nets clearance cli sample document typst build test

help:
	@echo "Pipelines:"
	@echo "  make generate  generator (svg + nets), then CLI, then document"
	@echo "  make both      generator svg, then CLI"
	@echo "  make maze      build CLI and time one debug maze"
	@echo "  make doc       generator svg, document, then typst compile (PDFs into docs/)"
	@echo "  make full      generator nets + svg, then CLI, then document"
	@echo "  make width     after changing PATH_WIDTH: fragments, then the clearance tables"
	@echo "Steps:"
	@echo "  make gen | gen-svg | gen-nets | clearance | cli | sample | document | typst | build | test"

# Pipelines
generate: gen cli document
both: gen-svg cli
doc: gen-svg document typst
full: gen-nets gen-svg cli document

maze:
	cd cli && $(CARGO) build
	@echo "Running ..."
	cd cli && time $(CARGO) run --quiet -- $(MAZE_ARGS)

# Steps
gen:
	cd generate && $(CARGO) run --package generate

gen-svg:
	cd generate && $(CARGO) run --package generate -- svg

gen-nets:
	cd generate && $(CARGO) run --package generate -- nets

# Everything that depends on the track width (PATH_WIDTH in common/src/types/point.rs):
# the fragments, then the clearance tables measured from them. Run it after changing
# the width; without the tables, wider tracks touch and cross wrongly.
width: gen-svg clearance

# Re-measures how tracks meet (clear, crossing or touching) from the fragments as
# drawn: needs rsvg-convert and Python with numpy, Pillow and OpenCV.
CLEARANCE_DIR ?= /tmp/lubra-clearance
clearance:
	rm -rf $(CLEARANCE_DIR) && mkdir -p $(CLEARANCE_DIR)
	CLEARANCE_DIR=$(CLEARANCE_DIR) $(CARGO) test -q -p lib dump_clearance_shapes -- --ignored
	python3 tools/clearance.py $(CLEARANCE_DIR) lib/src/maze/maze/fill_clearance.rs \
		lib/src/renders/vertex_s_clearance.rs

cli:
	cd cli && $(CARGO) run -- $(CLI_ARGS)

sample:
	cd cli && $(CARGO) run --release -- $(SAMPLE_ARGS) -o ../docs/sample-ink
	cd cli && $(CARGO) run --release -- $(SAMPLE_ARGS) -S -o ../docs/sample-ink-solved

document:
	cd document && $(CARGO) run

# The PDFs go into docs/, next to the README's images.
typst:
	cd typst && $(TYPST) compile shapes.typ ../docs/shapes.pdf
	cd typst && $(TYPST) compile tiles.typ ../docs/tiles.pdf
	cd typst && $(TYPST) compile main.typ ../docs/main.pdf

build:
	$(CARGO) build --workspace

test:
	$(CARGO) test --workspace

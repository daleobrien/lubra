# Build, generation and documentation tasks for the Lubra workspace.
# Each crate is run from its own directory, so output files land where the code
# expects them.

CARGO ?= cargo
TYPST ?= typst

# Arguments for `make cli`. Width is in columns of half-tiles (60 units each)
# and height in rows (about 104 units each), so W ~ 1.73 * H looks square.
# -B draws the tile boundaries and -N the node numbers (the debug overlay);
# drop them for a clean maze, e.g. make cli CLI_ARGS="-W 20 -H 12".
CLI_ARGS ?= -W 20 -H 12 -B -N

# Arguments for `make maze`, e.g. make maze MAZE_ARGS="--seed 7 -W 20 -H 20"
MAZE_ARGS ?= --seed 3 -W 40 -H 30 -B -N --debug

.DEFAULT_GOAL := help
# Pipeline prerequisites must run in order.
.NOTPARALLEL:
.PHONY: help generate both maze doc full \
	gen gen-svg gen-nets cli document typst build test

help:
	@echo "Pipelines:"
	@echo "  make generate  generator (svg + nets), then CLI, then document"
	@echo "  make both      generator svg, then CLI"
	@echo "  make maze      build CLI and time one debug maze"
	@echo "  make doc       generator svg, document, then typst compile"
	@echo "  make full      generator nets + svg, then CLI, then document"
	@echo "Steps:"
	@echo "  make gen | gen-svg | gen-nets | cli | document | typst | build | test"

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

cli:
	cd cli && $(CARGO) run -- $(CLI_ARGS)

document:
	cd document && $(CARGO) run

typst:
	cd typst && $(TYPST) compile shapes.typ
	cd typst && $(TYPST) compile tiles.typ
	cd typst && $(TYPST) compile main.typ

build:
	$(CARGO) build --workspace

test:
	$(CARGO) test --workspace

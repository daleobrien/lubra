# Replaces the old helper scripts (generate.sh, both.sh, maze.sh, doc.sh, full.sh).
# Each crate is run from its own directory, as the scripts did, so output files
# land in the same places.

CARGO ?= cargo
TYPST ?= typst

# Arguments for `make maze`, e.g. make maze MAZE_ARGS="--seed 7 -W 20 -H 20"
MAZE_ARGS ?= --seed 3 -W 40 -H 30 -B -N --debug

.DEFAULT_GOAL := help
# Pipeline prerequisites must run in order.
.NOTPARALLEL:
.PHONY: help generate both maze doc full \
	gen gen-svg gen-nets cli document typst build test

help:
	@echo "Pipelines:"
	@echo "  make generate  generator (svg + nets), then CLI, then document   (was generate.sh)"
	@echo "  make both      generator svg, then CLI                           (was both.sh)"
	@echo "  make maze      build CLI and time one debug maze                 (was maze.sh)"
	@echo "  make doc       generator svg, document, then typst compile       (was doc.sh)"
	@echo "  make full      generator nets + svg, then CLI, then document     (was full.sh)"
	@echo "Steps:"
	@echo "  make gen | gen-svg | gen-nets | cli | document | typst | build | test"

# Pipelines (same order as the old scripts)
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
	cd cli && $(CARGO) run

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

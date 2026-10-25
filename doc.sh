#!/usr/bin/env bash
set -e

FOLDER=$(cd `dirname $0` && pwd)

cd "${FOLDER}/generate"
cargo run --package generate -- svg

cd "${FOLDER}/document"
cargo run

cd "${FOLDER}/typst"
typst compile shapes.typ
typst compile tiles.typ
typst compile main.typ

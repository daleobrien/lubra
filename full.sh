#!/usr/bin/env bash
set -

FOLDER=$(cd `dirname $0` && pwd)

cd "${FOLDER}/generate"
cargo run --package generate -- nets
cargo run --package generate -- svg

cd "${FOLDER}/cli"
cargo run

cd "${FOLDER}/document"
cargo run

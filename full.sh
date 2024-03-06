#!/usr/bin/env bash
set -

FOLDER=$(cd `dirname $0` && pwd)

cd ${FOLDER}/generate
cargo run --bin generate_nets
cargo run --bin generate_svg

cd ${FOLDER}/cli
cargo run

cd ${FOLDER}/document
cargo run
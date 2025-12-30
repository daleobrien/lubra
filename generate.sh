#!/usr/bin/env bash
set -e
set -x

FOLDER=$(cd `dirname $0` && pwd)

# rm -rf cli/*.svg

cd ${FOLDER}/generate
cargo run --package generate

cd ${FOLDER}/cli
cargo run

cd ${FOLDER}/document
cargo run
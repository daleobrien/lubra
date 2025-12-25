#!/usr/bin/env bash
set -e

FOLDER=$(cd `dirname $0` && pwd)

rm -rf cli/*.svg

cd ${FOLDER}/generate
cargo run --package generate -- svg

cd ${FOLDER}/cli
cargo run

cd ${FOLDER}/document
cargo run
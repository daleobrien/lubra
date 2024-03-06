#!/usr/bin/env bash
set -e

FOLDER=$(cd `dirname $0` && pwd)

cd ${FOLDER}/generate
cargo run --bin generate_svg

cd ${FOLDER}/document
cargo run
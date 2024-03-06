#!/usr/bin/env bash
set -e

FOLDER=$(cd `dirname $0` && pwd)

echo "${FOLDER}"

cd ${FOLDER}/generate
cargo run --bin generate_svg

cd ${FOLDER}/cli
cargo run

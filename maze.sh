#!/usr/bin/env bash
set -e

FOLDER=$(cd `dirname $0` && pwd)

echo "${FOLDER}"

cd ${FOLDER}/cli
cargo run

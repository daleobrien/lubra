#!/usr/bin/env bash
set -e

FOLDER=$(cd `dirname $0` && pwd)

echo "${FOLDER}"

cd ${FOLDER}/cli
#rm -f *.svg
cargo build # --release

echo "Running ..."
time ./target.tmp/debug/lubra --seed 3 -W 40 -H 30 -B -N --debug
#time ./target.tmp/release/lubra --seed 3 -W 40 -H 30 -B -N --debug

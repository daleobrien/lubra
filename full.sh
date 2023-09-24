#!/usr/bin/env bash
set -

cd generate
cargo run --bin generate_nets
cargo run --bin generate_svg
cd ../cli
cargo run

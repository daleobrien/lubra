#!/usr/bin/env bash
set -e

cd generate
cargo run
cd ../cli
cargo run

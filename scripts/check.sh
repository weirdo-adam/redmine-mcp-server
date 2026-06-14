#!/usr/bin/env sh
set -eu

cargo fmt --check
cargo test

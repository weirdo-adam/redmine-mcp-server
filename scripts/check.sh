#!/usr/bin/env sh
set -eu

cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test

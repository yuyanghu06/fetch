#!/usr/bin/env bash
set -euo pipefail

# Run from the checkout regardless of the caller's current directory.
cd "$(dirname "$0")/.."
cargo fmt --all --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked --all-targets

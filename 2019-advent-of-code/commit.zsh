#!/bin/zsh

# exit on all non-zero codes
set -e

# precommit checks
cargo test
cargo clippy

git add .

echo 'COMMIT MESSAGE: (Ctrl+C to abort)'
# TODO: autodetect what day we are committing to
git commit -m "aoc19: 1: $(read -r; printf '%s' "$REPLY")"
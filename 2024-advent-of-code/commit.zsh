#!/bin/zsh

# exit on all non-zero codes
set -e

# precommit checks
cargo test
cargo clippy

day=$(git status . --porcelain | (rg 'src/bin/(day\d\d).rs' -or '$1' || true))

if [[ -z "${day// }" ]]; then
    echo 'WARNING: NO DAY DETECTED'
    day=""
elif [ $(echo $day | wc -l) = "1" ]; then
    echo "DETECTED $day"
    day=" $day:"
else
    echo 'WARNING: MULTIPLE DAYS DETECTED'
    day=""
fi

git add .

echo 'COMMIT MESSAGE: (Ctrl+C to abort)'
git commit -m "aoc24:$day $(read -r; printf '%s' "$REPLY")"

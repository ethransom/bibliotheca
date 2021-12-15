#!/bin/zsh

if [[ -z "$1" ]]; then
    echo "Must provide day of puzzle, left-padded with zeroes!!" 1>&2
    exit 1
fi

SOURCE="src/bin/day$1.rs"
EXAMPLE="src/bin/example$1.txt"
INPUT="src/bin/input$1.txt"

if [ -f "$SOURCE" ]; then
    echo "$SOURCE already exists!"
    exit 1
fi

if [ -f "$EXAMPLE" ]; then
    echo "$EXAMPLE already exists!"
    exit 1
fi

if [ -f "$INPUT" ]; then
    echo "$INPUT already exists!"
    exit 1
fi

set -ex

cat "src/bin/dayXX.rs.tmpl" | sed "s+XX+$1+g" > $SOURCE
touch $EXAMPLE
touch $INPUT

#!/bin/sh

PROJECT=$(echo "day$1")

if [ -d $PROJECT ]; then
    echo "$PROJECT already set up"
else
    echo "Setting up $PROJECT..."

    cargo new $PROJECT
    echo "aoc_utils = { path = \"../utils\" }" >> $PROJECT/Cargo.toml

    curl -b session=$(cat .aocsession) https://adventofcode.com/2018/day/$1/input > $PROJECT/input.txt

    cd $PROJECT
    cargo build
fi
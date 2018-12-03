#!/bin/sh

PROJECT=$(echo "day$1-test")

if [ -d $PROJECT ]; then
    echo "$PROJECT already set up"
else
    echo "Setting up $PROJECT..."
    cargo new $PROJECT
    curl -b session=$(cat .aocsession) https://adventofcode.com/2018/day/$1/input > $PROJECT/input.txt
    ln -s $(pwd)/util $PROJECT/src/util
fi
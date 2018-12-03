#!/bin/sh

PROJECT=$(echo "day$1$2")

if [ -d $PROJECT ]; then
    echo "$PROJECT already set up"
else
    echo "Setting up $PROJECT..."

    # Create cargo crate
    cargo new $PROJECT

    # Append dependencies to the crate
    echo "aoc_utils = { path = \"../utils\" }" >> $PROJECT/Cargo.toml
    echo "text_io = \"0.1.7\"" >> $PROJECT/Cargo.toml

    # Prepend extern crate statements to main.rs to use aoc_utils and text_io
    main_file=$(cat $PROJECT/src/main.rs)
    echo "extern crate aoc_utils;" > $PROJECT/src/main.rs
    echo "#[macro_use] extern crate text_io;" >> $PROJECT/src/main.rs
    echo "" >> $PROJECT/src/main.rs
    echo "$main_file" >> $PROJECT/src/main.rs

    # Get input file
    curl -b session=$(cat .aocsession) https://adventofcode.com/2018/day/$1/input > $PROJECT/input.txt

    # Create a test input file
    touch $PROJECT/test_input.txt

    # Move to project dir and build
    cd $PROJECT
    cargo build
fi
extern crate aoc_utils;
#[macro_use]
extern crate text_io;

mod solver;
mod step;

use aoc_utils::file_utils;
use solver::get_ordered_steps;
use std::cmp::max;
use step::Step;

fn main() {
    let mut buffer: String = String::new();
    let lines: Vec<&str> =
        file_utils::get_input("./input.txt", &mut buffer, true).expect("Error reading input");

    let steps: Vec<Step> = step::parse_steps(lines);
    println!("Step order: {}", get_ordered_steps(&steps));
}

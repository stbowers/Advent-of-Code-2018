extern crate aoc_utils;
#[macro_use]
extern crate text_io;

mod solver;
mod step;

use aoc_utils::file_utils;
use solver::{get_ordered_steps, get_time_to_complete};
use step::Step;

fn main() {
    let mut buffer: String = String::new();
    let lines: Vec<&str> =
        file_utils::get_input("./input.txt", &mut buffer, true).expect("Error reading input");

    let steps: Vec<Step> = step::parse_steps(lines);
    println!("Step order: {}", get_ordered_steps(&steps));
    println!(
        "Work will take {} seconds.",
        get_time_to_complete(&steps, 60, 5)
    );
}

extern crate time;

use std::fs::File;
use std::io::Read;
use time::{Duration, PreciseTime};

fn main() {
    let start = PreciseTime::now();
    let mut file: File = File::open("input.txt").expect(&format!("Error reading input file\n"));

    // read file into contents
    let mut contents: String = String::new();
    file.read_to_string(&mut contents)
        .expect(&format!("Error reading input file\n"));

    // get an iterator for lines in the file (contents split along newline char, remove trailing whitespace)
    let input_strings: Vec<&str> = contents
        .split("\n")
        .map(|line: &str| -> &str {
            return line.trim_end();
        }).collect();

    let mut frequency: i32 = 0;
    let mut seen_frequencies: Vec<i32> = Vec::new();
    seen_frequencies.push(0);

    let mut iterations: i32 = 0;
    loop {
        iterations += 1;
        let mut frequency_found: bool = false;

        for frequency_change in input_strings.iter() {
            let d_frequency: i32 = frequency_change.parse::<i32>().unwrap();
            frequency += d_frequency;

            let search: Result<usize, usize> =
                seen_frequencies.as_slice().binary_search(&frequency);

            if search.is_ok() {
                frequency_found = true;
                break;
            } else {
                seen_frequencies.insert(search.unwrap_err(), frequency);
            }
        }

        if frequency_found {
            break;
        }
    }

    let end = PreciseTime::now();
    println!(
        "First duplicate frequency: {} ({} iterations)",
        frequency, iterations
    );

    println!("Time: {}", start.to(end));
}

#[macro_use]
extern crate itertools;

use std::fs::File;
use std::io::Read;

fn main() {
    let mut input: File = File::open("./input.txt").expect("Error reading input file...");
    let mut buffer: String = String::new();
    input
        .read_to_string(&mut buffer)
        .expect("Error reading input file");

    let mut polymer1: String = buffer.clone();

    let reactions: u32 = react_polymer(&mut polymer1);
    println!(
        "After {} reactions polymer is {} units long",
        reactions,
        (&polymer1).trim_end().len()
    );

    let mut best_length = buffer.len();
    for ch in ('a' as u8)..('z' as u8) {
        let mut polymer: String = buffer.clone();
        remove_type(&mut polymer, ch as char);
        react_polymer(&mut polymer);
        if polymer.len() < best_length {
            best_length = polymer.len();
        }
        println!("{}", ch as char);
    }

    println!("Shortest length: {}", best_length);
}

fn remove_type(polymer: &mut String, type_ch: char) {
    let type_lowercase = type_ch.to_lowercase().next().unwrap();
    let type_uppercase = type_ch.to_uppercase().next().unwrap();
    polymer.retain(|c| c != type_lowercase && c != type_uppercase);
}

/// Reacts the given polymer until no more reactions can be made, and returns how many times the polymer was reacted
fn react_polymer(polymer: &mut String) -> u32 {
    let mut reactions: u32 = 0;
    while react_partial(polymer) {
        reactions += 1;
    }
    return reactions;
}

/// Reacts the polymer once, and returns true if a reaction occured
fn react_partial(polymer: &mut String) -> bool {
    let mut remove_index: isize = -1;

    // iterate through pairs of characters until the first pair of opposite units
    for (index, (ch1, ch2)) in ((&polymer[0..]).chars().zip((&polymer[1..]).chars())).enumerate() {
        // check for opposite cases
        if ch1.is_lowercase() != ch2.is_lowercase() {
            // Check if they're the same type
            let ch1_uppercase: char = ch1.to_uppercase().next().unwrap();
            let ch2_uppercase: char = ch2.to_uppercase().next().unwrap();
            if ch1_uppercase == ch2_uppercase {
                // Remove pair
                remove_index = index as isize;
                break;
            }
        }
    }

    if remove_index != -1 {
        polymer.remove(remove_index as usize);
        polymer.remove(remove_index as usize);
        return true;
    } else {
        // no reactions occured
        return false;
    }
}

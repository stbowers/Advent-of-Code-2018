extern crate aoc_utils;

use aoc_utils::file_utils;
use std::iter::Iterator;

fn main() {
    let mut buffer: String = String::new();
    let box_ids: Vec<&str> =
        file_utils::get_input("input.txt", &mut buffer).expect("Error reading input file");

    let checksum: u32 = get_checksum(&box_ids);
    println!("Checksum: {}", checksum);

    let common_letters: String = get_common_letters(&box_ids, 1);
    println!("Common letters: {}", common_letters);
}

fn get_common_letters(box_ids: &Vec<&str>, differing_by: u8) -> String {
    // find the first two strings which have only one difference
    for id_1 in box_ids.iter() {
        for id_2 in box_ids.iter() {
            if get_differences(id_1, id_2) == differing_by {
                // id_1 and id_2 differ by the specified ammount
                // build a string from the identical letters, and return
                let mut buffer: String = String::new();

                for (ch_1, ch_2) in id_1.chars().zip(id_2.chars()) {
                    if ch_1 == ch_2 {
                        buffer.push(ch_1);
                    }
                }

                return buffer;
            }
        }
    }

    return String::new();
}

fn get_differences(id_1: &str, id_2: &str) -> u8 {
    let mut differences: u8 = 0;
    for (ch_1, ch_2) in id_1.chars().zip(id_2.chars()) {
        if ch_1 != ch_2 {
            differences += 1;
        }
    }

    return differences;
}

fn get_checksum(box_ids: &Vec<&str>) -> u32 {
    // First get count of ids with a letter that appears exactly twice
    let num_two_appearances: u32 = count_n_appearances(box_ids, 2);
    let num_three_appearances: u32 = count_n_appearances(box_ids, 3);

    return num_two_appearances * num_three_appearances;
}

fn count_n_appearances(box_ids: &Vec<&str>, num_appearances: u8) -> u32 {
    let mut count: u32 = 0;
    for id in box_ids.iter() {
        for ch in id.chars() {
            let mut num_ch = 0;
            for other_ch in id.chars() {
                if other_ch == ch {
                    num_ch += 1;
                }
            }

            if num_ch == num_appearances {
                count += 1;
                break;
            }
        }
    }

    return count;
}

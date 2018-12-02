mod util;

fn main() {
    let mut buffer: String = String::new();
    let box_ids: Vec<&str> =
        util::get_input("input.txt", &mut buffer).expect("Error reading input file");

    let checksum: u32 = get_checksum(&box_ids);
    println!("Checksum: {}", checksum);
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

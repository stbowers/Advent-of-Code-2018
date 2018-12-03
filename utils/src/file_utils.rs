use std::fs::File;
use std::io::Read;

pub fn get_input<'a>(filename: &str, buffer: &'a mut String) -> std::io::Result<Vec<&'a str>> {
    let mut file: File = File::open(filename)?;

    // read file into contents
    file.read_to_string(buffer)?;

    // get an iterator for lines in the file (contents split along newline char, remove trailing whitespace)
    let input_strings: Vec<&str> = buffer
        .split("\n")
        .map(|line: &str| -> &str {
            return line.trim_end();
        }).collect();

    return Ok(input_strings);
}

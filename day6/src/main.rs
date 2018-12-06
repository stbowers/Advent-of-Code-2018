extern crate aoc_utils;
#[macro_use]
extern crate text_io;

mod board;
mod vector;

use aoc_utils::file_utils;
use board::Board;
use vector::Vector;

fn main() {
    let mut buffer: String = String::new();
    let input: Vec<&str> =
        file_utils::get_input("./input.txt", &mut buffer, true).expect("Error reading input file");

    let board: Board = Board::from_points(get_points(&input));
    let biggest_area: i32 = board.get_largest_area();
    println!("Part 1: {}", biggest_area);

    let area_near_points: i32 = board.get_area_near_points(10000);
    println!("Part 2: {}", area_near_points);
}

fn get_points(lines: &Vec<&str>) -> Vec<Vector> {
    let mut points: Vec<Vector> = Vec::new();

    for &line in lines {
        let (x, y): (i32, i32);
        scan!(line.bytes() => "{}, {}", x, y);

        points.push(Vector::new(x, y));
    }

    return points;
}

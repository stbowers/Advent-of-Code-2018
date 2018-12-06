extern crate aoc_utils;
#[macro_use]
extern crate text_io;

use std::ops::{Add, Sub};

struct Vector {
    x: i32,
    y: i32,
}

impl Vector {
    pub fn new(x: i32, y: i32) -> Vector {
        return Vector { x: x, y: y };
    }

    pub fn manhattan_distance(v1: Vector, v2: Vector) -> i32 {
        let d: Vector = v1 - v2;
        return d.x.abs() + d.y.abs();
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        return Vector {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Vector {
        return Vector {
            x: self.x - other.x,
            y: self.y - other.y,
        };
    }
}

fn main() {
    println!("Hello, world!");
}

use std::ops::{Add, Sub};

pub struct Vector {
    x: i32,
    y: i32,
}

impl Vector {
    pub fn new(x: i32, y: i32) -> Vector {
        return Vector { x: x, y: y };
    }

    pub fn manhattan_distance(v1: &Vector, v2: &Vector) -> i32 {
        let dx = (v1.get_x() - v2.get_x()).abs();
        let dy = (v1.get_y() - v2.get_y()).abs();

        return dx + dy;
    }

    pub fn get_x(&self) -> i32 {
        return self.x;
    }

    pub fn get_y(&self) -> i32 {
        return self.y;
    }
}

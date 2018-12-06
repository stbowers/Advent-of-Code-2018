use std::cmp::{max, min};
use vector::Vector;

pub struct Board {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    points: Vec<Vector>,
}

impl Board {
    pub fn from_points(points: Vec<Vector>) -> Board {
        let mut new_board: Board = Board {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            points: points,
        };

        for point in new_board.points.iter() {
            let x_max = max(new_board.x + new_board.width, point.get_x() + 1);
            let y_max = max(new_board.y + new_board.height, point.get_y() + 1);

            new_board.x = min(new_board.x, point.get_x() - 1);
            new_board.y = min(new_board.y, point.get_y() - 1);
            new_board.width = x_max - new_board.x;
            new_board.height = y_max - new_board.y;
        }

        return new_board;
    }

    pub fn get_area_near_points(&self, max_total_distance: i32) -> i32 {
        let mut area: i32 = 0;

        // Check each point
        for y in (self.y)..(self.y + self.height + 1) {
            for x in (self.x)..(self.x + self.width + 1) {
                let test_point: Vector = Vector::new(x, y);
                let mut total_distance: i32 = 0;
                for point in self.points.iter() {
                    total_distance += Vector::manhattan_distance(&test_point, point);
                }
                if total_distance < max_total_distance {
                    area += 1;
                }
            }
        }

        return area;
    }

    pub fn get_largest_area(&self) -> i32 {
        // (point index, area, on edge)
        let mut areas: Vec<(usize, i32, bool)> = Vec::new();

        for y in (self.y)..(self.y + self.height + 1) {
            for x in (self.x)..(self.x + self.width + 1) {
                let test_point: Vector = Vector::new(x, y);
                let is_edge = x == self.x
                    || y == self.y
                    || x == self.x + self.width - 1
                    || y == self.y + self.height - 1;
                let mut closest_point: usize = 0;
                let mut closest_distance: i32 = i32::max_value();
                let mut shared: bool = false;
                for (index, point) in self.points.iter().enumerate() {
                    let distance = Vector::manhattan_distance(&test_point, point);
                    if distance < closest_distance {
                        closest_point = index;
                        closest_distance = distance;
                        shared = false;
                    } else if distance == closest_distance {
                        shared = true;
                    }
                }

                // if shared, don't count it
                if !shared {
                    // Find point in areas and add one
                    let search: Result<usize, usize> =
                        areas.binary_search_by(|point| point.0.cmp(&closest_point));
                    if search.is_ok() {
                        areas[search.unwrap()].1 += 1;
                        areas[search.unwrap()].2 |= is_edge;
                    } else {
                        areas.insert(search.unwrap_err(), (closest_point, 1, is_edge))
                    }
                }
            }
        }

        // Filter out points on the edge
        areas.retain(|point| !point.2);

        // Sort by area in point
        areas.sort_unstable_by(|point1, point2| point1.1.cmp(&point2.1));

        // Return biggest area
        return areas[areas.len() - 1].1;
    }
}

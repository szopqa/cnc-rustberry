extern crate line_drawing;
use line_drawing::{
    Bresenham,
    BresenhamCircle,
    SignedNum
};

use super::position::{Position};

/* Implementation using: https://en.wikipedia.org/wiki/Bresenham's_line_algorithm */
pub fn _line_from_two_positions(_start_pos: &Position, _dest_pos: &Position) -> Vec<Position> {
    Bresenham::new(
        (_start_pos.get_x() as i32, _start_pos.get_y() as i32),
        (_dest_pos.get_x() as i32, _dest_pos.get_y() as i32)
    ).into_iter().map(|(x,y)| {
        Position::new(x as f32, y as f32, _start_pos.get_z())
    }).collect()
}

pub fn _circle_with_center_and_radius(_center: &Position, _radius: i32) -> Vec<Position> {
    BresenhamCircle::new(
        _center.get_x() as i32, _center.get_y() as i32, _radius
    ).into_iter().map(|(x,y)| {
        Position::new(x as f32, y as f32, _center.get_z())
    }).collect()
}
use crate::geometry::position::{Position};

pub trait Moveable {
    fn calibrate(&mut self);
    fn move_up(&mut self);
    fn move_down(&mut self);
    fn move_to_relative_pos(&mut self, _x: f32, _y: f32);
    fn move_to_absolute_pos(&mut self, _x: f32, _y: f32);
    fn get_current_pos(&self) -> &Position;
    // fn update_current_pos(&mut self, _x: i32, _y: i32, _z: i8);
}
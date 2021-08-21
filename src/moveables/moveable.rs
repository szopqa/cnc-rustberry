use crate::geometry::position::{Position};

pub trait Moveable {
    fn calibrate(&mut self);
    fn move_up(&mut self);
    fn move_down(&mut self);
    fn move_to_relative_pos(&mut self, _x_path: f32, _y_path: f32);
    fn move_to_absolute_pos(&mut self, _dest_pos: &Position);
    fn move_clockwise(&mut self, _start_pos: &Position, _finish_pos: &Position, _radius: i32);
    fn get_current_pos(&self) -> &Position;
}
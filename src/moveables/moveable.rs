use crate::geometry::position::{Position};

pub trait Moveable {
    fn calibrate(&mut self);
    fn move_up(&mut self);
    fn move_down(&mut self);
    fn move_to_relative_pos(&mut self, _x_path: f32, _y_path: f32);
    fn move_to_absolute_pos(&mut self, _dest_pos: &Position);
    fn get_current_pos(&self) -> &Position;
}
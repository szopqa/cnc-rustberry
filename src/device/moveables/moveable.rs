pub trait Moveable {
    fn calibrate(&mut self);
    fn move_up(&mut self);
    fn move_down(&mut self);
    fn move_to_relative_pos(&mut self, _x: i32, _y: i32);
    fn move_to_absolute_pos(&mut self, _x: i32, _y: i32);
}
pub struct Position {
   _x: i32,
   _y: i32,
   _z: Option<u8> // 0 -> down, 1 -> up
}

impl Position {
    pub fn update_position(&mut self, _new_x: i32, _new_y: i32, _new_z: Option<u8>) {
        self._x = _new_x;
        self._y = _new_y;
        self._z = _new_z;
    }

    pub fn get_x(&self) -> i32 {self._x}
    pub fn get_y(&self) -> i32 {self._y}
    pub fn get_z(&self) -> Option<u8> {self._z}

    pub fn set_x(&mut self, _x: i32) {self._x = _x;}
    pub fn set_y(&mut self, _y: i32) {self._y = _y;}
    pub fn set_z(&mut self, _z: Option<u8>) {self._z = _z;}
}

impl Default for Position {
    fn default() -> Self {
        Self {
            _x: 0,
            _y: 0,
            _z: Some(1)
        }
    }
}

pub trait Moveable {
    fn calibrate(&mut self);
    fn move_up(&mut self);
    fn move_down(&mut self);
    fn move_to_relative_pos(&mut self, _x: i32, _y: i32);
    fn move_to_absolute_pos(&mut self, _x: i32, _y: i32);
    fn get_current_pos(&self) -> &Position;
    // fn update_current_pos(&mut self, _x: i32, _y: i32, _z: i8);
}
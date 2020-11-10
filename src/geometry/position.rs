pub struct Position {
   _x: f32,
   _y: f32,
   _z: Option<u8> // 0 -> down, 1 -> up
}

impl Position {
    pub fn new(_x: f32, _y: f32, _z: Option<u8>) -> Self {
        Self {_x, _y, _z}
    }

    pub fn get_x(&self) -> f32 {self._x}
    pub fn get_y(&self) -> f32 {self._y}
    pub fn get_z(&self) -> Option<u8> {self._z}

    pub fn set_x(&mut self, _x: f32) {self._x = _x;}
    pub fn set_y(&mut self, _y: f32) {self._y = _y;}
    pub fn set_z(&mut self, _z: Option<u8>) {self._z = _z;}

    pub fn update_position(&mut self, _new_x: f32, _new_y: f32, _new_z: Option<u8>) {
        self._x = _new_x;
        self._y = _new_y;
        self._z = _new_z;
    }

    pub fn evaluate_relative_point(_start_point: &Self, _x_diff: f32, _y_dif: f32, _z_diff: Option<u8>) -> Position {
        Self::new(
            _start_point.get_x() + _x_diff,
            _start_point.get_y() + _y_dif,
            Some(_start_point.get_z().unwrap_or(0) + _z_diff.unwrap_or(0))
        )
    }
}

impl Default for Position {
    fn default() -> Self {
        Self {
            _x: 0.0,
            _y: 0.0,
            _z: Some(1)
        }
    }
}


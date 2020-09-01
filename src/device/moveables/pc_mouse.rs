use enigo::*;
use std::{thread, time};

use super::moveable::{Moveable};

pub struct PcMouseMoveable {
    pub _current_pos: (i32, i32),
    pub _move_driver: Enigo
}

impl PcMouseMoveable {
    pub fn new(_x: i32, _y: i32) -> Self {
        Self {
            _current_pos: (_x, _y),
            _move_driver: Enigo::new()
        }
    }
}

impl Moveable for PcMouseMoveable {
    fn calibrate(&mut self) {
        self._move_driver.mouse_move_to(self._current_pos.0, self._current_pos.1);
    }
    fn move_up(&mut self) { 
        let _left_btn = MouseButton::Left;
        self._move_driver.mouse_up(_left_btn);
    }

    fn move_down(&mut self) { 
        let _left_btn = MouseButton::Left;
        self._move_driver.mouse_down(_left_btn);
    }

    fn move_to_relative_pos(&mut self, _x: i32, _y: i32) { 
        let _to_x = _x - self._current_pos.0;
        let _to_y = _y - self._current_pos.1;
        println!("x: {}, y: {}", _to_x, _to_y);
        self._move_driver.mouse_move_relative(_to_x, _to_y);
        // TODO: update current pos
        self._current_pos.0 += _to_x;
        self._current_pos.1 += _to_y;
        thread::sleep(time::Duration::from_millis(2));
    }
    fn move_to_absolute_pos(&mut self, _: i32, _: i32) { todo!() }
}
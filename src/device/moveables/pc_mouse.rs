use enigo::*;
use std::{thread, time};

use super::moveable::{Moveable, Position};

pub struct PcMouseMoveable {
    pub _current_pos: Position,
    pub _move_driver: Enigo
}

impl PcMouseMoveable {
    pub fn new(_x: i32, _y: i32) -> Self {
        let mut _pos = Position::default();
        _pos.set_x(_x);
        _pos.set_y(_y);

        Self {
            _current_pos: _pos,
            _move_driver: Enigo::new()
        }
    }
}

impl Moveable for PcMouseMoveable {
    fn calibrate(&mut self) {
        self._move_driver.mouse_move_to(
            self._current_pos.get_x(), 
            self._current_pos.get_y()
        );
    }

    fn move_up(&mut self) { 
        let _left_btn = MouseButton::Left;
        self._move_driver.mouse_up(_left_btn);

        self._current_pos.update_position(
            self._current_pos.get_x(), 
            self._current_pos.get_y(),
            Some(1)
        );
    }

    fn move_down(&mut self) { 
        let _left_btn = MouseButton::Left;
        self._move_driver.mouse_down(_left_btn);

        self._current_pos.update_position(
            self._current_pos.get_x(), 
            self._current_pos.get_y(),
            Some(0)
        );
    }

    fn move_to_relative_pos(&mut self, _x: i32, _y: i32) { 
        let _to_x = _x - self._current_pos.get_x();
        let _to_y = _y - self._current_pos.get_y();

        self._move_driver.mouse_move_relative(_to_x, _to_y);

        self._current_pos.update_position(
            _to_x,
            _to_y,
            self._current_pos.get_z()
        );

        thread::sleep(time::Duration::from_millis(2));
    }

    fn move_to_absolute_pos(&mut self, _x: i32, _y: i32) { 
        self._move_driver.mouse_move_to(_x, _y);

        self._current_pos.update_position(
            _x,
            _y,
            self._current_pos.get_z()
        );

        thread::sleep(time::Duration::from_millis(2));
    }

    fn get_current_pos(&self) -> &Position {
        &self._current_pos
    }
}
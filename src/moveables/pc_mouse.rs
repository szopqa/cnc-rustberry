use enigo::*;
use std::{thread, time};

use super::{
    moveable::{Moveable}
};

use crate::geometry::{
    position::{Position, ZPosition},
    path
};

pub struct PcMouseMoveable {
    pub _current_pos: Position,
    pub _move_driver: Enigo
}

impl PcMouseMoveable {
    pub fn new(_x: f32, _y: f32) -> Self {
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
            self._current_pos.get_x() as i32, 
            self._current_pos.get_y() as i32
        );
    }

    fn move_up(&mut self) { 
        let _left_btn = MouseButton::Left;
        self._move_driver.mouse_up(_left_btn);

        self._current_pos.update_position(
            self._current_pos.get_x(), 
            self._current_pos.get_y(),
            ZPosition::Up
        );
    }

    fn move_down(&mut self) { 
        let _left_btn = MouseButton::Left;
        self._move_driver.mouse_down(_left_btn);

        self._current_pos.update_position(
            self._current_pos.get_x(), 
            self._current_pos.get_y(),
            ZPosition::Down
        );
    }

    fn move_to_relative_pos(&mut self, _x_path: f32, _y_path: f32) { 
        let _left_btn = MouseButton::Left;

        let _dest_pos = Position::evaluate_relative_point(
            &self._current_pos,
            _x_path,
            _y_path, 
            ZPosition::Down
        );
        self._move_driver.mouse_down(_left_btn);

        path::_line_from_two_positions(self.get_current_pos(), &_dest_pos).iter().for_each(|_p| {
            self.move_to_absolute_pos(&_p);

            self._current_pos.update_position(
                _p.get_x(),
                _p.get_y(),
                self._current_pos.get_z()
            );

            thread::sleep(time::Duration::from_millis(3));
        });
        self._move_driver.mouse_up(_left_btn);
    }

    fn move_to_absolute_pos(&mut self, _dest_pos: &Position) { 
        self._move_driver.mouse_move_to(_dest_pos.get_x() as i32, _dest_pos.get_y() as i32);

        self._current_pos.update_position(
            _dest_pos.get_x(),
            _dest_pos.get_y(),
            self._current_pos.get_z()
        );

        thread::sleep(time::Duration::from_millis(3));
    }

    fn get_current_pos(&self) -> &Position {
        println!("{}, {}, {:?}", self._current_pos.get_x(), self._current_pos.get_y(), self._current_pos.get_z());
        &self._current_pos
    }

    fn move_clockwise(&mut self, _dest_pos: &Position, _radius: Option<f32>) {
        let _arc_start_pos: &Position = self.get_current_pos();
        let _left_btn = MouseButton::Left;

        let _center_position: Position = Position::new(
            (_dest_pos.get_x() + _arc_start_pos.get_x()) / 2 as f32,
            (_dest_pos.get_y() + _arc_start_pos.get_y()) / 2 as f32,
            crate::geometry::position::ZPosition::Up
        );

        let _radius: i32 = match _radius {
            Some(_r) => _r as i32,
            _ => (_dest_pos.get_x()-_arc_start_pos.get_x()).abs() as i32 / 2
            
        };

        println!(
            "Drawing circle starting at {:?}, finishing at {:?}, with center point {:?} and radius {:?}",
            _arc_start_pos,
            _dest_pos,
            _center_position,
            _radius
        );

        self._move_driver.mouse_down(_left_btn);

        path::_circle_with_center_and_radius(&_center_position, _radius).iter().for_each(|_p| {
            self.move_to_absolute_pos(&_p);

            self._current_pos.update_position(
                _p.get_x(),
                _p.get_y(),
                self._current_pos.get_z()
            );

            thread::sleep(time::Duration::from_millis(3));
        });
        self._move_driver.mouse_up(_left_btn);
    }
}
use enigo::*;
use std::{thread, time};

use super::{
    moveable::{Moveable}
};

use crate::geometry::{
    position::{Position, ZPosition},
    path
};

pub struct DummyMoveable{
    pub _current_pos: Position,
}

impl  DummyMoveable {
    pub fn new(_x: f32, _y: f32) -> Self {
        println!("Initializing dummy moveable module");

        let mut _pos = Position::default();
        _pos.set_x(_x);
        _pos.set_y(_y);

        Self {
            _current_pos: _pos,
        }
    }
}

impl Moveable for  DummyMoveable {
    fn calibrate(&mut self) {
        println!("Executing 'calibrate' command");
    }

    fn move_up(&mut self) { 
        println!("Executing 'move_up' command");

        self._current_pos.update_position(
            self._current_pos.get_x(), 
            self._current_pos.get_y(),
            ZPosition::Up
        );
    }

    fn move_down(&mut self) { 
        println!("Executing 'move_down' command");

        self._current_pos.update_position(
            self._current_pos.get_x(), 
            self._current_pos.get_y(),
            ZPosition::Down
        );
    }

    fn move_to_relative_pos(&mut self, _x_path: f32, _y_path: f32) { 

        let _dest_pos = Position::evaluate_relative_point(
            &self._current_pos,
            _x_path,
            _y_path, 
            ZPosition::Down
        );

        println!("Executing  'move_to_relative_pos' command: ");
        path::_line_from_two_positions(self.get_current_pos(), &_dest_pos).iter().for_each(|_p| {
            self._current_pos.update_position(
                _p.get_x(),
                _p.get_y(),
                self._current_pos.get_z()
            );

            thread::sleep(time::Duration::from_millis(2));
        });

        println!("\t\t({:?}, {:?}, {:?})", self._current_pos.get_x(), self._current_pos.get_y(), self._current_pos.get_z());
    }

    fn move_to_absolute_pos(&mut self, _dest_pos: &Position) { 
        println!("Executing  'move_to_absolute_pos' command: ");
        println!("\t\t({:?}, {:?}, {:?})", _dest_pos.get_x(), _dest_pos.get_y(), self._current_pos.get_z());

        self._current_pos.update_position(
            _dest_pos.get_x(),
            _dest_pos.get_y(),
            self._current_pos.get_z()
        );

        thread::sleep(time::Duration::from_millis(2));
    }

    fn get_current_pos(&self) -> &Position {
        println!("({}, {}, {:?})", self._current_pos.get_x(), self._current_pos.get_y(), self._current_pos.get_z());
        &self._current_pos
    }

    fn move_clockwise(&mut self, _start_pos: &Position, _finish_pos: &Position, _radius: i32) {
        todo!()
    }
}
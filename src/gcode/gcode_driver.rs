use std::io;
use crate::moveables::{
    moveable::{Moveable}
};

use crate::geometry::{
    position::{Position}
};

use super::moves::{
    ArcMoveCommandData,
    MoveCommandData
};

#[derive(Debug)]
pub enum Command {
    RapidMove(MoveCommandData),
    LinearMove(MoveCommandData),
    ClockwiseArc(ArcMoveCommandData),
    CounterClockwiseArc(ArcMoveCommandData),
    Home,
    SetHome(MoveCommandData),
    SetInches,
    SetMillimeters,
    SetAbsolutePositioning,
    SetRelativePositioning
}

pub enum Unit {
    Inches,
    Millimeters
}

pub enum Positioning {
    Relative,
    Absolute
}

pub struct GCodeDriver {
    pub commands: Vec<Command>,
    pub home_position: Position,
    pub unit: Unit,
    pub positioning: Positioning,
    pub position: Position
}

impl Default for GCodeDriver {
    fn default() -> Self {
        Self {
            commands: vec![],
            home_position: Position::default(),
            unit: Unit::Millimeters,
            positioning: Positioning::Absolute,
            position: Position::default()
        }
    }
}

impl GCodeDriver {
    pub fn execute_commands(&mut self, _moveable: &mut Box<dyn Moveable>) -> Result <(), io::Error>{
        for _each_command in &self.commands {
            match _each_command {
                Command::RapidMove(_move_command) => {
                    let _pos = Position::from(_move_command);

                    match self.positioning {
                        Positioning::Relative => {
                            _moveable.move_to_relative_pos(_pos.get_x(), _pos.get_y());
                        }
                        Positioning::Absolute => {
                            _moveable.move_to_absolute_pos(&_pos);
                        }
                    }
                }
                // currently Rapid and Linear moves do not differ in implementation
                Command::LinearMove(_move_command) => {
                    let _pos = Position::from(_move_command);

                    match self.positioning {
                        Positioning::Relative => {
                            _moveable.move_to_relative_pos(_pos.get_x(), _pos.get_y());
                        },
                        Positioning::Absolute => {
                            _moveable.move_to_absolute_pos(&_pos);
                        }
                    }
                },
                Command::ClockwiseArc(_arc_move_command) => {
                    let _arc_start_pos: &Position = _moveable.get_current_pos();
                    let _arc_dest_position: Position = Position::new(
                        _arc_move_command.finish_point_x_pos.unwrap(),
                        _arc_move_command.finish_point_y_pos.unwrap(),
                        crate::geometry::position::ZPosition::Up
                    );
                },
                Command::CounterClockwiseArc(_arc_move_command) => {
                    todo!();
                },
                Command::Home => {
                    _moveable.move_to_absolute_pos(&self.home_position);
                }
                Command::SetHome(_move_command) => {
                    self.home_position = Position::from(_move_command);
                }
                Command::SetInches => {
                    self.unit = Unit::Inches;
                }
                Command::SetMillimeters => {
                    self.unit = Unit::Inches;
                }
                Command::SetAbsolutePositioning => {
                    self.positioning = Positioning::Absolute;
                }
                Command::SetRelativePositioning => {
                    self.positioning = Positioning::Relative;
                }
            }
        }

        Ok(())
    }
}
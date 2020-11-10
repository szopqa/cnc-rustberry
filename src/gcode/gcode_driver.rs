use std::io;
use crate::moveables::{
    moveable::{Moveable}
};

use crate::geometry::{
    position::{Position, ZPosition}
};

#[derive(Debug, Clone, Copy)]
pub struct MoveCommandData {
    pub x_axis: Option<f32>,
    pub y_axis: Option<f32>,
    pub z_axis: Option<f32>,
}

impl Default for MoveCommandData {
    fn default() -> Self { 
        Self {
            x_axis: None,
            y_axis: None,
            z_axis: None
        }
    }
}

impl From<&MoveCommandData> for Position {
    fn from(_command_data: &MoveCommandData) -> Self {
        Self::new(
            _command_data.x_axis.unwrap_or(0.0),
            _command_data.y_axis.unwrap_or(0.0),
            match _command_data.z_axis {
                Some(_z_val) => ZPosition::from(_z_val),
                None => ZPosition::Up // TODO: fix, taking Z position of last move containing Z position value
            },
        )
    }
}

#[derive(Debug)]
pub enum Command {
    RapidMove(MoveCommandData),
    LinearMove(MoveCommandData),
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
            println!("Executing: {:?}", _each_command);

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
                }
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
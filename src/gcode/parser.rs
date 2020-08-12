use std::fs::File;
use std::io::{self, prelude::*, BufReader};

pub struct Parser {
    gcode: String
}

pub struct MoveMetadata {
    x_axis: i32,
    y_axis: i32
}

pub enum GcodeCommand {
    RapidMove(MoveMetadata),
    LinearMove(MoveMetadata)
}

pub struct Command {
    command_type: GcodeCommand
}

impl Parser {
    pub fn new(gcode: String) -> Self {
        Parser {
            gcode
        }
    }

    pub fn parse(&self) -> Vec<Command> {
        vec![
            Command {
                command_type: GcodeCommand::LinearMove(
                    MoveMetadata {x_axis: 12, y_axis: 22}
                )
            }
        ]
    }
}
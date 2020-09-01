use std::fs::File;
use std::io::{BufRead, Lines, BufReader};

pub struct Parser<R: BufRead> {
    gcode_buf: Lines<R>
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

impl <R: BufRead> Parser<R> {
    pub fn new(gcode_buf: R) -> Self {
        Parser {
            gcode_buf: gcode_buf.lines()
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Command>, std::io::Error> {

        while let Some(_line) = self.gcode_buf.next() {
            println!("{:?}", _line);
        }

        Ok(vec![
            Command {
                command_type: GcodeCommand::LinearMove(
                    MoveMetadata {x_axis: 12, y_axis: 22}
                )
            }
        ])
    }
}
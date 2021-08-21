use std::io::{BufRead, Lines};
use super::gcode_driver::{
    Command,
    GCodeDriver
};

use super::moves::{
    IntoMoveData,
    MoveCommandData,
    ArcMoveCommandData
};

pub struct Parser<R: BufRead> {
    gcode_buf: Lines<R>
}

impl <R: BufRead> Parser<R> {
    pub fn new(gcode_buf: R) -> Self {
        Parser {
            gcode_buf: gcode_buf.lines()
        }
    }

    fn get_param_value(_param_value: &str) -> Option<f32> {
        match _param_value[1..].parse::<f32>() {
            Ok(_value) => { Some(_value) },
            _ => { None }
        }
    } 

    fn parse_data<'a, T>(_command: &str) -> T
    where T: Default  + IntoMoveData<'a> {
        let mut _arc_move_command_data: T = T::default();
        for _each_param in _command.split_ascii_whitespace() {
            _arc_move_command_data.parse_move_data(
                &_each_param[..1],
                Self::get_param_value(_each_param)
            )
        }

        _arc_move_command_data
    }

    fn parse_g_command(_command: &str) -> Command {
        match &_command.split_ascii_whitespace().next() {
            Some("G0") | Some("G00") => Command::RapidMove(Self::parse_data::<MoveCommandData>(_command)),
            Some("G1") | Some("G01") => Command::LinearMove(Self::parse_data::<MoveCommandData>(_command)),
            Some("G2") | Some("G02") => Command::ClockwiseArc(Self::parse_data::<ArcMoveCommandData>(_command)),
            Some("G3") | Some("G03") => Command::CounterClockwiseArc(Self::parse_data::<ArcMoveCommandData>(_command)),
            Some("G20") => Command::SetInches,
            Some("G21") => Command::SetMillimeters,
            Some("G28") => Command::Home,
            Some("G28.1") => Command::SetHome(Self::parse_data::<MoveCommandData>(_command)),
            Some("G90") => Command::SetAbsolutePositioning,
            Some("G91") => Command::SetRelativePositioning,
            _ => panic!(_command.to_string())
        }
    }

    fn get_command(_line: &str, _gcode_driver: &GCodeDriver) -> Option<Command> {
        match &_line[..1] {
            "G" => { 
                let _g_command = Self::parse_g_command(_line); 
                Some(_g_command)
            },
            "X" | "Y" | "Z" => { // TODO: Refactor parsing parameters for 'nested' commands
                let _last_command: &Command = _gcode_driver.commands.last().unwrap();
                match _last_command {
                    Command::RapidMove(_) => {
                        let _command_data = Self::parse_data::<MoveCommandData>(_line);
                        return Some(Command::RapidMove(_command_data));
                    },
                    Command::LinearMove(_) => {
                        let _command_data = Self::parse_data::<MoveCommandData>(_line);
                        return Some(Command::LinearMove(_command_data));
                    },
                    _ => unreachable!()
                }
            },
            _ => { None }
        }
    }

    fn parsing_error_for_line(_line_num: i64, _reason: &str) {
        println!("[Parser Debug] Skipping line: {}. Reason: {}", _line_num, _reason)
    }

    fn is_commented(_line: &str) -> bool {
        &_line[..1] == ";"
    }

    pub fn parse(&mut self) -> Result<GCodeDriver, std::io::Error> {
        let mut _gcode_driver: GCodeDriver = GCodeDriver::default();

        let mut _line_num: i64 = 1;
        while let Some(_line) = self.gcode_buf.next() {
            let _line = _line.unwrap();

            if _line.trim().is_empty() {
                Self::parsing_error_for_line(_line_num, "Line is empty");
                continue;
            } else if Self::is_commented(&_line) {
                Self::parsing_error_for_line(_line_num, "Line is commented");
                continue;
            }

            match Self::get_command(&_line, &_gcode_driver) {
                Some(_command) => {
                    _gcode_driver.commands.push(_command)
                },
                None => {
                    Self::parsing_error_for_line(_line_num, &format!("Unexpected command. Line: {}", _line));
                }
            }
        }

        println!("");

        Ok(_gcode_driver)
    }
}
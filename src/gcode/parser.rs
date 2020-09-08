use std::fs::File;
use std::io::{BufRead, Lines, BufReader};

pub struct Parser<R: BufRead> {
    gcode_buf: Lines<R>
}

#[derive(Debug)]
pub struct MoveCommandData {
    x_axis: Option<i32>,
    y_axis: Option<i32>,
    z_axis: Option<i32>,
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

#[derive(Debug)]
pub enum MoveCommand {
    RapidMove(MoveCommandData),
    LinearMove(MoveCommandData),
    Home(MoveCommandData)
}

#[derive(Debug)]
pub enum Command {
    Move(MoveCommand)
}

impl <R: BufRead> Parser<R> {
    pub fn new(gcode_buf: R) -> Self {
        Parser {
            gcode_buf: gcode_buf.lines()
        }
    }

    fn get_param_value(_param_value: &str) -> Option<i32> {
        match _param_value[1..].parse::<i32>() {
            Ok(_value) => { Some(_value) },
            _ => { None }
        }
    } 

    fn parse_g_command(_command: &str) -> MoveCommand {
        let mut _move_command_data: MoveCommandData = MoveCommandData::default();

        for _each_param in _command.split_ascii_whitespace() {
            match &_each_param[..1] {
                "X" => _move_command_data.x_axis = Self::get_param_value(_each_param),
                "Y" => _move_command_data.y_axis = Self::get_param_value(_each_param),
                "Z" => _move_command_data.z_axis = Self::get_param_value(_each_param),
                _ => ()
            }
        }

        match &_command[..3] {
            "G0 " | "G00" => MoveCommand::RapidMove(_move_command_data),
            "G1 " | "G01" => MoveCommand::LinearMove(_move_command_data),
            "G28" => MoveCommand::Home(_move_command_data),
            _ => panic!(_command[..3].to_string())
        }
    }

    fn get_command(_line: &str) -> Option<Command> {
        match &_line[..1] {
            "G" => { 
                let _g_command = Self::parse_g_command(_line); 
                Some(Command::Move(_g_command))
            },
            _ => { None }
        }
    }

    fn parsing_error_for_line(_line_num: i64, _reason: &str) {
        println!("Skipping line: {}. Reason: {}", _line_num, _reason)
    }

    fn is_commented(_line: &str) -> bool {
        &_line[..1] == ";"
    }

    pub fn parse(&mut self) -> Result<Vec<Command>, std::io::Error> {
        let mut _commands: Vec<Command> = vec![];

        let mut _line_num: i64 = 1;
        while let Some(_line) = self.gcode_buf.next() {
            let _line = _line.unwrap();
            println!("[DEBUG]: Processing line: {}", _line);

            if _line.trim().is_empty() {
                Self::parsing_error_for_line(_line_num, "Line is empty");
                continue;
            } else if Self::is_commented(&_line) {
                Self::parsing_error_for_line(_line_num, "Line is commented");
                continue;
            }

            match Self::get_command(&_line) {
                Some(_command) => _commands.push(_command),
                None => {
                    Self::parsing_error_for_line(_line_num, "Unexpected command");
                }
            }
        }

        println!("Commands: {:?}", _commands);

        Ok(_commands)
    }
}
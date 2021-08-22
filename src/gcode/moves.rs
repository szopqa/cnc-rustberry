use crate::geometry::{
    position::{Position, ZPosition}
};

pub trait IntoMoveData<'a> {
    fn parse_move_data(&mut self, _command_symbol: &str, _command_value: Option<f32>);
}

pub trait MoveCommand<'a> {
    fn contains_move_data(&self) -> bool;
}

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

impl MoveCommand<'_> for MoveCommandData {
    fn contains_move_data(&self) -> bool {
        self.x_axis.is_some() ||
            self.y_axis.is_some() || 
                self.z_axis.is_some()
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

impl IntoMoveData<'_> for MoveCommandData {
    fn parse_move_data(&mut self, _command_symbol: &str, _command_value: Option<f32>) {
        match _command_symbol {
            "X" => self.x_axis = _command_value,
            "Y" => self.y_axis = _command_value,
            "Z" => self.z_axis = _command_value,
            _ => ()
        }
    }
}

#[derive(Debug)]
pub struct ArcMoveCommandData {
    pub x_offset: Option<f32>, // I
    pub y_offset: Option<f32>, // J,
    pub finish_point_x_pos: Option<f32>, // X
    pub finish_point_y_pos: Option<f32>, // Y
    pub radius: Option<f32>
}

impl Default for ArcMoveCommandData {
    fn default() -> Self { 
        Self {
            x_offset: None,
            y_offset: None,
            finish_point_x_pos: None,
            finish_point_y_pos: None,
            radius: None,
        }
    }
}

impl MoveCommand<'_> for ArcMoveCommandData {
    fn contains_move_data(&self) -> bool {
        self.x_offset.is_some() || 
            self.y_offset.is_some() || 
                self.finish_point_x_pos.is_some() || 
                    self.finish_point_y_pos.is_some() || 
                        self.radius.is_some()
    }
}

impl IntoMoveData<'_> for ArcMoveCommandData {
    fn parse_move_data(&mut self, _command_symbol: &str, _command_value: Option<f32>) {
        match _command_symbol {
            "I" => self.x_offset = _command_value,
            "J" => self.y_offset = _command_value,
            "X" => self.finish_point_x_pos = _command_value,
            "Y" => self.finish_point_y_pos = _command_value,
            "R" => self.radius = _command_value,
            _ => ()
        }
    }
}
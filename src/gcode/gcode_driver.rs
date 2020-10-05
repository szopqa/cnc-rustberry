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

#[derive(Debug)]
pub enum Command {
    RapidMove(MoveCommandData),
    LinearMove(MoveCommandData),
    Home(MoveCommandData),
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
    pub home_position: MoveCommandData,
    pub unit: Unit,
    pub positioning: Positioning
}

impl Default for GCodeDriver {
    fn default() -> Self {
        Self {
            commands: vec![],
            home_position: MoveCommandData::default(),
            unit: Unit::Millimeters,
            positioning: Positioning::Absolute
        }
    }
}

impl GCodeDriver {
    pub fn set_to_relative(&mut self) {
        self.positioning = Positioning::Relative;
    }

    pub fn set_to_absolute(&mut self) {
        self.positioning = Positioning::Absolute;
    }

    pub fn set_unit_to_millimeters(&mut self) {
        self.unit = Unit::Millimeters;
    }

    pub fn set_unit_to_inches(&mut self) {
        self.unit = Unit::Inches;
    }
}
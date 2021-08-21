pub trait Stepper {
    fn rotate_clockwise(&self);
    fn rotate_counter_clockwise(&self);
}

pub enum SteppingDirection {
    CW,
    CCW,
}
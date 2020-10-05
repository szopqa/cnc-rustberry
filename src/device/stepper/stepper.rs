pub trait Stepper {
    fn rotate_clockwise(&self);
    fn rotate_counter_clockwise(&self);
}

pub enum SteppingMode {
    ClockwiseHalfStep,
    CounterClockwiseHalfStep,

    ClockwiseStep,
    CounterClockwiseStep,
}
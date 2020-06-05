use rppal::gpio::OutputPin;
use std::time::Duration;

pub trait Servo {
    fn move_up(&mut self) -> Result<(), String>;
    fn move_down(&mut self) -> Result<(), String>;
}
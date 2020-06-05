use rppal::gpio::OutputPin;
use std::thread;
use std::time::Duration;

use super::servo::Servo;

pub struct Sg90 {
    pulse_cycle_ms: u64,
    pulse_width_min: u64,
    pulse_width_max: u64,
    pub is_moving: bool,
    output_pin: OutputPin
}

impl Sg90 {
    pub fn new(output_pin: OutputPin) -> Self {
        Sg90 {
            pulse_cycle_ms: 20, // 50HZ
            pulse_width_min: 500,
            pulse_width_max: 2400,
            is_moving: false,
            output_pin
        }
    }
} 

impl Servo for Sg90 {
    fn move_up(&mut self) -> Result<(), String> {
        match self.output_pin.set_pwm(
            Duration::from_millis(self.pulse_cycle_ms),
            Duration::from_micros(self.pulse_width_max),
        ) {
            Ok(_) => {
                self.is_moving = true;
                println!("Finished move_up");
                self.is_moving = false;
                Ok(())
            },
            Err(_) => Err(String::from("Error moving servo up"))
        }
    }

    fn move_down(&mut self) -> Result<(), String> {
        match self.output_pin.set_pwm(
            Duration::from_millis(self.pulse_cycle_ms),
            Duration::from_micros(self.pulse_width_min),
        ) {
            Ok(_) => {
                self.is_moving = true;
                println!("Finished move_down");
                self.is_moving = false;
                Ok(())
            },
            Err(_) => Err(String::from("Error moving servo down"))
        }
    }
}
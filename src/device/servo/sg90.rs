use super::servo::servo;
use rppal::gpio::OutputPin;
use std::time::Duration;

pub struct sg90 {
    pulse_cycle_ms: u64,
    pulse_width_min: u64,
    pulse_width_max: u64,
    output_pin: OutputPin
}

impl sg90 {
    pub fn new(output_pin: OutputPin) -> Self {
        sg90 {
            pulse_cycle_ms: 20, // 50HZ
            pulse_width_min: 500,
            pulse_width_max: 2400,
            output_pin
        }
    }
} 

impl servo for sg90 {
    fn move_up(&mut self) -> Result<(), String> {
        match self.output_pin.set_pwm(
            Duration::from_millis(self.pulse_cycle_ms),
            Duration::from_micros(self.pulse_width_max),
        ) {
            Ok(_) => Ok(()),
            Err(_) => Err(String::from("Error moving servo up"))
        }
    }

    fn move_down(&mut self) -> Result<(), String> {
        match self.output_pin.set_pwm(
            Duration::from_millis(self.pulse_cycle_ms),
            Duration::from_micros(self.pulse_width_min),
        ) {
            Ok(_) => Ok(()),
            Err(_) => Err(String::from("Error moving servo down"))
        }
    }
}
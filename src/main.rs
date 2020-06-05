use std::io::{self, Read};
use std::error::Error;
use rppal::gpio::Gpio;

use std::thread;
use std::time::Duration;

mod device;
use device::{
    servo::{
        servo::{Servo},
        sg90::{Sg90}
    },
    cnc_plotter_device::{
        CncPlotterDevice
    }
};

const GPIO_PWM: u8 = 23;

fn main() -> Result<(), Box<dyn Error>> {
    let servo_pwm_pin = Gpio::new()?.get(GPIO_PWM)?.into_output();
    let mut servo = Sg90::new(servo_pwm_pin);

    let mut cnc_plotter = CncPlotterDevice::new(Box::new(servo));

    cnc_plotter.run();

    Ok(())
}
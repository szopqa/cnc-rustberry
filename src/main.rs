
use std::error::Error;
use rppal::gpio::Gpio;

use std::thread;
use std::time::Duration;

mod device;
use device::servo::{
    servo::{servo},
    sg90::{sg90}
};

const GPIO_PWM: u8 = 23;

fn main() -> Result<(), Box<dyn Error>> {
    let servo_pwm_pin = Gpio::new()?.get(GPIO_PWM)?.into_output();

    let mut servo = sg90::new(servo_pwm_pin);

    servo.move_down()?;
    servo.move_up()?;

    Ok(())
}
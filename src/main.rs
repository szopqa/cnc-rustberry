use std::io::{self, Read};
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

    let MANUAL_SERVO_DOWN_INPUT: String = String::from("d\n");
    let MANUAL_SERVO_UP_INPUT: String = String::from("u\n");

    let mut buffer = String::new();
    loop {
        std::io::stdin().read_line(&mut buffer).unwrap();
        if buffer == MANUAL_SERVO_DOWN_INPUT {
            servo.move_down().unwrap();
        } else if buffer == MANUAL_SERVO_UP_INPUT {
            servo.move_up().unwrap();
        } else {
            println!("Manual stearing input: {}", buffer);
        }
        buffer = String::new();
    } 

    Ok(())
}
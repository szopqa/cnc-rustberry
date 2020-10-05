use super::stepper::{SteppingMode};
use rppal::gpio::{Gpio, OutputPin};
use std::{thread, time};

// Stepper driver data sheet: https://www.st.com/resource/en/datasheet/uln2001.pdf
pub struct UlnStepperStepperDriver {
    output_pins: [u8;4],
    move_seq: [[u8;4];8]
}

impl UlnStepperStepperDriver {
    pub fn new() -> Self {
        Self {
            output_pins: [17, 22, 23, 24],
            move_seq: [
                [0,0,0,1], //  A
                [0,0,1,1], //  AB
                [0,0,1,0], //  B
                [0,1,1,0], //  BC
                [0,1,0,0], //  C
                [1,1,0,0], //  CD
                [1,0,0,0], //  D
                [1,0,0,1], //  DA
            ]
        }
    }

    pub fn rotate(&self, _stepping_mode: SteppingMode, _max_steps: usize) {
        let mut steps: isize = 0;

        let mut _gpio_pins: Vec<OutputPin> = (0..4).map(|_pin_index| {
            Gpio::new().unwrap().get(self.output_pins[_pin_index]).unwrap().into_output()
        }).collect();

        let dir: isize = match _stepping_mode {
            SteppingMode::CounterClockwiseHalfStep => 1,
            SteppingMode::CounterClockwiseStep => 2,

            SteppingMode::ClockwiseHalfStep => -1,
            SteppingMode::ClockwiseStep => -2,
        };

        let mut _steps_counter = 0;

        loop {
            println!("Step: {}", steps);
            println!("command: {:?}", self.move_seq[steps as usize]);

            for _each_pin_index in 0..4 {
                let _output_pin = &mut _gpio_pins[_each_pin_index];

                if self.move_seq[steps as usize][_each_pin_index] != 0 {
                    _output_pin.set_high();
                } else {
                    _output_pin.set_low();
                }
            }

            steps = steps + dir;

            if steps >= self.move_seq.len() as isize {
                steps = 0;
            } 

            if steps < 0 {
                // println!("Got: {}", self.move_seq.len() + dir as usize);
                steps = self.move_seq.len() as isize + dir;
            }

            thread::sleep(time::Duration::from_millis(1));

            if (_max_steps != 0 && _steps_counter == _max_steps) {
                break;
            }
            _steps_counter += 1;
        }
    }
}

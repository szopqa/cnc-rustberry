use super::stepper::{SteppingDirection};
use rppal::gpio::{Gpio, OutputPin};
use std::{thread, time};

// Stepper driver data sheet: https://www.st.com/resource/en/datasheet/uln2001.pdf
pub struct UlnStepperStepperDriver {
    output_pins: Vec<OutputPin>,
    move_seq: [[u8;4];8],
    steps_for_full_rotation: usize
}

impl UlnStepperStepperDriver {
    pub fn new(_gpio_output_pins: [u8;4]) -> Self {
        Self {
            output_pins: _gpio_output_pins.into_iter().map(|_pin| {Gpio::new().unwrap().get(*_pin).unwrap().into_output()}).collect(),
            move_seq: [
                [0,0,0,1], //  A
                [0,0,1,1], //  AB
                [0,0,1,0], //  B
                [0,1,1,0], //  BC
                [0,1,0,0], //  C
                [1,1,0,0], //  CD
                [1,0,0,0], //  D
                [1,0,0,1], //  DA
            ],
            steps_for_full_rotation: 4096
        }
    }

    fn evaluate_step (_stepping_mode: SteppingDirection) -> isize {
        match _stepping_mode {
            SteppingDirection::CW => -1,
            SteppingDirection::CCW => 1,
        }
    }

    pub fn rotate_full_circle(&mut self, _stepping_mode: SteppingDirection) {
        self.rotate_for_angle(360.0, _stepping_mode);
    }

    pub fn rotate_for_angle(&mut self, _angle: f32, _stepping_mode: SteppingDirection) {
        let _steps = (self.steps_for_full_rotation * _angle as usize) / 360;

        let mut step: isize = 0;

        let dir: isize = Self::evaluate_step(_stepping_mode);

        let mut _steps_counter = 0;

        loop {
            for _each_pin_index in 0..4 {
                let _output_pin = &mut self.output_pins[_each_pin_index];

                if self.move_seq[step as usize][_each_pin_index] != 0 {
                    _output_pin.set_high();
                } else {
                    _output_pin.set_low();
                }
            }

            step = step + dir;

            if step >= self.move_seq.len() as isize {
                step = 0;
            } 

            if step < 0 {
                step = self.move_seq.len() as isize + dir;
            }

            thread::sleep(time::Duration::from_millis(1));

            if (_steps_counter == _steps) {
                break;
            }
            _steps_counter += 1;
        }
    }
}

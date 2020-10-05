use super::stepper::Stepper;
use rppal::gpio::Gpio;

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

    pub fn rotate(&self, dir: isize) {
        let mut steps = 0;

        loop {
            println!("Step: {}", steps);
            println!("command: {:?}", self.move_seq[steps]);

            for _each_pin_index in 0..=4 {
                let mut xpin = Gpio::new().unwrap().get(self.output_pins[_each_pin_index]).unwrap().into_output();

                if self.move_seq[steps][_each_pin_index] != 0 {
                    xpin.set_high();
                } else {
                    xpin.set_low();
                }
            }

            steps += 1;

            if steps >= self.move_seq.len() {
                steps = 0;
            } else {
                steps = steps + dir as usize;
            }
        }
    }
}
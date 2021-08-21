use super::servo::{servo::Servo};

pub struct CncPlotterDevice {
    pub marker_controller: Box<dyn Servo>
}

impl CncPlotterDevice {
    pub fn new(_marker_servo: Box<dyn Servo>) -> Self {
        CncPlotterDevice {
            marker_controller: _marker_servo
        }
    }

    pub fn run(&mut self) -> () {
        let MANUAL_SERVO_DOWN_INPUT: String = String::from("d\n");
        let MANUAL_SERVO_UP_INPUT: String = String::from("u\n");

        let mut buffer = String::new();
        loop {
            std::io::stdin().read_line(&mut buffer).unwrap();
            if buffer == MANUAL_SERVO_DOWN_INPUT {
                self.marker_controller.move_down().unwrap();
            } else if buffer == MANUAL_SERVO_UP_INPUT {
                self.marker_controller.move_up().unwrap();
            } else {
                println!("Manual stearing input: {}", buffer);
            }
            buffer = String::new();
        } 
    }
}
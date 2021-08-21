use std::error::Error;
use std::fs::File;
use std::io::{BufReader};

mod gcode;
use gcode::parser::Parser;

mod device;
use device::{
    stepper::{stepper::{SteppingDirection}, uln2003::UlnStepperStepperDriver}
};

mod moveables;
use moveables::{moveable::Moveable, pc_mouse::PcMouseMoveable, dummy::DummyMoveable};

mod geometry;


fn test_hardware() {
    const GPIO_PWM: u8 = 23;
    const X_AXIS_STEPPER_PINS: [u8; 4] = [19, 26, 20, 21];
    const Y_AXIS_STEPPER_PINS: [u8; 4] = [17, 22, 23, 24];

    // stepper motor initialization
    let mut x_axis_stepper = UlnStepperStepperDriver::new(X_AXIS_STEPPER_PINS);
    let mut y_axis_stepper = UlnStepperStepperDriver::new(Y_AXIS_STEPPER_PINS);

    loop {
        x_axis_stepper.rotate_for_angle(2.0, SteppingDirection::CW);
        y_axis_stepper.rotate_for_angle(2.0, SteppingDirection::CCW);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let _gcode: BufReader<File> = BufReader::new(File::open(String::from("pc_plot.g"))?);
    
    let mut parser = Parser::new(_gcode);

    let mut _gcode_driver = parser.parse()?; println!("");

    let mut _pc_mouse: Box<dyn Moveable> = Box::new(PcMouseMoveable::new(200.0, 200.0));
    let mut _dummy_moveable: Box<dyn Moveable> = Box::new(DummyMoveable::new(0.0, 0.0));

    _gcode_driver.execute_commands(&mut _pc_mouse)?;
    // _gcode_driver.execute_commands(&mut _dummy_moveable)?;


    /*
        Currently supported only on raspberry

        test_hardware();
    */

    Ok(())
}

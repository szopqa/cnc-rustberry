use std::error::Error;
use std::fs::File;
use std::io::{BufReader};
extern crate bresenham;
use bresenham::Bresenham;


mod gcode;
use gcode::parser::Parser;

mod device;
use device::{
    cnc_plotter_device::CncPlotterDevice,
    servo::{servo::Servo, sg90::Sg90},
    stepper::{stepper::{Stepper, SteppingDirection}, uln2003::UlnStepperStepperDriver}
};

mod moveables;
use moveables::{moveable::Moveable, pc_mouse::PcMouseMoveable};

mod geometry;
use geometry::{
    position::Position,
    path
};

struct DrawablePlatform {
    x_pos: i32,
    y_pos: i32,
    x_resolution: i32,
    y_resolution: i32,
    assets: Vec<(isize, isize)>,
}

impl DrawablePlatform {
    pub fn new(x_resolution: i32, y_resolution: i32) -> Self {
        Self {
            x_pos: 1,
            y_pos: 1,
            x_resolution,
            y_resolution,
            assets: vec![],
        }
    }

    pub fn draw_line(&mut self, to_x: i32, to_y: i32) -> () {
        let mut x_addon: isize = 0; // additional range applied to x destination
        let mut y_addon: isize = 0; // additional range applied to y destination

        if self.x_pos < to_x {
            x_addon = 1;
        } else if self.x_pos > to_x {
            x_addon = -1;
        }

        if self.y_pos < to_y {
            y_addon = 1;
        } else if self.y_pos > to_y {
            y_addon = -1;
        }

        for (x, y) in Bresenham::new(
            (self.x_pos as isize, self.y_pos as isize),
            (to_x as isize + x_addon, to_y as isize + y_addon),
        ) {
            self.assets.push((x, y));
        }

        let (x_current, y_current) = self.assets.last().copied().unwrap();
        self.x_pos = x_current as i32;
        self.y_pos = y_current as i32;
    }

    pub fn draw(&self) {
        for y in 1..=self.y_resolution {
            for x in 1..=self.x_resolution {
                if self.assets.contains(&(x as isize, y as isize)) {
                    print!("%");
                } else {
                    print!("*");
                }
                if x  == self.x_resolution {
                    println!("");
                }
            }
        }
    }
}

const GPIO_PWM: u8 = 23;
const X_AXIS_STEPPER_PINS: [u8; 4] = [19, 26, 20, 21];
const Y_AXIS_STEPPER_PINS: [u8; 4] = [17, 22, 23, 24];

fn test_hardware() {
    // stepper motor initialization
    let mut x_axis_stepper = UlnStepperStepperDriver::new(X_AXIS_STEPPER_PINS);
    let mut y_axis_stepper = UlnStepperStepperDriver::new(Y_AXIS_STEPPER_PINS);

    loop {
        x_axis_stepper.rotate_for_angle(2.0, SteppingDirection::CW);
        y_axis_stepper.rotate_for_angle(2.0, SteppingDirection::CCW);
    }
}

fn draw_using_pc_mouse() {
    let mut pc_mouse = PcMouseMoveable::new(200.0, 200.0);
    pc_mouse.calibrate();

    // pc_mouse.move_down();
    let destinations: Vec<(isize, isize)> = vec![
        (200, 600),
        (600, 200),
        (600, 600),
        (200, 200),
    ];

    for _offset in 0..100 {
        for _each_dest in &destinations {
            for (x, y) in Bresenham::new(
                (pc_mouse._current_pos.get_x() as isize, 
                 pc_mouse._current_pos.get_y() as isize),
                (_each_dest.0, _each_dest.1),
            ) {
                pc_mouse.move_to_absolute_pos(x as f32, y as f32);
            }
        }
    }

    // pc_mouse.move_up();
}

fn main() -> Result<(), Box<dyn Error>> {
    let _gcode: BufReader<File> = BufReader::new(File::open(String::from("pc_plot.g"))?);
    
    let mut parser = Parser::new(_gcode);

    let mut _gcode_driver = parser.parse()?;

    let mut _pc_mouse: Box<dyn Moveable> = Box::new(PcMouseMoveable::new(200.0, 200.0));

    _gcode_driver.execute_commands(&mut _pc_mouse)?;

    // let _pos_a = Position::new(0,0,Some(0));
    // let _pos_b = Position::new(20,2,Some(0));

    // path::_line_from_two_positions(&_pos_a,& _pos_b).iter().for_each(|_pos| {
    //     println!("{}, {}, {}", _pos.get_x(), _pos.get_y(), _pos.get_z().unwrap_or(0));
    // });

    // draw_using_pc_mouse();

    /*
        Currently supported only on raspberry
    */

    // test_hardware();


    /* 
        Supported only outside raspberry
    */

    // draw_using_pc_mouse();

    Ok(())
}

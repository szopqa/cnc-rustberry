use rppal::gpio::Gpio;
use std::error::Error;
use std::fs::File;
use std::io::{self, prelude::*, BufReader, Read};
extern crate bresenham;
use bresenham::Bresenham;

use std::thread;
use std::time::Duration;

mod gcode;
use gcode::parser::Parser;

mod device;
use device::{
    cnc_plotter_device::CncPlotterDevice,
    servo::{servo::Servo, sg90::Sg90},
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

fn main() -> Result<(), Box<dyn Error>> {
    // let g_code = File::open("plot.g")?;

    // let g_code = BufReader::new(g_code);
    // for line in g_code.lines() {
    //     println!("{}", line.unwrap());
    // }

    // let mut g_code_parser = Parser::new(
    //     r"
    //     G90 N1
    //     G
    //     G01 X50 $$%# Y20
    // "
    //     .to_string(),
    // );

    // let commands = g_code_parser.parse();

    // let servo_pwm_pin = Gpio::new()?.get(GPIO_PWM)?.into_output();
    // let mut servo = Sg90::new(servo_pwm_pin);

    // let mut cnc_plotter = CncPlotterDevice::new(Box::new(servo));

    // cnc_plotter.run();
    let mut drawable = DrawablePlatform::new(50, 50);

    drawable.draw_line(1, 50);
    drawable.draw_line(50, 50);
    drawable.draw_line(50, 1);
    drawable.draw_line(1, 1);
    drawable.draw_line(50, 50);
    drawable.draw_line(50, 1);
    drawable.draw_line(1, 50);
    drawable.draw_line(40, 30);

    drawable.draw();

    Ok(())
}

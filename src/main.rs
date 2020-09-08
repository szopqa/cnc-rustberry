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
    moveables::{moveable::Moveable, pc_mouse::PcMouseMoveable}
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
    let _g_code = File::open(String::from("plot.g"))?;
    let _g_code: BufReader<File> = BufReader::new(_g_code);
    
    let mut parser = Parser::new(_g_code);
    let _commands = parser.parse().unwrap();

    // let mut pc_mouse = PcMouseMoveable::new(200, 200);
    // pc_mouse.calibrate();

    // pc_mouse.move_down();
    // let destinations: Vec<(isize, isize)> = vec![
    //     (200, 600),
    //     (600, 200),
    //     (600, 600),
    //     (200, 200),
    // ];

    // for _offset in 0..100 {
    //     for _each_dest in &destinations {
    //         for (x, y) in Bresenham::new(
    //             (pc_mouse._current_pos.0 as isize, 
    //              pc_mouse._current_pos.1 as isize),
    //             (_each_dest.0 + _offset * 10, _each_dest.1 + _offset * 10),
    //         ) {
    //             pc_mouse.move_to_relative_pos(x as i32, y as i32)
    //         }
    //     }
    // }

    // pc_mouse.move_up();

    Ok(())
}

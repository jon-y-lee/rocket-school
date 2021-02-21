#[path = "util/constants.rs"]
mod util;
// extern crate sysfs_gpio;

mod orientation_task_initializer;
mod orientation;

use std::thread;
use std::time::Duration;

fn main() {
    println!("Orientation Task Initializer");

    println!("{}", util::RAD_TO_DEG);

    // let mut orientationTask =
    //     Orientation::orientation::initialize();

    // let mut an = Angle::Angle{x: 12.1, y: 0.0, dateTime: 0};

    // Angle::angle::Angle::setX(  4.0);
    // an::setX(12.1);
    // an.set_x(4.0);
    // println!("{} angle:", an.x);

    // let handle =
    orientation_task_initializer::initialize();
}

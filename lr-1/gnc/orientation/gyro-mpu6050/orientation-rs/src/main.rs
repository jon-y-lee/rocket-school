#[path = "util/constants.rs"] mod util;
#[path = "orientation/Orientation.rs"] mod Orientation;
#[path = "orientation/Angle.rs"] mod Angle;
extern crate sysfs_gpio;

// use util::RAD_TO_DEG;
// mod util/constants;
mod OrientationTaskInitializer;
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
    OrientationTaskInitializer::initialize();
}

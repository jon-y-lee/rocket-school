use crate::orientation::processor;
use i2cdev::*;
use linux_embedded_hal::{I2cdev, Delay};
// use processor::orientation_processor;

use std::{thread, time};
use crate::orientation::processor::orientation_processor;
use crate::orientation::processor::processor::SignalProcessor;

pub fn initialize() {

    let i2c = I2cdev::new("/dev/i2c-1");
        // .map_err(orientation_task_initializer::linux::LinuxI2CError);

    let dev = i2c.unwrap();

    let mut proc = orientation_processor::Processor::new(dev);

    proc.init();

    println!("Sleeping for 1s");

    let second = time::Duration::from_millis(20);
    thread::sleep(second);

    thread::spawn(move || loop {
        proc.read();
        thread::sleep(second);
    }).join().map_err(|err| println!("{:?}", err)).ok();
}

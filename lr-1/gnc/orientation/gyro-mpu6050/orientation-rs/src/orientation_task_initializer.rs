use crate::orientation::processor;
use i2cdev::*;
use linux_embedded_hal::{I2cdev, Delay};
use processor::orientation_processor::Processor;

use std::{thread, time};

pub fn initialize() {

    let i2c = I2cdev::new("/dev/i2c-1");
        // .map_err(orientation_task_initializer::linux::LinuxI2CError);

    let dev = i2c.unwrap();

    let mut proc = Processor::new(dev);

    proc.wake();

    println!("Sleeping for 1s");

    let second = time::Duration::from_millis(1000);
    thread::sleep(second);


    // proc::test();
    // processor::orientation_processor::Processor::test(&proc);

    // processor::orientation_processor::start();

    thread::spawn(move || loop {
        proc.read();
        thread::sleep(second);
        // processor::orientation_processor::::orientation_processor::read();
    }).join().map_err(|err| println!("{:?}", err)).ok();
}

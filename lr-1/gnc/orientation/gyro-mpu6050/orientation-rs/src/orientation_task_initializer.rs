use crate::orientation::processor;
use i2cdev::*;
use linux_embedded_hal::{I2cdev, Delay};

use std::{thread, time};
use crate::orientation::processor::orientation_processor;
use crate::orientation::processor::accelerometer_processor;
use crate::orientation::processor::processor::SignalProcessor;
use crate::orientation::processor::temperature_processor;

pub fn initialize() {

    let i2c = I2cdev::new("/dev/i2c-1");
        // .map_err(orientation_task_initializer::linux::LinuxI2CError);

    let dev = i2c.unwrap();

    let mut i2cProcessor = orientation_processor::I2CProcessor::new(dev);

    i2cProcessor.init();

    let second = time::Duration::from_millis(1000);
    let millis = time::Duration::from_millis(100);

    thread::sleep(second);
    i2cProcessor.verify();

    // let mut accelerometerProcessor
    //     = accelerometer_processor::AccelerometerProcessor::new(i2cProcessor);

    let mut tempProcessor
        = temperature_processor::TemperatureProcessor::new(i2cProcessor);

    // thread::spawn(move || loop {
    //     accelerometerProcessor.read();
    //     thread::sleep(second);
    // }).join().map_err(|err| println!("{:?}", err)).ok();

    thread::spawn(move || loop {
        tempProcessor.read();
        thread::sleep(second);
    }).join().map_err(|err| println!("{:?}", err)).ok();

}

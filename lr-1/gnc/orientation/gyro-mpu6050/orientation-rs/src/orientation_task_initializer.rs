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
    let mut i2cProcessor2 = orientation_processor::I2CProcessor::new(I2cdev::new("/dev/i2c-1").unwrap());

    i2cProcessor.init();
    i2cProcessor2.init();
    let second = time::Duration::from_millis(1000);
    let millis = time::Duration::from_millis(100);

    thread::sleep(second);
    i2cProcessor.verify();
    i2cProcessor2.verify();

    let mut accelerometerProcessor
        = accelerometer_processor::AccelerometerProcessor::new(i2cProcessor);

    let mut tempProcessor
        = temperature_processor::TemperatureProcessor::new(i2cProcessor2);

    tempProcessor.init();

    thread::spawn(move || loop {
        accelerometerProcessor.read();
        thread::sleep(second);
    });

    println!("Starting temp thread");
    thread::spawn(move || loop {
        println!("!Temp");
        tempProcessor.read();
        thread::sleep(second);
    }).join().map_err(|err| println!("{:?}", err)).ok();

}

// use mpu6050::*;
// use linux_embedded_hal::{I2cdev, Delay};
// use i2cdev::linux::LinuxI2CError;
// use gpio::{GpioIn, GpioOut};
// use sysfs_gpio::{Direction, Pin};

pub fn start() {
    println!("Initialization started");

    // let mut gpio68 = gpio::sysfs::SysFsGpioInput::open(68).unwrap();

    // let input = Pin::new(68);
    // input.with_exported(|| {
    //     // input.set_direction(Direction::In)?;
    //     // let mut prev_val: u8 = 255;
    //     loop {
    //         let val = input.get_value()?;
    //         println!("val: {:?}", val);
    //         // if val != prev_val {
    //         //     println!("Pin State: {}", if val == 0 { "Low" } else { "High" });
    //         //     prev_val = val;
    //         // }
    //         std::thread::sleep(std::time::Duration::from_millis(1000));
    //     }
    // });

    std::thread::sleep(std::time::Duration::from_millis(1000));
}

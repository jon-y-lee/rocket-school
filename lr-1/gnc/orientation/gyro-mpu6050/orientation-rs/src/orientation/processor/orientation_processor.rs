// pub mod processor {

    use mpu6050::*;
    // use linux_embedded_hal::{I2cdev, Delay};
    // use i2cdev::linux::LinuxI2CError;
    use gpio::{GpioIn, GpioOut};

    pub fn start() {
        println!("Initialization started");

        // let mut gpio68 = gpio::sysfs::SysFsGpioInput::open(68).unwrap();

        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
// }

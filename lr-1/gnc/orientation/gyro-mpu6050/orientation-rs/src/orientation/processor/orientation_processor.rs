use linux_embedded_hal::I2cdev;
use embedded_hal::{
    blocking::delay::DelayMs,
    blocking::i2c::{Write, WriteRead},
};
use std::*;
use std::io;
use nalgebra::{Vector3, Vector2};
use libm::{powf, atan2f, sqrtf};
use crate::util::RAD_TO_DEG;
use crate::orientation::processor::processor::SignalProcessor;

/// Slave address of Mpu6050
pub const SLAVE_ADDR: u8 = 0x68;
/// Internal register to check slave addr
pub const WHOAMI: u8 = 0x75;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
/// Register 107: Power Management 1
pub struct PWR_MGMT_1;

/// All possible errors in this crate
#[derive(Debug)]
pub enum MPUError {
    /// I2C bus error
    I2c,

    /// Invalid chip ID was read
    InvalidChipId(u8),
}

impl PWR_MGMT_1 {
    /// Base Address
    pub const ADDR: u8 = 0x6b;
    /// Device Reset bit
    pub const DEVICE_RESET: u8 = 7;
    /// Sleep mode bit (Should be called "Low Power", doesn't actually sleep)
    pub const SLEEP: u8 = 6;
    /// Cycle bit for wake operations
    pub const CYCLE: u8 = 5;
    /// Temperature sensor enable/disable bit
    pub const TEMP_DIS: u8 = 3;

    // Clock Control
    // pub const CLKSEL: BitBlock = BitBlock { bit: 2, length: 3 };
}

/// Describes a bit block from bit number 'bit' to 'bit'+'length'
pub struct BitBlock {
    pub bit: u8,
    pub length: u8
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
/// Register 27: Gyro Config
pub struct GYRO_CONFIG;

impl GYRO_CONFIG {
    pub const ADDR: u8 = 0x1b;
    /// Gyro x axis self test bit
    pub const XG_ST: u8 = 7;
    /// Gyro y axis self test bit
    pub const YG_ST: u8 = 6;
    /// Gyro z axis self test bit
    pub const ZG_ST: u8 = 5;
    /// Gyro Config FS_SEL
    pub const FS_SEL: BitBlock = BitBlock { bit: 4, length: 2 };
}
/// PI, f32
pub const PI: f32 = core::f32::consts::PI;

/// PI / 180, for conversion to radians
pub const PI_180: f32 = PI / 180.0;

/// High Byte Register Gyro x orientation
pub const GYRO_REGX_H: u8 = 0x43;
/// High Byte Register Gyro y orientation
pub const GYRO_REGY_H: u8 = 0x45;
/// High Byte Register Gyro z orientation
pub const GYRO_REGZ_H: u8 = 0x47;

pub const GYRO_SENS: (f32, f32, f32, f32) = (131., 65.5, 32.8, 16.4);
pub const ACCEL_SENS: (f32, f32, f32, f32) = (16384., 8192., 4096., 2048.);

/// High Byte Register Calc roll
pub const ACC_REGX_H : u8= 0x3b;

/// Handles all operations on/with Mpu6050
pub struct I2CProcessor {
    i2c: I2cdev,
}

impl SignalProcessor for I2CProcessor {
    fn new(i2c: I2cdev) -> Self {
        I2CProcessor {
            i2c,
        }
    }

    fn init(&mut self) {
        // MPU6050 has sleep enabled by default -> set bit 0 to wake
        // Set clock source to be PLL with x-axis gyroscope reference, bits 2:0 = 001 (See Register Map )
        self.write_byte(PWR_MGMT_1::ADDR, 0x01);
        println!("test test");
    }

    fn verify(&mut self) {
        let address = self.read_byte(WHOAMI);
        if address.unwrap() != SLAVE_ADDR {
            println!("Not verified");
        }
    }

    fn write_byte(&mut self, reg: u8, byte: u8) {
        self.i2c.write(SLAVE_ADDR, &[reg, byte]);
    }

    fn read_byte(&mut self, reg: u8) -> Result<u8, MPUError> {
        let mut byte: [u8; 1] = [0; 1];
        self.i2c.write_read(SLAVE_ADDR, &[reg], &mut byte);
        Ok(byte[0])
    }

    /// Reads series of bytes into buf from specified reg
    fn read_bytes(&mut self, reg: u8, buf: &mut [u8]) -> Result<(), MPUError> {
        self.i2c.write_read(SLAVE_ADDR, &[reg], buf);
        Ok(())
    }

    /// Converts 2 bytes number in 2 compliment
    /// TODO i16?! whats 0x8000?!
    fn read_word_2c(&self, byte: &[u8]) -> i32 {
        let high: i32 = byte[0] as i32;
        let low: i32 = byte[1] as i32;
        let mut word: i32 = (high << 8) + low;

        if word >= 0x8000 {
            word = -((65535 - word) + 1);
        }

        word
    }
}
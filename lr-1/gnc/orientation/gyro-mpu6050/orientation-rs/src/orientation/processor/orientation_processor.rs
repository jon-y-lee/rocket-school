use linux_embedded_hal::I2cdev;
use embedded_hal::{
    blocking::delay::DelayMs,
    blocking::i2c::{Write, WriteRead},
};
use std::*;
use std::io;
use nalgebra::{Vector3, Vector2};

// use linux_embedded_hal::sysfs_gpio::Result;
// pub struct Processor <I> {
//     i2c: I2cdev
// }

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

/// Handles all operations on/with Mpu6050
pub struct Processor {
    i2c: I2cdev,
    // acc_sensitivity: f32,
    // gyro_sensitivity: f32,
}

impl Processor {
    /// Side effect free constructor with default sensitivies, no calibration
    pub fn new(i2c: I2cdev) -> Self {
        Processor {
            i2c,
            // acc_sensitivity: ACCEL_SENS.0,
            // gyro_sensitivity: GYRO_SENS.0,
        }
    }

    pub fn wake(&mut self) {
        // MPU6050 has sleep enabled by default -> set bit 0 to wake
        // Set clock source to be PLL with x-axis gyroscope reference, bits 2:0 = 001 (See Register Map )
        self.write_byte(PWR_MGMT_1::ADDR, 0x01);
        // delay.delay_ms(100u8);
        println!("test test");
    }

    fn verify(&mut self) {
        let address = self.read_byte(WHOAMI);
        if address.unwrap() != SLAVE_ADDR {
            println!("Not verified");
        }
    }


    pub fn write_byte(&mut self, reg: u8, byte: u8) {
        self.i2c.write(SLAVE_ADDR, &[reg, byte]);
            // .map_err(Mpu6050Error::I2c)?;
        // delay disabled for dev build
        // TODO: check effects with physical unit
        // self.delay.delay_ms(10u8);
        // Ok(())
    }

    pub fn read_byte(&mut self, reg: u8) -> Result<u8, MPUError> {
        let mut byte: [u8; 1] = [0; 1];
        self.i2c.write_read(SLAVE_ADDR, &[reg], &mut byte);
        Ok(byte[0])
    }

    /// Reads series of bytes into buf from specified reg
    pub fn read_bytes(&mut self, reg: u8, buf: &mut [u8]) -> Result<(), MPUError> {
        self.i2c.write_read(SLAVE_ADDR, &[reg], buf);
        Ok(())
    }

    // pub fn read_bits(&mut self, reg: u8, start_bit: u8, length: u8) -> Result<u8, MPUError> {
    //     let mut byte: [u8; 1] = [0; 1];
    //     self.read_bytes(reg, &mut byte);
    //     Ok(self.get_bits(byte[0], start_bit, length))
    // }
    //
    // /// get bits start - start+length from byte
    // pub fn get_bits(mut byte: u8, bit_start: u8, length: u8) -> u8 {
    //     // 01101001 read byte
    //     // 76543210 bit numbers
    //     //    xxx   args: bit_start=4, length=3
    //     //    010   masked
    //     //   -> 010 shifted
    //
    //     // without mask_shift, strange behavior occurs, wenn bit_start < length.
    //     // e.g. bit_start=2, length = 2
    //     // in SOME cases, you get an 'attempt to subtract with overflow' exception, when
    //     // bitstart - length + 1 = 0
    //     // therefore just "cut off" at 0 shift
    //     let mask_shift: u8 = if bit_start < length { 0 } else { bit_start - length + 1 };
    //     let mask: u8 = ((1 << length) - 1) << mask_shift;
    //     byte &= mask as u8;
    //     byte >>= mask_shift;
    //     byte
    // }

    /// Reads rotation (gyro/acc) from specified register
    fn read_rot(&mut self, reg: u8) -> Result<Vector3<f32>, MPUError> {
        let mut buf: [u8; 6] = [0; 6];
        self.read_bytes(reg, &mut buf);

        Ok(Vector3::<f32>::new(
            self.read_word_2c(&buf[0..2]) as f32,
            self.read_word_2c(&buf[2..4]) as f32,
            self.read_word_2c(&buf[4..6]) as f32
        ))
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


    pub fn read(&mut self) {

        // let byte = self.read_bits(GYRO_CONFIG::ADDR,
        //                           GYRO_CONFIG::FS_SEL.bit,
        //                           GYRO_CONFIG::FS_SEL.length)?;

        let mut gyro = self.read_rot(GYRO_REGX_H).unwrap();

        gyro *= PI_180 * GYRO_SENS.0;
        // println!("Reading from registry");
        println!("gyro: {:?}", gyro);

        // Ok(gyro);
    }
}
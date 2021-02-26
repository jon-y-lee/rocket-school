use linux_embedded_hal::I2cdev;
use crate::orientation::processor::orientation_processor::{MPUError, PWR_MGMT_1, WHOAMI, SLAVE_ADDR};

pub trait SignalProcessor {

    // Static method signature; `Self` refers to the implementor type.
    fn new(i2c: I2cdev) -> Self;

    fn init(&mut self);

    fn verify(&mut self);

    fn write_byte(&mut self, reg: u8, byte: u8);

    fn read_byte(&mut self, reg: u8) -> Result<u8, MPUError>;

    fn read_bytes(&mut self, reg: u8, buf: &mut [u8]) -> Result<(), MPUError>;

    fn read_word_2c(&self, byte: &[u8]) -> i32;
}
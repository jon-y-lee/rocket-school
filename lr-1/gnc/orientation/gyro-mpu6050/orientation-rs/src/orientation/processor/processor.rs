use linux_embedded_hal::I2cdev;
use crate::orientation::processor::orientation_processor::{MPUError, Processor, PWR_MGMT_1, WHOAMI, SLAVE_ADDR};

pub trait SignalProcessor {

    // Static method signature; `Self` refers to the implementor type.
    fn new(i2c: I2cdev) -> Self;

    fn init(&mut self);

    fn verify(&mut self);

    fn write_byte(&mut self, reg: u8, byte: u8);

    fn read_byte(&mut self, reg: u8) -> Result<u8, MPUError>;

    fn read_bytes(&mut self, reg: u8, buf: &mut [u8]) -> Result<(), MPUError>;

    fn read_word_2c(&self, byte: &[u8]) -> i32;



    // fn new(i2c: I2cdev) -> Processor {
    //     Processor {
    //         i2c,
    //     }
    // }
    //
    // fn init(&mut self) {
    //     // MPU6050 has sleep enabled by default -> set bit 0 to wake
    //     // Set clock source to be PLL with x-axis gyroscope reference, bits 2:0 = 001 (See Register Map )
    //     self.write_byte(PWR_MGMT_1::ADDR, 0x01);
    //     println!("test test");
    // }
    //
    // fn verify(&mut self) {
    //     let address = self.read_byte(WHOAMI);
    //     if address.unwrap() != SLAVE_ADDR {
    //         println!("Not verified");
    //     }
    // }
    //
    // fn write_byte(&mut self, reg: u8, byte: u8) {
    //     self.i2c.write(SLAVE_ADDR, &[reg, byte]);
    // }
    //
    // fn read_byte(&mut self, reg: u8) -> Result<u8, MPUError> {
    //     let mut byte: [u8; 1] = [0; 1];
    //     self.i2c.write_read(SLAVE_ADDR, &[reg], &mut byte);
    //
    //     Ok(byte[0])
    // }
    //
    // /// Reads series of bytes into buf from specified reg
    // fn read_bytes(&mut self, reg: u8, buf: &mut [u8]) -> Result<(), MPUError> {
    //     self.i2c.write_read(SLAVE_ADDR, &[reg], buf);
    //     Ok(())
    // }
    //
    // /// Converts 2 bytes number in 2 compliment
    // /// TODO i16?! whats 0x8000?!
    // fn read_word_2c(&self, byte: &[u8]) -> i32 {
    //     let high: i32 = byte[0] as i32;
    //     let low: i32 = byte[1] as i32;
    //     let mut word: i32 = (high << 8) + low;
    //
    //     if word >= 0x8000 {
    //         word = -((65535 - word) + 1);
    //     }
    //
    //     word
    // }
}
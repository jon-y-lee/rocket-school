use crate::orientation::processor::processor::SignalProcessor;
use nalgebra::{Vector3, Vector2};
use crate::orientation::processor::orientation_processor::{MPUError, GYRO_REGX_H, PI_180, ACCEL_SENS, ACC_REGX_H, GYRO_SENS, I2CProcessor};
use crate::util::RAD_TO_DEG;
use libm::{powf, atan2f, sqrtf};
use logs::{info, warn};

pub struct AccelerometerProcessor {
    signal_processor: I2CProcessor,
}

impl AccelerometerProcessor {

    pub fn new(signal_processor: I2CProcessor) -> Self {
        AccelerometerProcessor {
            signal_processor
        }
    }

    /// Reads rotation (gyro/acc) from specified register
    pub fn read_rotation(&mut self, reg: u8) -> std::result::Result<Vector3<f32>, MPUError> {
        let mut buf: [u8; 6] = [0; 6];
        self.signal_processor.read_bytes(reg, &mut buf);

        Ok(Vector3::<f32>::new(
            self.signal_processor.read_word_2c(&buf[0..2]) as f32,
            self.signal_processor.read_word_2c(&buf[2..4]) as f32,
            self.signal_processor.read_word_2c(&buf[4..6]) as f32
        ))
    }

    pub fn read_gyro(&mut self) -> std::result::Result<Vector3<f32>, MPUError> {
        let mut gyro = self.read_rotation(GYRO_REGX_H).unwrap();

        gyro *= PI_180 * GYRO_SENS.0;
        info!("gyro: {:?}", gyro);

        Ok(gyro)
    }


    /// Accelerometer readings in g
    pub fn read_accelerometer_in_g(&mut self) -> std::result::Result<Vector3<f32>, MPUError> {
        let mut acc = self.read_rotation(ACC_REGX_H)?;
        acc /= ACCEL_SENS.0;

        Ok(acc)
    }

    /// Roll and pitch estimation from raw accelerometer readings
    /// NOTE: no yaw! no magnetometer present on MPU6050
    /// https://www.nxp.com/docs/en/application-note/AN3461.pdf equation 28, 29
    pub fn get_roll_and_pitch(&mut self) -> std::result::Result<Vector2<f32>, MPUError> {
        let acc = self.read_accelerometer_in_g()?;

        Ok(Vector2::<f32>::new(
            atan2f(acc.y, sqrtf(powf(acc.x, 2.) + powf(acc.z, 2.))),
            atan2f(-acc.x, sqrtf(powf(acc.y, 2.) + powf(acc.z, 2.)))
        ))
    }

    pub(crate) fn read(&mut self) {

        let pitchRoll = self.get_roll_and_pitch().unwrap();
        info!("acc: {:?} {:?}", pitchRoll[0] * RAD_TO_DEG, pitchRoll[1] * RAD_TO_DEG);

    }
}
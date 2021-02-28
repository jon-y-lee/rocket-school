use crate::orientation::processor::processor::SignalProcessor;
use nalgebra::{Vector3, Vector2};
use crate::orientation::processor::orientation_processor::{MPUError, I2CProcessor};
use crate::util::RAD_TO_DEG;
use libm::{powf, atan2f, sqrtf};
use logs::{info, warn};
use crate::producer::multicast_client::MulticastProducer;
use crate::producer::temperature_event::Event;
use crate::producer::event_type::EventType::TEMPERATURE;

pub struct TemperatureProcessor {
    signal_processor: I2CProcessor,
    publisher: MulticastProducer
}

pub const TEMP_OUT_H : u8= 0x41;

/// Temperature Sensitivity
pub const TEMP_SENSITIVITY: f32 = 340.;
/// Temperature Offset
pub const TEMP_OFFSET: f32 = 36.53;

impl TemperatureProcessor {

    pub fn new(signal_processor: I2CProcessor) -> Self {
        TemperatureProcessor {
            signal_processor,
            publisher: crate::producer::multicast_client::MulticastProducer::new()
        }
    }

    pub fn init(&self) {
        self.publisher.init();
    }

    /// Sensor Temp in degrees celcius
    pub fn get_temp(&mut self) -> std::result::Result<f32, MPUError> {
        let mut buf: [u8; 2] = [0; 2];
        self.signal_processor.read_bytes(TEMP_OUT_H, &mut buf)?;
        let raw_temp = self.signal_processor.read_word_2c(&buf[0..2]) as f32;

        // According to revision 4.2
        Ok((raw_temp / TEMP_SENSITIVITY) + TEMP_OFFSET)
    }

    pub(crate) fn read(&mut self) {
        let temp = self.get_temp().unwrap();
        info!("temp: {:?}", (temp * (9.0/5.0)) + 32.0);
        let fTemp: f32 = (temp * (9.0/5.0)) + 32.0;

        let event = Event {
            eventType: TEMPERATURE,
            value: fTemp
        };

        self.publisher.send(event);
    }
}
pub mod reading;
pub mod record;

use crate::bme280::reading::Reading;
use bme280::i2c::BME280;
use linux_embedded_hal::{Delay, I2cdev};

pub struct Device {
    bme280: BME280<I2cdev>,
}

impl Device {
    pub fn new() -> Self {
        let i2c_bus = I2cdev::new("/dev/i2c-1").unwrap();
        let mut bme280 = BME280::new_secondary(i2c_bus);

        bme280.init(&mut Delay).unwrap();

        Self { bme280 }
    }

    pub fn read(&mut self) -> Reading {
        let measurements = self.bme280.measure(&mut Delay).unwrap();

        let reading = Reading::new(measurements.temperature, measurements.pressure, measurements.temperature);

        reading
    }

    pub fn record(&mut self) -> Result<record::Record, Box<dyn std::error::Error>> {
        let reading = self.read();
        let record = record::Record::new(reading, chrono::Utc::now());
        Ok(record)
    }
}

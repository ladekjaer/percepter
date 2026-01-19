use bme280::i2c::BME280;
use linux_embedded_hal::{Delay, I2cdev};
use uuid::Uuid;
use rerec;
use rerec::Reading;
use rerec::record::Record;

pub struct BME280Driver {
    bme280: BME280<I2cdev>,
}

impl BME280Driver {
    pub fn new() -> Self {
        let i2c_bus = I2cdev::new("/dev/i2c-1").unwrap();
        let mut bme280 = BME280::new_secondary(i2c_bus);

        bme280.init(&mut Delay).unwrap();

        Self { bme280 }
    }

    pub fn read(&mut self) -> Reading {
        let reading = self.bme280.measure(&mut Delay).unwrap();
        let reading =
            rerec::bme280::BME280::new(reading.temperature, reading.pressure, reading.humidity);
        Reading::BME280(reading)
    }

    pub fn record(&mut self) -> Result<Record, Box<dyn std::error::Error>> {
        let id = Uuid::new_v4();
        let reading = self.read();
        let timestamp = chrono::Utc::now();

        let record = Record::new(id, timestamp, reading);

        Ok(record)
    }
}

use std::fmt::Display;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::bme280::reading::Reading;

pub struct Record {
    id: Uuid,
    reading: Reading,
    timestamp: DateTime<Utc>
}

impl Record {
    pub fn new(reading: Reading, timestamp: DateTime<Utc>) -> Self {
        Self {
            id: Uuid::new_v4(),
            reading,
            timestamp
        }
    }

    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub fn get_temperature(&self) -> f32 {
        self.reading.get_temperature()
    }

    pub fn get_pressure(&self) -> f32 {
        self.reading.get_pressure()
    }

    pub fn get_humidity(&self) -> f32 {
        self.reading.get_humidity()
    }

    pub fn get_timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }
}

impl Display for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] BME280: {:.2} °C, {:.2} Pa, {:.2}% ({})", self.get_timestamp(), self.get_temperature(), self.get_pressure(), self.get_humidity(), self.get_id())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_record() {
        let reading = Reading::new(22.625, 101325.0, 35.0);
        let timestamp = Utc::now();
        let _record = Record::new(reading, timestamp);
    }

    #[test]
    fn test_get_id() {
        let reading = Reading::new(22.625, 101325.0, 35.0);
        let timestamp = Utc::now();
        let record = Record::new(reading, timestamp);
        assert_ne!(record.get_id(), Uuid::new_v4());
    }

    #[test]
    fn test_get_temperature() {
        let reading = Reading::new(22.625, 101325.0, 35.0);
        let timestamp = Utc::now();
        let record = Record::new(reading, timestamp);
        assert_eq!(record.get_temperature(), 22.625);
    }

    #[test]
    fn test_get_pressure() {
        let reading = Reading::new(22.625, 101325.0, 35.0);
        let timestamp = Utc::now();
        let record = Record::new(reading, timestamp);
        assert_eq!(record.get_pressure(), 101325.0);
    }

    #[test]
    fn test_get_humidity() {
        let reading = Reading::new(22.625, 101325.0, 35.0);
        let timestamp = Utc::now();
        let record = Record::new(reading, timestamp);
        assert_eq!(record.get_humidity(), 35.0);
    }

    #[test]
    fn test_display() {
        let reading = Reading::new(22.625, 101325.0, 35.0);
        let timestamp = Utc::now();
        let record = Record::new(reading, timestamp);
        let record_id = record.get_id();
        let expected = format!("[{}] BME280: 22.62 °C, 101325.00 Pa, 35.00% ({})", timestamp, record_id);
        assert_eq!(format!("{}", record), expected);
    }
}
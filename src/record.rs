use crate::reading::Reading;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Record {
    id: Uuid,
    timestamp: DateTime<Utc>,
    reading: Reading,
}

impl Record {
    pub fn new(id: Uuid, timestamp: DateTime<Utc>, reading: Reading) -> Self {
        Self {
            id,
            timestamp,
            reading,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }

    pub fn reading(&self) -> &Reading {
        &self.reading
    }
}

impl Display for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let id = self.id();
        let timestamp = self.timestamp();
        let reading_display = format!("{}", self.reading);
        write!(f, "[{}] {} ({})", timestamp, reading_display, id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reading::bme280::BME280;

    #[test]
    fn test_record() {
        let _record = Record {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            reading: Reading::BME280(BME280::new(22.625, 101325.0, 35.0)),
        };
    }

    #[test]
    fn test_new() {
        let id = Uuid::new_v4();
        let timestamp = Utc::now();
        let reading = Reading::BME280(BME280::new(22.625, 101325.0, 35.0));
        let reading_copy = reading.clone();
        let record = Record::new(id, timestamp, reading);
        assert_eq!(record.id(), id);
        assert_eq!(record.timestamp(), timestamp);
        assert_eq!(record.reading(), &reading_copy);
    }

    #[test]
    fn test_id() {
        let id = Uuid::new_v4();
        let timestamp = Utc::now();
        let reading = Reading::BME280(BME280::new(22.625, 101325.0, 35.0));
        let record = Record::new(id, timestamp, reading);
        assert_eq!(record.id(), id);
    }

    #[test]
    fn test_timestamp() {
        let id = Uuid::new_v4();
        let timestamp = Utc::now();
        let reading = Reading::BME280(BME280::new(22.625, 101325.0, 35.0));
        let record = Record::new(id, timestamp, reading);
        assert_eq!(record.timestamp(), timestamp);
    }

    #[test]
    fn test_reading() {
        let id = Uuid::new_v4();
        let timestamp = Utc::now();
        let reading = Reading::BME280(BME280::new(22.625, 101325.0, 35.0));
        let reading_copy = reading.clone();
        let record = Record::new(id, timestamp, reading);
        assert_eq!(record.reading(), &reading_copy);
    }

    #[test]
    fn test_display() {
        let id = Uuid::new_v4();
        let timestamp = Utc::now();
        let reading = Reading::BME280(BME280::new(22.625, 101325.0, 35.0));
        let record = Record::new(id, timestamp, reading);

        let expected = format!(
            "[{}] BME280: 22.62 °C, 101325.00 Pa, 35.00% ({})",
            timestamp, id
        );
        let actual = format!("{}", record);
        assert_eq!(actual, expected);
    }
}

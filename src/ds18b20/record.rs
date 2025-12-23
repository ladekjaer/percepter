use std::fmt::{Display, Formatter};
use chrono::{DateTime, Utc};

pub struct Record {
    device_name: String,
    raw_reading: i32,
    timestamp: DateTime<Utc>,
}

impl Record {
    pub(crate) fn new(device_name: &str, raw_reading: i32, timestamp: DateTime<Utc>) -> Self {
        let device_name = device_name.to_string();
        Self { device_name, raw_reading, timestamp }
    }
    pub fn get_device_name(&self) -> String {
        self.device_name.clone()
    }

    pub fn get_temperature(&self) -> f32 {
        self.raw_reading as f32 / 1000.0
    }

    pub fn get_timestamp(&self) -> DateTime<Utc> {
        self.timestamp.clone()
    }
}

impl Display for Record {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = self.get_device_name();
        let temperature = self.get_temperature();
        let timestamp = self.get_timestamp();
        write!(f, "[{}] {}: {:.3} °C", timestamp, name, temperature)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_record() {
        let _record = Record {
            device_name: "28-000000000000".to_string(),
            raw_reading: 22625,
            timestamp: Utc::now()
        };
    }

    #[test]
    fn test_get_device_name() {
        let now = Utc::now();
        let record = Record::new("28-000000000000", 22625, now);
        let expected = "28-000000000000".to_string();
        assert_eq!(record.get_device_name(), expected);
    }

    #[test]
    fn test_get_temperature() {
        let now = Utc::now();
        let record = Record::new("28-000000000000", 22625, now);
        let expected = 22.625;
        assert_eq!(record.get_temperature(), expected);
    }

    #[test]
    fn test_get_timestamp() {
        let now = Utc::now();
        let record = Record::new("28-000000000000", 22625, now);
        assert_eq!(record.get_timestamp(), now);
    }

    #[test]
    fn test_new() {
        let _record = Record::new("28-000000000000", 22625, Utc::now());
    }

    #[test]
    fn test_display() {
        let timestamp = DateTime::parse_from_rfc3339("2025-12-23T04:03:47.117838086Z")
            .unwrap()
            .to_utc();
        let record = Record::new("28-000000000000", 22625, timestamp);
        let expected = "[2025-12-23 04:03:47.117838086 UTC] 28-000000000000: 22.625 °C";
        assert_eq!(&record.to_string(), expected);
    }
}

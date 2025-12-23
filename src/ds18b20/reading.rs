use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, PartialEq)]
pub struct Reading {
    device_name: String,
    // The raw reading from the device is an i32 indicating the temperature in millidegrees Celsius (m°C).
    raw_reading: i32
}

impl Reading {
    pub(crate) fn new(device_name: &str, raw_reading: i32) -> Self {
        let device_name = device_name.to_string();
        Self { device_name, raw_reading }
    }

    pub(crate) fn get_device_name(&self) -> String {
        self.device_name.clone()
    }

    /// Returns the temperature in degrees Celsius.
    pub fn get_temperature(&self) -> f32 {
        self.raw_reading as f32 / 1000.0
    }
}

impl fmt::Display for Reading {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let name = self.get_device_name();
        let temperature = self.get_temperature();
        write!(f, "{}: {} °C", name, temperature)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reading() {
        let _reading = Reading { device_name: "28-000000000000".to_string(), raw_reading: 22625 };
    }

    #[test]
    fn test_new() {
        let _reading = Reading::new("28-000000000000", 22625);
    }

    #[test]
    fn test_debug() {
        let reading = Reading::new("28-000000000000", 22625);
        let debug_output = format!("{:?}", reading);
        assert!(debug_output.contains("Reading"));
        assert!(debug_output.contains("28-000000000000"));
        assert!(debug_output.contains("22625"));
    }

    #[test]
    fn test_partial_eq() {
        let reading1 = Reading::new("28-000000000000", 22625);
        let reading1_again = Reading::new("28-000000000000", 22625);
        let reading2 = Reading::new("28-000000000000", 23000);

        assert_eq!(reading1, reading1_again);
        assert_ne!(reading1, reading2);
    }

    #[test]
    fn test_get_temperature() {
        let reading = Reading::new("28-000000000000", 22625);
        assert_eq!(reading.get_temperature(), 22.625);

        let reading = Reading::new("28-000000000000", -22625);
        assert_eq!(reading.get_temperature(), -22.625);
    }

    #[test]
    fn test_get_device_name() {
        let reading = Reading::new("28-000000000000", 22625);
        let actual = reading.get_device_name();
        assert_eq!(actual, "28-000000000000");
    }

    #[test]
    fn test_display() {
        let reading = Reading::new("28-000000000000", 22625);
        assert_eq!(format!("{}", reading), "28-000000000000: 22.625 °C");

        let reading = Reading::new("28-000000000000", -22625);
        assert_eq!(format!("{}", reading), "28-000000000000: -22.625 °C");
    }
}

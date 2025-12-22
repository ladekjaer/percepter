use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, PartialEq)]
pub struct Reading {
    // The raw reading from the device is an i32 indicating the temperature in millidegrees Celsius (m°C).
    raw_reading: i32
}

impl Reading {
    pub(crate) fn new(raw_reading: i32) -> Self {
        Self { raw_reading }
    }

    /// Returns the temperature in degrees Celsius.
    pub fn get_temperature(&self) -> f32 {
        self.raw_reading as f32 / 1000.0
    }
}

impl fmt::Display for Reading {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let temperature = self.get_temperature();
        write!(f, "{} °C", temperature)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reading() {
        let _reading = Reading { raw_reading: 22625 };
    }

    #[test]
    fn test_new() {
        let _reading = Reading::new(22625);
    }

    #[test]
    fn test_get_temperature() {
        let reading = Reading::new(22625);
        assert_eq!(reading.get_temperature(), 22.625);

        let reading = Reading::new(-22625);
        assert_eq!(reading.get_temperature(), -22.625);
    }

    #[test]
    fn test_display() {
        let reading = Reading::new(22625);
        assert_eq!(format!("{}", reading), "22.625 °C");

        let reading = Reading::new(-22625);
        assert_eq!(format!("{}", reading), "-22.625 °C");
    }
}

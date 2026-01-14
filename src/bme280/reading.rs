use std::fmt;

pub struct Reading {
    // temperature in degrees Celsius
    temperature: f32,

    // Pressure in Pascals
    pressure: f32,

    // Relative humidity in percentage
    humidity: f32,
}

impl Reading {
    pub fn new(temperature: f32, pressure: f32, humidity: f32) -> Self {
        Self {
            temperature,
            pressure,
            humidity,
        }
    }

    pub fn get_temperature(&self) -> f32 {
        self.temperature
    }

    pub fn get_pressure(&self) -> f32 {
        self.pressure
    }

    pub fn get_humidity(&self) -> f32 {
        self.humidity
    }
}

impl fmt::Display for Reading {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Temperature: {:.2} °C, Pressure: {:.2} Pa, Humidity: {:.2}%", self.get_temperature(), self.get_pressure(), self.get_humidity())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reading() {
        let _reading = Reading { temperature: 22.625, pressure: 101325.0, humidity: 35.0 };
    }

    #[test]
    fn test_new() {
        let _reading = Reading::new( 22.625, 101325.0, 35.0);
    }

    #[test]
    fn test_get_temperature() {
        let reading = Reading::new(22.625, 101325.0, 35.0);
        assert_eq!(reading.get_temperature(), 22.625);
    }

    #[test]
    fn test_get_pressure() {
        let reading = Reading::new(22.625, 101325.0, 35.0);
        assert_eq!(reading.get_pressure(), 101325.0);
    }

    #[test]
    fn test_get_humidity() {
        let reading = Reading::new(22.625, 101325.0, 35.0);
        assert_eq!(reading.get_humidity(), 35.0);
    }

    #[test]
    fn test_display() {
        let reading = Reading::new(22.625, 101325.0, 35.0);
        assert_eq!(format!("{}", reading), "Temperature: 22.62 °C, Pressure: 101325.00 Pa, Humidity: 35.00%");
    }
}

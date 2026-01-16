use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};
use crate::reading::bme280::BME280;
use crate::reading::ds18b20::DS18B20;

pub mod bme280;
pub mod ds18b20;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Reading {
    BME280(BME280),
    DS18B20(DS18B20),
}

impl Display for Reading {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Reading::BME280(bme280) => { bme280.fmt(f) }
            Reading::DS18B20(ds18b20) => { ds18b20.fmt(f) }
        }
    }
}

use crate::ds18b20::reading::Reading;

mod ds18b20;

fn main() {
    let readings = read_all().unwrap();

    for reading in readings {
        println!("{}: {}", reading.0, reading.1.to_string());
    }
}

fn read_all() -> Result<Vec<(String, Reading)>, Box<dyn std::error::Error>> {
    let devices = ds18b20::DS18B20::get_all()?;
    let mut readings: Vec<(String, Reading)> = vec!();
    for device in devices {
        let device_name = device.get_name();
        let reading = device.read()?;
        readings.push((device_name, reading));
    }

    Ok(readings)
}

mod ds18b20;

fn main() {
    let readings = read_all().unwrap();

    for reading in readings {
        println!("{}: {} m°C", reading.0, reading.1);
    }
}

fn read_all() -> Result<Vec<(String, i32)>, Box<dyn std::error::Error>> {
    let devices = ds18b20::DS18B20::get_all()?;
    let mut readings: Vec<(String, i32)> = vec!();
    for device in devices {
        readings.push((device.get_name(), device.read()?));
    }

    Ok(readings)
}

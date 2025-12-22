use crate::ds18b20::reading::Reading;

mod ds18b20;

fn main() {
    let readings = read_all().unwrap();

    for reading in readings {
        println!("{}", reading);
    }
}

fn read_all() -> Result<Vec<Reading>, Box<dyn std::error::Error>> {
    let devices = ds18b20::DS18B20::get_all()?;
    let mut readings: Vec<Reading> = vec!();
    for device in devices {
        let reading = device.read()?;
        readings.push(reading);
    }

    Ok(readings)
}

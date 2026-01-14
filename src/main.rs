use std::thread;
use std::time::Duration;
use clap::Parser;
use crate::ds18b20::reading::Reading;
use crate::ds18b20::record::Record;

mod bme280;
mod ds18b20;

fn main() {
    let args = Args::parse();

    if let Some(interval) = args.interval {
        loop {
            output_all(args.timestamps);
            output_bme280_record();
            thread::sleep(Duration::from_secs(interval));
        }
    } else {
        output_all(args.timestamps);
        output_bme280_record();
    }
}

fn output_bme280_record() {
    let mut bme280 = bme280::Device::new();
    let bme280_record = bme280.record().unwrap();
    println!("{}", bme280_record);
}

fn output_all(timestamps: bool) {
    match timestamps {
        true => record_all_to_std_out(),
        false => read_all_to_std_out()
    }
}

fn record_all_to_std_out() {
    let records = record_all().unwrap();

    for record in records {
        println!("{}", record);
    }
}

fn read_all_to_std_out() {
    let readings = read_all().unwrap();

    for reading in readings {
        println!("{}", reading);
    }
}

fn record_all() -> Result<Vec<Record>, Box<dyn std::error::Error>> {
    let devices = ds18b20::DS18B20::get_all()?;
    let mut records: Vec<Record> = vec!();
    for device in devices {
        let record = device.record()?;
        records.push(record);
    }

    Ok(records)
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

#[derive(Debug, Parser)]
#[command(version, about)]
struct Args {
    #[arg(short, long, help = "Interval in seconds for recapture of sensor readings")]
    interval: Option<u64>,

    #[arg(short, long, help = "Include timestamps in output")]
    timestamps: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_all() {
        let _readings = read_all().unwrap();
    }

    #[test]
    fn test_read_all_to_std_out() {
        read_all_to_std_out();
    }

    #[test]
    fn test_record_all() {
        let _records = record_all().unwrap();
    }

    #[test]
    fn test_record_all_to_std_out() {
        record_all_to_std_out();
    }

    #[test]
    fn test_output_all() {
        output_all(false);
        output_all(true);
    }
}

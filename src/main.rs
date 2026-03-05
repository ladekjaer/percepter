use clap::Parser;
use rerec::record::Record;
use rerec::Reading;
use std::thread;
use std::time::Duration;

mod drivers;
mod herodot;

fn main() {
    let args = Args::parse();

    let host: Option<&str> = args.host.as_ref().map(|host| host.as_str());
    let auth_token: Option<&str> = args.auth_token.as_ref().map(|auth_token| auth_token.as_str());

    loop {
        output_all(args.timestamps, host, auth_token);
        output_bme280_record(host, auth_token);
        if let Some(interval) = args.interval {
            thread::sleep(Duration::from_secs(interval));
        } else {
            break;
        }
    }
}

fn output_bme280_record(host: Option<&str>, auth_token: Option<&str>) {
    let mut bme280 = drivers::bme280::BME280Driver::new();
    let record = bme280.record().unwrap();
    match host {
        Some(host) => {
            let herodot = herodot::Herodot::new(host.into(), auth_token.unwrap().into());
            match herodot.commit_record(&record) {
                Ok(uuid) => println!("Committed record with UUID: {}", uuid),
                Err(e) => println!("Failed to commit record: {}", e),
            }
        }
        None => println!("No host specified, skipping commit."),
    }
    println!("{}", record);
}

fn output_all(timestamps: bool, host: Option<&str>, auth_token: Option<&str>) {
    match timestamps {
        true => record_all_to_std_out(host, auth_token),
        false => read_all_to_std_out(),
    }
}

fn record_all_to_std_out(host: Option<&str>, auth_token: Option<&str>) {
    let records = record_all_ds18b20().unwrap();

    for record in records {
        println!("{}", record);
        match host {
            Some(host) => {
                let herodot = herodot::Herodot::new(host.into(), auth_token.unwrap().into());
                match herodot.commit_record(&record) {
                    Ok(uuid) => println!("Committed record with UUID: {}", uuid),
                    Err(e) => println!("Failed to commit record: {}", e),
                }
            }
            None => println!("No host specified, skipping commit."),
        }
    }
}

fn read_all_to_std_out() {
    let readings = read_all_ds18b20().unwrap();

    for reading in readings {
        println!("{}", reading);
    }
}

fn record_all_ds18b20() -> Result<Vec<Record>, Box<dyn std::error::Error>> {
    let devices = drivers::ds18b20::DS18B20::get_all()?;
    let mut records: Vec<Record> = vec![];
    for device in devices {
        let record = device.record()?;
        records.push(record);
    }

    Ok(records)
}

fn read_all_ds18b20() -> Result<Vec<Reading>, Box<dyn std::error::Error>> {
    let devices = drivers::ds18b20::DS18B20::get_all()?;
    let mut readings: Vec<Reading> = vec![];
    for device in devices {
        let reading = device.read()?;
        readings.push(reading);
    }

    Ok(readings)
}

#[derive(Debug, Parser)]
#[command(version, about)]
struct Args {
    #[arg(
        short,
        long,
        help = "Interval in seconds for recapture of sensor readings"
    )]
    interval: Option<u64>,

    #[arg(short, long, help = "Include timestamps in output")]
    timestamps: bool,

    #[arg(long, help = "Herodot server host")]
    host: Option<String>,
    
    #[arg(long, help = "Herodot server auth token")]
    auth_token: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_all() {
        let _readings = read_all_ds18b20().unwrap();
    }

    #[test]
    fn test_read_all_to_std_out() {
        read_all_to_std_out();
    }

    #[test]
    fn test_record_all() {
        let _records = record_all_ds18b20().unwrap();
    }

    #[test]
    fn test_record_all_to_std_out() {
        record_all_to_std_out(None, None);
    }

    #[test]
    fn test_output_all() {
        output_all(false, None, None);
        output_all(true, None, None);
    }
}

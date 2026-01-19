use crate::reading;
use crate::reading::Reading;
use crate::record::Record;
use chrono::Utc;
use std::error::Error;
use uuid::Uuid;

#[derive(Debug, PartialEq)]
pub struct DS18B20 {
    sysfs_path: std::path::PathBuf,
}

impl DS18B20 {
    pub fn device_name(&self) -> String {
        self.sysfs_path
            .file_name()
            .expect("The file name must be the device name.")
            .to_str()
            .expect("The file name must be valid UTF-8")
            .to_string()
    }

    pub fn read(&self) -> Result<Reading, Box<dyn Error>> {
        let slave_path = self.sysfs_path.join("w1_slave");
        let content = std::fs::read_to_string(slave_path)?;

        if !content.contains("YES") {
            return Err("CRC check failed".into());
        }

        let temp_str = content
            .split("t=")
            .nth(1)
            .ok_or("Failed to find temperature")?
            .trim();

        let temp = temp_str.parse::<i32>()?;
        let reading = reading::ds18b20::DS18B20::new(self.device_name(), temp);
        let reading = Reading::DS18B20(reading);
        Ok(reading)
    }

    pub fn record(&self) -> Result<Record, Box<dyn Error>> {
        let timestamp = Utc::now();
        let reading = self.read()?;
        let record = Record::new(Uuid::new_v4(), timestamp, reading);
        Ok(record)
    }

    pub fn get_all() -> Result<Vec<Self>, Box<dyn Error>> {
        Self::get_all_from_path("/sys/bus/w1/devices/")
    }

    // easily be written by creating a temporary mock directory with mock DS18B20 devices.

    fn get_all_from_path<P: AsRef<std::path::Path>>(path: P) -> Result<Vec<Self>, Box<dyn Error>> {
        let mut devices = Vec::new();

        let devices_path = path.as_ref();
        if !devices_path.exists() {
            return Ok(devices);
        }

        for device in std::fs::read_dir(devices_path)? {
            let device = device?;
            let path = device.path();
            let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

            if name.starts_with("28-") {
                devices.push(DS18B20 { sysfs_path: path });
            }
        }

        Ok(devices)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_ds18b20() {
        let sysfs_path = PathBuf::from("/sys/bus/w1/devices/28-000000000000");
        let _device = DS18B20 { sysfs_path };
    }

    #[test]
    fn test_debug() {
        let sysfs_path = PathBuf::from("/sys/bus/w1/devices/28-000000000000");
        let device = DS18B20 {
            sysfs_path: sysfs_path.clone(),
        };
        let debug_output = format!("{:?}", device);
        assert!(debug_output.contains("DS18B20"));
        assert!(debug_output.contains(&format!("{:?}", sysfs_path)));
    }

    #[test]
    fn test_partial_eq() {
        let path1 = PathBuf::from("/sys/bus/w1/devices/28-000000000001");
        let path2 = PathBuf::from("/sys/bus/w1/devices/28-000000000002");

        let device1 = DS18B20 {
            sysfs_path: path1.clone(),
        };
        let device1_again = DS18B20 { sysfs_path: path1 };
        let device2 = DS18B20 { sysfs_path: path2 };

        assert_eq!(device1, device1_again);
        assert_ne!(device1, device2);
    }

    #[test]
    fn test_get_name() {
        let sysfs_path = PathBuf::from("/sys/bus/w1/devices/28-000000000000");
        let device = DS18B20 { sysfs_path };
        let actual = device.device_name();
        let expected = "28-000000000000";
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_all() {
        let devices = DS18B20::get_all();
        assert!(devices.is_ok());
    }

    #[test]
    fn test_get_all_empty() {
        let temp_dir = std::env::temp_dir().join("ds18b20_test_empty");
        std::fs::create_dir_all(&temp_dir).unwrap();

        let devices = DS18B20::get_all_from_path(&temp_dir).unwrap();
        assert_eq!(devices.len(), 0);

        std::fs::remove_dir_all(&temp_dir).unwrap();
    }

    #[test]
    fn test_get_all_multiple() {
        let temp_dir = std::env::temp_dir().join("ds18b20_test_multiple");
        std::fs::create_dir_all(&temp_dir).unwrap();

        std::fs::create_dir_all(temp_dir.join("28-000000000001")).unwrap();
        std::fs::create_dir_all(temp_dir.join("28-000000000002")).unwrap();
        std::fs::create_dir_all(temp_dir.join("not-a-device")).unwrap();

        let devices = DS18B20::get_all_from_path(&temp_dir).unwrap();
        assert_eq!(devices.len(), 2);

        let names: Vec<String> = devices.iter().map(|d| d.device_name()).collect();
        assert!(names.contains(&"28-000000000001".to_string()));
        assert!(names.contains(&"28-000000000002".to_string()));

        std::fs::remove_dir_all(&temp_dir).unwrap();
    }

    #[test]
    fn test_get_all_not_exists() {
        let temp_dir = std::env::temp_dir().join("ds18b20_test_not_exists");
        // Ensure it doesn't exist
        if temp_dir.exists() {
            std::fs::remove_dir_all(&temp_dir).unwrap();
        }

        let devices = DS18B20::get_all_from_path(&temp_dir).unwrap();
        assert_eq!(devices.len(), 0);
    }

    #[test]
    fn test_read() {
        let temp_dir = std::env::temp_dir().join("ds18b20_test");
        std::fs::create_dir_all(&temp_dir).unwrap();
        let device_dir = temp_dir.join("28-000000000000");
        std::fs::create_dir_all(&device_dir).unwrap();
        let slave_file = device_dir.join("w1_slave");
        std::fs::write(
            &slave_file,
            "6a 01 4b 46 7f ff 0c 10 3a : crc=3a YES\n6a 01 4b 46 7f ff 0c 10 3a t=22625\n",
        )
        .unwrap();

        let device = DS18B20 {
            sysfs_path: device_dir,
        };
        let actual = device.read().unwrap();
        let expected = reading::ds18b20::DS18B20::new(device.device_name(), 22625);
        let expected = Reading::DS18B20(expected);
        assert_eq!(actual, expected);

        std::fs::remove_dir_all(&temp_dir).unwrap();
    }

    #[test]
    fn test_read_crc_error() {
        let temp_dir = std::env::temp_dir().join("ds18b20_test_crc");
        std::fs::create_dir_all(&temp_dir).unwrap();
        let device_dir = temp_dir.join("28-000000000000");
        std::fs::create_dir_all(&device_dir).unwrap();
        let slave_file = device_dir.join("w1_slave");
        std::fs::write(
            &slave_file,
            "6a 01 4b 46 7f ff 0c 10 3a : crc=3a NO\n6a 01 4b 46 7f ff 0c 10 3a t=22625\n",
        )
        .unwrap();

        let device = DS18B20 {
            sysfs_path: device_dir,
        };
        let actual = device.read();
        assert!(actual.is_err());
        assert_eq!(actual.unwrap_err().to_string(), "CRC check failed");

        std::fs::remove_dir_all(&temp_dir).unwrap();
    }
}

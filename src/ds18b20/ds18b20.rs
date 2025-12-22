pub struct DS18B20 {
    sysfs_path: std::path::PathBuf
}

impl DS18B20 {
    pub fn get_name(&self) -> String {
        let filename = self.sysfs_path
            .file_name()
            .expect("The file name must be the device name.")
            .to_str()
            .expect("The file name must be valid UTF-8.");
        filename.to_string()
    }

    pub fn read(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let slave_path = self.sysfs_path.join("w1_slave");
        let content = std::fs::read_to_string(slave_path)?;
        
        if !content.contains("YES") {
            return Err("CRC check failed".into());
        }

        let temp_str = content.split("t=")
            .nth(1)
            .ok_or("Failed to find temperature")?
            .trim();
        
        let temp = temp_str.parse::<i32>()?;
        Ok(temp)
    }

    pub fn get_all() -> Result<Vec<DS18B20>, Box<dyn std::error::Error>> {
        Ok(vec!())
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use super::*;

    #[test]
    fn test_ds18b20() {
        let sysfs_path = PathBuf::from("/sys/bus/w1/devices/28-000000000000");
        let _device = DS18B20 {
            sysfs_path
        };
    }

    #[test]
    fn test_get_name() {
        let sysfs_path = PathBuf::from("/sys/bus/w1/devices/28-000000000000");
        let device = DS18B20 {
            sysfs_path
        };
        let actual = device.get_name();
        let expected = "28-000000000000";
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_all() {
        let devices = DS18B20::get_all();
        assert!(devices.is_ok());
    }

    #[test]
    fn test_read() {
        let temp_dir = std::env::temp_dir().join("ds18b20_test");
        std::fs::create_dir_all(&temp_dir).unwrap();
        let device_dir = temp_dir.join("28-000000000000");
        std::fs::create_dir_all(&device_dir).unwrap();
        let slave_file = device_dir.join("w1_slave");
        std::fs::write(&slave_file, "6a 01 4b 46 7f ff 0c 10 3a : crc=3a YES\n6a 01 4b 46 7f ff 0c 10 3a t=22625\n").unwrap();

        let device = DS18B20 {
            sysfs_path: device_dir
        };
        let actual = device.read().unwrap();
        let expected = 22625;
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
        std::fs::write(&slave_file, "6a 01 4b 46 7f ff 0c 10 3a : crc=3a NO\n6a 01 4b 46 7f ff 0c 10 3a t=22625\n").unwrap();

        let device = DS18B20 {
            sysfs_path: device_dir
        };
        let actual = device.read();
        assert!(actual.is_err());
        assert_eq!(actual.unwrap_err().to_string(), "CRC check failed");

        std::fs::remove_dir_all(&temp_dir).unwrap();
    }
}

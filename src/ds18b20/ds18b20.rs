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
}

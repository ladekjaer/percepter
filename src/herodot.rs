use crate::record::Record;
use serde_json::Value;
use std::error::Error;
use uuid::Uuid;

pub struct Herodot {
    host: String,
}

impl Herodot {
    pub fn new(host: String) -> Self {
        Self { host }
    }

    pub fn commit_record(&self, record: &Record) -> Result<Uuid, Box<dyn Error>> {
        let client = reqwest::blocking::Client::new();
        let url = format!("{}/api/record", self.host);
        let response = client.put(url).json(record).send()?;

        let reply = response.json::<Value>();
        let binding = reply?;
        let uuid = binding.get("record_id").unwrap().as_str().unwrap();
        let uuid = Uuid::parse_str(uuid)?;
        Ok(uuid)
    }
}

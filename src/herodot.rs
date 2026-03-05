use serde_json::Value;
use std::error::Error;
use rerec::record::Record;
use uuid::Uuid;

pub struct Herodot {
    host: String,
    auth_token: String,
}

impl Herodot {
    pub fn new(host: String, auth_token: String) -> Self {
        Self { host, auth_token }
    }

    pub fn commit_record(&self, record: &Record) -> Result<Uuid, Box<dyn Error>> {
        let client = reqwest::blocking::Client::new();
        let url = format!("{}/api/records", self.host);
        let response = client
            .put(url)
            .header("Authorization", format!("Bearer {}", self.auth_token))
            .json(record)
            .send()?;

        let reply = response.json::<Value>();
        let binding = reply?;
        let uuid = binding.get("record_id").unwrap().as_str().unwrap();
        let uuid = Uuid::parse_str(uuid)?;
        Ok(uuid)
    }
}

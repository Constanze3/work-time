use std::path::PathBuf;

use crate::Config;
use reqwest::Url;
use serde_json::json;

// the data recorded about each work session
pub struct WorkData {
    pub description: String,
    pub category: String,
}

pub trait Client {
    fn work_started(&self);
    fn work_ended(&self, data: WorkData);
}

pub struct LocalClient {
    id: String,
    output_folder: PathBuf,
}

impl LocalClient {
    pub fn new(id: String, output_folder: PathBuf) -> Self {
        Self { id, output_folder }
    }

    pub fn from_config(config: Config) -> Self {
        Self {
            id: config.client_id.clone(),
            output_folder: config.output_folder.clone(),
        }
    }
}

impl Client for LocalClient {
    fn work_started(&self) {
        // TODO call common work_started here
    }
    fn work_ended(&self, data: WorkData) {
        // TODO call common work_ended here
    }
}

pub struct NetworkedClient {
    local: LocalClient,
    base_url: Url,
}

impl NetworkedClient {
    pub fn new(local: LocalClient, base_url: Url) -> Self {
        Self { local, base_url }
    }

    pub fn send(&self, action: &str, data: serde_json::Value) -> reqwest::Result<()> {
        let client = reqwest::blocking::Client::new();
        let url = self.base_url.join(action).unwrap();
        let result = client.post(url).json(&data).send()?;
        println!("{:?}", result);
        Ok(())
    }
}

impl Client for NetworkedClient {
    // TODO implement these properly

    fn work_started(&self) {
        test_send(self, "/start", "client: work started");
    }

    fn work_ended(&self, data: WorkData) {
        test_send(self, "/end", "client: work ended");
    }
}

fn test_send(client: &NetworkedClient, action: &str, message: &str) {
    let test_data = json!({
        "content": message
    });

    let _ = client.send(action, test_data);
}

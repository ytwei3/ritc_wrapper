use reqwest::Error;
use serde_json::Value;

use crate::client::handle_respone;
use crate::client::RIT;

pub type JSON = Value;

pub trait Case {
    fn get_case(&self) -> Result<JSON, Error>;
    fn get_tick(&self) -> Result<i64, Error>;
}

impl Case for RIT {
    fn get_case(&self) -> Result<JSON, Error> {
        let resp = self.client.get("http://localhost:9999/v1/case").send()?;
        handle_respone(resp)
    }

    fn get_tick(&self) -> Result<i64, Error> {
        let resp = self.client.get("http://localhost:9999/v1/case").send()?;
        let tick = handle_respone(resp)?["tick"].as_i64().unwrap();

        Ok(tick)
    }
}

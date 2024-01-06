use serde_json::Value;

use crate::client::RIT;

pub type JSON = Value;

enum Security {
    ALL,
    TICKER(String),
}

pub trait OrderBook {
    // fn get_order_book(&self, ticker: &str) -> Result<JSON, reqwest::Error>;
    fn get_sercurity_info(&self, security: Security) -> Result<JSON, reqwest::Error>;
}

impl OrderBook for RIT {
    fn get_sercurity_info(&self, security: Security) -> Result<JSON, reqwest::Error> {
        todo!()
    }
}

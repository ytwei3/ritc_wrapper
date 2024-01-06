use reqwest::blocking::{Client, Response};
use reqwest::{self, Error, StatusCode};
use serde_json::Value;

pub type JSON = Value;

pub struct RIT {
    pub client: Client,
}

pub enum OrderType {
    MARKET,
    LIMIT(f64),
}

enum Security {
    ALL,
    TICKER(String),
}

impl RIT {
    pub fn new(key: &str) -> RIT {
        let mut headers = reqwest::header::HeaderMap::new();

        headers.insert(
            reqwest::header::HeaderName::from_static("x-api-key"),
            reqwest::header::HeaderValue::from_str(key).unwrap(),
        );

        let client = reqwest::blocking::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        RIT { client: client }
    }

    /// Case
    pub fn get_case(&self) -> Result<JSON, Error> {
        let resp = self.client.get("http://localhost:9999/v1/case").send()?;
        handle_respone(resp)
    }

    pub fn get_tick(&self) -> Result<i64, Error> {
        let resp = self.client.get("http://localhost:9999/v1/case").send()?;
        let tick = handle_respone(resp)?["tick"].as_i64().unwrap();

        Ok(tick)
    }

    /// Order
    pub fn post_order(
        &self,
        ticker: &str,
        order_type: OrderType,
        quantity: i32,
        action: &str,
    ) -> Result<(), Error> {
        let (order_type_str, price) = match order_type {
            OrderType::MARKET => ("MARKET", "".to_string()),
            OrderType::LIMIT(price) => ("LIMIT", price.to_string()),
        };
        let para = [
            ("ticker", ticker),
            ("type", order_type_str),
            ("quantity", &quantity.to_string()),
            ("action", action),
            ("price", &price),
        ];

        let resp = self
            .client
            .post("http://localhost:9999/v1/orders")
            .query(&para)
            .body("")
            .send()?;

        handle_respone(resp)?;
        Ok(())
    }

    /// Orderbook
    pub fn get_sercurity_info(&self, security: Security) -> Result<JSON, Error> {
        todo!()
    }
}

fn handle_respone(resp: Response) -> Result<JSON, Error> {
    match resp.status() {
        StatusCode::OK => resp.json::<JSON>(),
        StatusCode::UNAUTHORIZED => panic!("Unauthorized: check your API key"),
        _ => panic!("Failed to get tick"),
    }
}

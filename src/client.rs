use reqwest::blocking::{Client, Response};
use reqwest::StatusCode;
use serde_json::Value;

use std::fmt::Display;

use crate::{Action, OrderType, Security};

pub type Error = Box<dyn std::error::Error>;
pub type JSON = Value;

pub struct RIT {
    pub client: Client,
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

        RIT { client }
    }

    /// Case
    pub fn get_case(&self) -> Result<JSON, Error> {
        let resp = self.client.get("http://localhost:9999/v1/case").send()?;
        handle_respone(resp, None)
    }

    pub fn get_tick(&self) -> Result<i64, Error> {
        let resp = self.client.get("http://localhost:9999/v1/case").send()?;
        let tick = handle_respone(resp, None)?["tick"].as_i64().unwrap();

        Ok(tick)
    }

    /// Order
    pub fn post_order<T: Display>(
        &self,
        ticker: T,
        order_type: OrderType,
        quantity: i32,
        action: Action,
    ) -> Result<(), Error> {
        let url = format!(
            "http://localhost:9999/v1/orders?ticker={}&type={}&quantity={}&action={}",
            ticker, order_type, quantity, action
        );
        let resp = self.client.post(&url).body("").send()?;

        handle_respone(resp, Some((self, &url)))?;
        Ok(())
    }

    /// Orderbook
    pub fn get_sercurity_info(&self, security: Security) -> Result<JSON, Error> {
        let url = format!("http://localhost:9999/v1/securities{}", security);
        let resp = self.client.get(url).send()?;

        handle_respone(resp, None)
    }
}

pub fn sleep(secs: f64) {
    std::thread::sleep(std::time::Duration::from_secs_f64(secs))
}

fn handle_respone(resp: Response, retry: Option<(&RIT, &str)>) -> Result<JSON, Error> {
    match resp.status() {
        StatusCode::OK => match retry {
            Some(_) => Ok(JSON::Null),
            None => Ok(resp.json::<JSON>()?),
        },
        StatusCode::TOO_MANY_REQUESTS => {
            let json = resp.json::<JSON>()?;
            let wait = json.get("wait").unwrap().as_f64().unwrap();

            sleep(wait);
            retry
                .unwrap()
                .0
                .client
                .post(retry.unwrap().1)
                .body("")
                .send()?;

            Ok(JSON::Null)
        }
        _ => {
            let resp = resp.json::<JSON>()?;
            panic!(
                "++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++\n
                fail to handle response:\n{}\n
                ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++",
                resp["message"]
            )
        }
    }
}

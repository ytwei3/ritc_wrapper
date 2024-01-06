use reqwest::blocking::{Client, Response};
use reqwest::{self, Error, StatusCode};
use serde_json::Value;

pub use crate::case::Case;

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

        RIT { client: client }
    }
}

pub fn handle_respone(resp: Response) -> Result<JSON, Error> {
    match resp.status() {
        StatusCode::OK => resp.json::<JSON>(),
        StatusCode::UNAUTHORIZED => panic!("Unauthorized: check your API key"),
        _ => panic!("Failed to get tick"),
    }
}

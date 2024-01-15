use reqwest::StatusCode;
use reqwest::{Client, Response};
use serde_json::Value;

use std::fmt::Display;

use crate::{Action, OrderType, Security};

pub type Error = Box<dyn std::error::Error>;

pub fn new_client(key: &str) -> Client {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::HeaderName::from_static("x-api-key"),
        reqwest::header::HeaderValue::from_str(key).unwrap(),
    );

    reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap()
}

pub async fn get_case(client: &Client) -> Result<Value, Error> {
    let url = "http://localhost:9999/v1/case";
    let resp = client.get(url).send().await?;

    handle_response(resp).await
}

pub async fn get_tick(client: &Client) -> Result<i64, Error> {
    Ok(get_case(client).await?["tick"].as_i64().unwrap())
}

pub async fn get_sercurity_info(client: &Client, security: Security) -> Result<Value, Error> {
    let url = format!("http://localhost:9999/v1/securities{}", security);
    let resp = client.get(&url).send().await?;

    handle_response(resp).await
}

pub async fn get_tender(client: &Client) -> Result<Value, Error> {
    let url = "http://localhost:9999/v1/tender";
    let resp = client.get(url).send().await?;

    handle_response(resp).await
}

pub async fn accept_tender(client: &Client, tender_id: i32) -> Result<(), Error> {
    let url = format!("http://localhost:9999/v1/tender/{}", tender_id);
    let resp = client.post(&url).body("").send().await?;

    handle_response(resp).await?;
    Ok(())
}

pub async fn post_order<T: Display>(
    client: &Client,
    ticker: T,
    order_type: OrderType,
    quantity: i32,
    action: Action,
) -> Result<(), Error> {
    let url = format!(
        "http://localhost:9999/v1/orders?ticker={}&type={}&quantity={}&action={}",
        ticker, order_type, quantity, action
    );

    loop {
        let resp = client.post(&url).body("").send().await?;
        match resp.status() {
            StatusCode::OK => break,
            StatusCode::TOO_MANY_REQUESTS => {
                let json = resp.json::<Value>().await?;
                let wait = json.get("wait").unwrap().as_f64().unwrap();

                tokio::time::sleep(tokio::time::Duration::from_secs_f64(wait)).await;
                continue;
            }
            _ => {
                panic!(
                    "++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++\n
                    fail to post order:\n{}\n",
                    resp.text().await?
                )
            }
        }
    }

    Ok(())
}

async fn handle_response(resp: Response) -> Result<Value, Error> {
    match resp.status() {
        StatusCode::OK => Ok(resp.json::<Value>().await?),
        _ => {
            panic!(
                "++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++\n
                fail to post order:\n{}\n",
                resp.text().await?
            )
        }
    }
}

pub fn sleep(secs: f64) {
    std::thread::sleep(std::time::Duration::from_secs_f64(secs))
}

pub fn click_convertor(x: i32, y: i32) {
    //TODO: need to test whether all price will change at same time.
    todo!("find a suitable lib")
}

use reqwest::Error;

use crate::client::handle_respone;
use crate::client::RIT;

pub enum OrderType {
    MARKET,
    LIMIT(f64),
}

pub trait Order {
    fn post_order(
        &self,
        ticker: &str,
        order_type: OrderType,
        quantity: i32,
        action: &str,
    ) -> Result<(), Error>;
}

impl Order for RIT {
    fn post_order(
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
}

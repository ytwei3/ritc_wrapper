pub mod client;

pub use client::RIT;

pub enum Security {
    ALL,
    TICKER(String),
}

pub enum OrderType {
    MARKET,
    LIMIT(f64),
}

impl OrderType {
    fn as_str(&self) -> &'static str {
        match self {
            Self::MARKET => "MARKET",
            Self::LIMIT(_) => "LIMIT",
        }
    }

    fn price(&self) -> String {
        match self {
            Self::MARKET => "".to_string(),
            Self::LIMIT(price) => price.to_string(),
        }
    }
}

pub enum Action {
    BUY,
    SELL,
}

impl Action {
    fn as_str(&self) -> &'static str {
        match self {
            Self::BUY => "BUY",
            Self::SELL => "SELL",
        }
    }
}

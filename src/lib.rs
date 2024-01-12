pub use client::*;
use std::fmt::{Display, Formatter, Result};

pub mod client;

pub enum Security {
    ALL,
    TICKER(String),
}

impl Display for Security {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Security::ALL => write!(f, ""),
            Security::TICKER(t) => write!(f, "?ticker={}", t),
        }
    }
}

pub enum OrderType {
    MARKET,
    LIMIT(f64),
}

impl Display for OrderType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match *self {
            OrderType::MARKET => write!(f, "MARKET"),
            OrderType::LIMIT(price) => write!(f, "LIMIT&price={}", price),
        }
    }
}

pub enum Action {
    BUY,
    SELL,
}

impl Display for Action {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            &Action::BUY => write!(f, "BUY"),
            &Action::SELL => write!(f, "SELL"),
        }
    }
}

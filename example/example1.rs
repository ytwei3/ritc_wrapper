///an exmaple for mispricing arbitrage bewteen main and alternative market
use reqwest::Error;
use ritc::Action::*;
use ritc::OrderType::*;
use std::thread::sleep;
use std::time::Duration;

use ritc;

fn main() -> Result<(), Error> {
    let rit = ritc::RIT::new("114514");

    let mut tick = rit.get_tick();
    while 0 < tick && tick < 300 {
        let securities = rit.get_security_info();

        let crzy_m_bid = securities[0]["bid"].as_f64()?;
        let crzy_m_ask = securities[0]["ask"].as_f64()?;
        let crzy_a_bid = securities[1]["bid"].as_f64()?;
        let crzy_a_ask = securities[1]["ask"].as_f64()?;

        if crzy_m_bid > crzy_a_ask {
            rit.post_order("CRZY_A", MARKET, 5000, BUY)?;
            rit.post_order("CRZY_M", MARKET, 5000, SELL)?;

            sleep(Duration::from_millis(200));
        }

        if crzy_a_bid > crzy_m_ask {
            rit.post_order("CRZY_M", MARKET, 5000, BUY)?;
            rit.post_order("CRZY_A", MARKET, 5000, SELL)?;

            sleep(Duration::from_millis(200));
        }
    }
}

use std::thread::sleep;
use std::time::Duration;

use crate::ritc::rit::RIT;

pub fn algo_one(key: &str) {
    let rit = RIT::new(key);
    let tick = rit.get_tick();

    while 0 < tick && tick < 300 {
        let securities = rit.get_all_securities();

        let crzy_m_bid = securities[0]["bid"].as_f64().unwrap();
        let crzy_m_ask = securities[0]["ask"].as_f64().unwrap();
        let crzy_a_bid = securities[1]["bid"].as_f64().unwrap();
        let crzy_a_ask = securities[1]["ask"].as_f64().unwrap();

        if crzy_m_bid > crzy_a_ask {
            rit.post_market_order("CRZY_A", 5000, "BUY");
            rit.post_market_order("CRZY_M", 5000, "SELL");

            sleep(Duration::from_millis(200));
        }

        if crzy_a_bid > crzy_m_ask {
            rit.post_market_order("CRZY_M", 5000, "BUY");
            rit.post_market_order("CRZY_A", 5000, "SELL");

            sleep(Duration::from_millis(200));
        }
    }
}

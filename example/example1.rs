///an exmaple for mispricing arbitrage bewteen main and alternative market
extern crate ritc;
use ritc::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rit = ritc::RIT::new("114514");
    let a = AlgoOneTicker::CRZY_A.as_str();

    let mut tick = rit.get_tick();
    while 0 < tick && tick < 300 {
        let securities = rit.get_security_info(Security::ALL);

        let crzy_m_bid = securities[0]["bid"].as_f64()?;
        let crzy_m_ask = securities[0]["ask"].as_f64()?;
        let crzy_a_bid = securities[1]["bid"].as_f64()?;
        let crzy_a_ask = securities[1]["ask"].as_f64()?;

        if crzy_m_bid > crzy_a_ask {
            rit.post_order("CRZY_A", OrderType::MARKET, 5000, Action::BUY)?;
            rit.post_order("CRZY_M", OrderType::MARKET, 5000, Action::SELL)?;

            sleep(0.2)
        }

        if crzy_a_bid > crzy_m_ask {
            rit.post_order("CRZY_M", OrderType::MARKET, 5000, Action::BUY)?;
            rit.post_order("CRZY_A", OrderType::MARKET, 5000, Action::SELL)?;

            sleep(0.2)
        }
    }
}

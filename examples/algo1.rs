///an exmaple for mispricing arbitrage bewteen main and alternative market
use ritc::*;

fn main() -> Result<(), Error> {
    let rit = ritc::RIT::new("114514");

    let mut tick = rit.get_tick()?;
    while 0 < tick && tick < 300 {
        let securities = rit.get_sercurity_info(Security::ALL)?;

        let crzy_m_bid = securities[0]["bid"].as_f64().unwrap();
        let crzy_m_ask = securities[0]["ask"].as_f64().unwrap();
        let crzy_a_bid = securities[1]["bid"].as_f64().unwrap();
        let crzy_a_ask = securities[1]["ask"].as_f64().unwrap();

        if crzy_m_bid > crzy_a_ask {
            rit.post_order("CRZY_A", OrderType::MARKET, 5000, Action::BUY)?;
            rit.post_order("CRZY_M", OrderType::MARKET, 5000, Action::SELL)?;
        }

        if crzy_a_bid > crzy_m_ask {
            rit.post_order("CRZY_M", OrderType::MARKET, 5000, Action::BUY)?;
            rit.post_order("CRZY_A", OrderType::MARKET, 5000, Action::SELL)?;
        }
        tick = rit.get_tick()?;
        sleep(0.2);
    }
    Ok(())
}

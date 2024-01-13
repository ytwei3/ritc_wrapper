use ritc::*;

const ALGO: &str = "ALGO";

fn main() -> Result<(), Error> {
    let rit = RIT::new("114514");

    let mid_price = 20.0;
    let gap = 0.05;

    let mut tick = rit.get_tick()?;
    while 0 <= tick && tick <= 300 {
        rit.post_order(ALGO, OrderType::LIMIT(mid_price - gap), 5000, Action::BUY)?;
        rit.post_order(ALGO, OrderType::LIMIT(mid_price + gap), 5000, Action::SELL)?;

        tick = rit.get_tick()?;
    }

    Ok(())
}

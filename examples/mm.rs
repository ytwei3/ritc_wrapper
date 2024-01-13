use ritc::*;

const ALGO: &str = "ALGO";

fn main() -> Result<(), Error> {
    let rit = RIT::new("114514");

    let mid_price = 20.0;
    let gap = 0.05;

    let mut tick = rit.get_tick()?;
    while 0 <= tick && tick <= 300 {
        let start = std::time::Instant::now();
        rit.post_order(ALGO, OrderType::LIMIT(mid_price - gap), 5000, Action::BUY)?;
        rit.post_order(ALGO, OrderType::LIMIT(mid_price + gap), 5000, Action::SELL)?;
        let elapsed = start.elapsed();
        println!("{}.{:03} sec", elapsed.as_secs(), elapsed.subsec_millis());

        tick = rit.get_tick()?;
    }

    Ok(())
}

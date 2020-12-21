use chrono::Duration;

pub mod wanted_thing;
use wanted_thing::WantedThing;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut test_item: WantedThing = WantedThing::new(
        "https://www.amazon.com/Watch-Dogs-Legion-PlayStation-5-Standard/dp/B08FS6BB9N/ref=sr_1_2?dchild=1&keywords=ps5+game&qid=1608266021&sr=8-2".to_string(), 
        Duration::seconds(2)
    );

    test_item.details();
    test_item.check_for_price_drop().await?;
    Ok(())
}

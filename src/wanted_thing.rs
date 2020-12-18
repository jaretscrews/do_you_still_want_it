use url::Url;
use chrono::{DateTime, Duration};
use chrono::offset::Utc;
use reqwest;
use scraper::{Html, Selector};
use rusty_money::Money;
use futures::executor::block_on;

pub struct WantedThing {
    name: String,
    item_url: Url,
    price: Money,
    time_to_check: DateTime<Utc>,
}

pub async fn check_price(item_url: Url) -> Result<Money, Box<dyn std::error::Error>> {
    let body = reqwest::get(item_url.clone()).await?.text().await?;
    let doc = Html::parse_document(&body);
    let selector = Selector::parse("#priceblock_ourprice").unwrap(); 
    for element in doc.select(&selector){
        let mut price = element.text().collect::<Vec<_>>()[0].to_string();
        price.remove(0);
        println!("price: {}", price);
        return Ok(Money::from_str(&price, "USD").unwrap());
    }
    return Ok(Money::from_str("0.00", "USD").unwrap());
}

impl WantedThing {
    pub fn new(name: String, item_url: String, duration: Duration) -> Self {
        let parsed_url = Url::parse(&item_url).unwrap();
        return WantedThing {
            name: name,
            item_url: parsed_url.clone(),
            price: block_on(check_price(parsed_url.clone())).unwrap(),
            time_to_check: Utc::now() + duration,
        }
    }
    pub fn details(&self) {
        println!("Name: {}", self.name);
        println!("URL: {}", self.item_url);
        println!("Price: {}", self.price);
        println!("Time to check: {}", self.time_to_check);
    }

    pub fn has_check_passed(&self) -> bool {
        return Utc::now() > self.time_to_check;
    }
}

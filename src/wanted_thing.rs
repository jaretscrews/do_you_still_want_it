use url::Url;
use chrono::{DateTime, Duration};
use chrono::offset::Utc;
use reqwest;
use scraper::{Html, Selector};
use rusty_money::{Money, Currency};
use futures::executor::block_on;

pub struct WantedThing {
    name: String,
    item_url: Url,
    price: Money,
    time_to_check: DateTime<Utc>,
}

//Pulls down the html to check to see if the price has changed
//TODO move this into the struct
async fn check_price(item_url: Url) -> Result<Money, Box<dyn std::error::Error>> {
    //Get the html
    let body = reqwest::get(item_url.clone()).await?.text().await?;
    //parse html
    let doc = Html::parse_document(&body);
    //selector for the price
    let selector = Selector::parse("#priceblock_ourprice").unwrap(); 
    for element in doc.select(&selector){
        let mut price = element.text().collect::<Vec<_>>()[0].to_string();
        price.remove(0);
        return Ok(Money::from_str(&price, "USD").unwrap());
    }
    //TODO figure out how to return an error you dummy
    return Ok(Money::from_str("0.00", "USD").unwrap());
}

//TODO move this into the struct
async fn get_thing_from_link(item_url: Url, duration: Duration) -> Result<WantedThing, Box<dyn std::error::Error>> {
    let mut name: String = "".to_string();
    let mut price: Money = Money::new(0, Currency::from_string("USD".to_string()).unwrap());


    //Get the html
    let body = reqwest::get(item_url.clone()).await?.text().await?;
    //parse that shit
    let doc = Html::parse_document(&body);
    //look for the price then the name
    let selector = Selector::parse("#priceblock_ourprice").unwrap(); 
    for element in doc.select(&selector){
        let mut price_txt = element.text().collect::<Vec<_>>()[0].to_string();
        price_txt.remove(0);
        price = Money::from_str(&price_txt, "USD").unwrap();
    }

    let selector = Selector::parse("#productTitle").unwrap(); 
    for element in doc.select(&selector){
        name = element.text().collect::<Vec<_>>()[0].to_string();
    }

    //return the struct formed from the information in the doc
    return Ok(WantedThing{
        //remove extra white space from the names
        name: name.replace("\n",""),
        item_url: item_url,
        price: price,
        time_to_check: Utc::now() + duration,
    })
}

impl WantedThing {
    pub fn new(item_url: String, duration: Duration) -> Self {
        let parsed_url = Url::parse(&item_url).unwrap();
        return block_on(get_thing_from_link(parsed_url, duration)).unwrap();
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

    pub async fn check_for_price_drop(& mut self) -> Result<(), Box< dyn std::error::Error>> {
        let new_price = check_price(self.item_url.clone()).await?;
        if new_price < self.price {
            //TODO send an email that the price has dropped
            //maybe make it send an email if it has dropped by a certain percentage
            println!("The price is lower");
            self.price = new_price;
        }

        Ok(())
    }
}

use url::Url;
use chrono::{DateTime, Duration};
use chrono::offset::Utc;
use reqwest;
use scraper::{Html, Selector};

pub struct WantedThing {
    name: String,
    url: Url,
    time_to_check: DateTime<Utc>,
}

impl WantedThing {
    pub fn new(name: String, url: String, duration: Duration) -> Self {
        return WantedThing {
            name: name,
            url: Url::parse(&url).unwrap(),
            time_to_check: Utc::now() + duration,
        }
    }
    pub fn details(&self) {
        println!("Name: {}", self.name);
        println!("URL: {}", self.url);
        println!("Time to check: {}", self.time_to_check);
    }

    pub fn has_check_passed(&self) -> bool {
        return Utc::now() > self.time_to_check;
    }

    pub async fn check_url(&self) -> Result<(), Box<dyn std::error::Error>> {
        let body = reqwest::get(self.url.clone()).await?.text().await?;
        let doc = Html::parse_document(&body);
        let selector = Selector::parse("#priceblock_ourprice").unwrap(); 
        for element in doc.select(&selector){
            let price = element.text().collect::<Vec<_>>();
            println!("{:?}", price);
        }
        Ok(())
    }
}

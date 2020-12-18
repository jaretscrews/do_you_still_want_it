use std::collections::HashMap;

use url::Url;
use chrono::{DateTime, Duration};
use chrono::offset::Utc;
use reqwest;

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
        let resp = reqwest::get("https://httpbin.org/ip")
            .await?
            .json::<HashMap<String, String>>()
            .await?;
        println!("{:#?}", resp);
        Ok(())
    }
}

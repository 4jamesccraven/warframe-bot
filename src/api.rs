use std::boxed::Box;
use std::collections::HashMap;
use std::error::Error;

use reqwest as rq;
use serde::{Deserialize, Serialize};
use serde_json;


#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct NewsItem {
    pub id: String,
    pub message: String,
    pub link: String,
    pub imageLink: String,
    pub priority: bool,
    pub date: String,
    pub eta: String,
    pub update: bool,
    pub primeAccess: bool,
    pub stream: bool,
    pub translations: HashMap<String, String>,
    pub asString: String,
}

impl NewsItem {
    pub async fn get_all() -> Result<Vec<Self>, Box<dyn Error>> {
        let url = "https://api.warframestat.us/pc/news";
        let response = rq::get(url)
            .await?
            .text()
            .await?;

        let news: Vec<Self> = serde_json::from_str(&response)?;

        Ok(news)
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct VoidTrader {
    id: String,
    activation: String,
    expiry: String,
    startString: String,
    active: bool,
    character: String,
    location: String,
    inventory: Vec<TradeItem>,
    psId: String,
    endString: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TradeItem {
    pub item: String,
    pub ducats: i16,
    pub credits: i32,
}

impl VoidTrader {
    pub async fn get() -> Result<Self, Box<dyn Error>> {
        let url = "https://api.warframestat.us/pc/voidTrader";
        let response = rq::get(url)
            .await?
            .text()
            .await?;

        let trader = serde_json::from_str(&response)?;

        Ok(trader)
    }
}

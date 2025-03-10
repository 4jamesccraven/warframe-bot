use crate::discord::ToDiscordMessage;

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
        let response = rq::get(url).await?.text().await?;

        let news: Vec<Self> = serde_json::from_str(&response)?;

        Ok(news)
    }
}

impl ToDiscordMessage for NewsItem {
    fn message(&self) -> String {
        self.asString.clone()
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
        let response = rq::get(url).await?.text().await?;

        let trader = serde_json::from_str(&response)?;

        Ok(trader)
    }
}

impl ToDiscordMessage for VoidTrader {
    fn message(&self) -> String {
        match self.active {
            true => {
                let end_date = crate::date::api_to_chrono(&self.expiry)
                    .expect("Error, cannot map date in `VoidTrader::message`");
                let end_date = end_date.format("%B %d at %T");

                let inventory_strings: Vec<_> = self
                    .inventory
                    .iter()
                    .map(|i| {
                        (
                            i.item.clone(),
                            format!("{}", i.credits),
                            format!("{}", i.ducats),
                        )
                    })
                    .collect();

                let maxes: Vec<_> = (0..3)
                    .map(|i| {
                        inventory_strings
                            .iter()
                            .map(|item| match i {
                                0 => item.0.len(),
                                1 => item.1.len(),
                                2 => item.2.len(),
                                _ => unreachable!(),
                            })
                            .max()
                            .unwrap()
                    })
                    .collect();

                let (item_max, cred_max, duc_max) = (maxes[0], maxes[1], maxes[2]);

                let inventory = inventory_strings
                    .iter()
                    .map(|(item, credit, ducat)| {
                        format!(
                            "{item:imax$}  {credit:cmax$}  {ducat:dmax$}",
                            imax = item_max,
                            cmax = cred_max,
                            dmax = duc_max
                        )
                    })
                    .collect::<Vec<String>>()
                    .join("\n");

                format!(
                    "{} is in {} until {}:\n{}",
                    self.character, self.location, end_date, inventory
                )
            }
            false => {
                let begin_date = crate::date::api_to_chrono(&self.activation)
                    .expect("Error, cannot map date in `VoidTrader::message`");
                let begin_date = begin_date.format("%B %d at %T");

                format!(
                    "{} will be in {} on {}",
                    self.character, self.location, begin_date
                )
            }
        }
    }
}

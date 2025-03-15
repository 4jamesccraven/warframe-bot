use super::*;

use std::sync::Arc;

use serenity::all::Http;
use tokio::time::{sleep, Duration};

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct VoidTrader {
    pub id: String,
    pub activation: String,
    pub expiry: String,
    pub startString: String,
    pub active: bool,
    pub character: String,
    pub location: String,
    pub inventory: Vec<TradeItem>,
    pub psId: String,
    pub endString: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TradeItem {
    pub item: String,
    pub ducats: i16,
    pub credits: i32,
}

impl VoidTrader {
    pub async fn get() -> Result<Self, Error> {
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
                    .expect("[ERROR]: unable to map api date (1)");
                let end_date = end_date.format("%B %d at %T");

                let mut inventory_strings: Vec<_> = self
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

                let header: (String, String, String) =
                    ("Item".into(), "Credits".into(), "Ducats".into());

                inventory_strings.insert(0, header);

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

                let dividers: (String, String, String) = (
                    "-".repeat(item_max),
                    "-".repeat(cred_max),
                    "-".repeat(duc_max),
                );

                inventory_strings.insert(1, dividers);

                let inventory = inventory_strings
                    .iter()
                    .map(|(item, credit, ducat)| {
                        format!(
                            "{item:item_max$}  {credit:>cred_max$}  {ducat:>duc_max$}",
                            item_max = item_max,
                            cred_max = cred_max,
                            duc_max = duc_max
                        )
                    })
                    .collect::<Vec<String>>()
                    .join("\n");

                format!(
                    "{} is in {} until {}:```\n{}```",
                    self.character, self.location, end_date, inventory
                )
            }
            false => {
                let begin_date = crate::date::api_to_chrono(&self.activation)
                    .expect("[ERROR]: unable to map api date (2)");
                let begin_date = begin_date.format("%B %d at %T");

                format!(
                    "{} will be in {} on {}",
                    self.character, self.location, begin_date
                )
            }
        }
    }
}

pub async fn handle_baro() -> String {
    match VoidTrader::get().await {
        Ok(trader) => {
            let content = trader.message();

            content
        }
        Err(why) => {
            eprintln!("[ERROR]: could not fetch Trader info from api: {why:?}");

            let message = "Unable to get information on the void trader. \
            Please try again.";

            message.into()
        }
    }
}

pub async fn baro_loop(http: Arc<Http>, channel_id: ChannelId) {
    loop {
        match VoidTrader::get().await {
            Ok(trader) => match trader.active {
                true => {
                    if let Err(why) = channel_id.say(&http, trader.message()).await {
                        eprintln!("[ERROR]: failed to post void trader inventory: {why:?}");
                    }
                    sleep(Duration::from_secs(172800)).await;
                }
                false => {
                    eprintln!("[INFO]: Trader inactive.");
                    sleep(Duration::from_secs(86400)).await;
                }
            },
            Err(why) => {
                eprintln!("[ERROR]: unable to get void trader info from api.\n{why:?}\n Retrying in 1m...");
                sleep(Duration::from_secs(60)).await;
            }
        }
    }
}

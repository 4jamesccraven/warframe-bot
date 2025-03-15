use super::*;

use crate::date::within_24_hrs;

use std::sync::Arc;

use serenity::all::{ChannelId, Http};
use tokio::time::{Duration, sleep};

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
    pub async fn get_all() -> Result<Vec<Self>, Error> {
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

async fn handle_news(cache: Cache, http: Arc<Http>, channel_id: &ChannelId) {
    let news_items = NewsItem::get_all().await;
    if let Err(why) = news_items {
        eprintln!("Error getting news: {why:?}");
        return;
    }
    let news_items = news_items.unwrap();
    let mut seen_items = cache.seen_news.lock().await;

    let message = news_items
        .iter()
        .filter(|item| {
            let seen = seen_items.contains(&item.id);
            if !seen {
                seen_items.insert(item.id.clone());
            }

            let recent = within_24_hrs(&item.date);

            !seen && recent
        })
        .map(|item| item.asString.clone())
        .collect::<Vec<_>>()
        .join("\n");

    if let Err(why) = channel_id.say(&http, message).await {
        eprintln!("Error sending news items: {why:?}");
    }
}

pub async fn news_loop(cache: Cache, http: Arc<Http>, channel_id: ChannelId) {
    loop {
        handle_news(cache.clone(), Arc::clone(&http), &channel_id).await;
        sleep(Duration::from_secs(86400)).await;
    }
}

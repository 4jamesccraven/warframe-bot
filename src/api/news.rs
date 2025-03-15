use super::*;

use crate::date::{api_to_chrono, within_5_days};

use std::sync::Arc;

use serenity::all::{ChannelId, Http};
use tokio::time::{sleep, Duration};

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

    pub fn find_link_text(message: &str) -> Option<String> {
        let l = message.rfind('[')? + 1;
        let r = message.rfind(']')? - 1;

        Some(message[l..=r].into())
    }
}

impl ToDiscordMessage for NewsItem {
    fn message(&self) -> String {
        let date = match api_to_chrono(&self.date) {
            Some(dt) => format!("[{}] ", dt.format("%a, %b %d")),
            None => String::new(),
        };

        format!("{}[{}]({})", date, self.message, self.link)
    }
}

async fn handle_news(cache: Cache, http: Arc<Http>, channel_id: &ChannelId) {
    let news_items = NewsItem::get_all().await;
    if let Err(why) = news_items {
        eprintln!("[ERROR]: unable to get news: {why:?}");
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

            let recent = within_5_days(&item.date);

            !seen && recent
        })
        .map(|item| item.message())
        .collect::<Vec<_>>()
        .join("\n");

    if message.is_empty() {
        eprintln!("[INFO]: no unseen news items");
        return;
    }

    if let Err(why) = channel_id.say(&http, message).await {
        eprintln!("[ERROR]: could not post news: {why:?}");
    }
}

pub async fn news_loop(cache: Cache, http: Arc<Http>, channel_id: ChannelId) {
    loop {
        handle_news(cache.clone(), Arc::clone(&http), &channel_id).await;
        sleep(Duration::from_secs(3600)).await;
    }
}

mod news;
mod trader;

pub use news::*;
pub use trader::*;

use crate::discord::ToDiscordMessage;

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use anyhow::Error;
use reqwest as rq;
use serde::{Deserialize, Serialize};
use serde_json;
use serenity::all::{ChannelId, Client};
use serenity::builder::GetMessages;
use tokio::sync::Mutex;

#[derive(Clone, Default)]
pub struct Cache {
    seen_news: Arc<Mutex<HashSet<String>>>,
}

impl Cache {
    pub async fn update_news_from_channel(&self, client: &Client, channel_id: &ChannelId) {
        let retriever = GetMessages::new().limit(100);
        let messages = channel_id.messages(&client.http, retriever).await.unwrap();
        let msg_to_id: HashMap<String, String>;

        match NewsItem::get_all().await {
            Ok(news) => {
                msg_to_id = news
                    .iter()
                    .filter_map(|item| {
                        let msg = NewsItem::find_link_text(&item.asString)?;
                        Some((msg, item.id.clone()))
                    })
                    .collect();
            },
            Err(why) => {
                eprintln!("[ERROR]: could not get news items: {why:?}");
                return;
            }
        }

        let message_contents: Vec<_> = messages
            .iter()
            .flat_map(|m| m.content.lines())
            .filter_map(NewsItem::find_link_text)
            .collect();

        for msg in message_contents {
            if let Some(id) = msg_to_id.get(&msg) {
                self.seen_news.lock().await.insert(id.to_owned());
            }
        }
    }
}

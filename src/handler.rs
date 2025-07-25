use crate::cache::SeenCache;
use crate::News;

use std::sync::{Arc, Mutex};

use serenity::all::{ChannelId, Http, Message};
use serenity::async_trait;
use serenity::prelude::*;
use warframe::worldstate::client::Client;
use warframe::worldstate::{queryable, Queryable};

#[derive(Debug, Clone)]
pub struct Handler {
    news_cache: Arc<Mutex<SeenCache<News, 20>>>,
    channel_id: ChannelId,
    connection: Arc<Http>,
    worldstate: Client,
}

impl Handler {
    pub fn new(connection: Arc<Http>, channel_id: ChannelId) -> Self {
        Self {
            news_cache: Arc::new(Mutex::new(SeenCache::default())),
            connection,
            channel_id,
            worldstate: Client::new(),
        }
    }

    /// Send a message to the news channel with currently unseen news items.
    pub async fn notify_news(&self) {
        let news: Vec<News> = match self.worldstate.fetch::<queryable::News>().await {
            Ok(response) => response.into_iter().map(News::from).collect(),
            Err(e) => {
                eprintln!("Fail to fetch news: {}", e);
                return;
            }
        };

        let news = self.news_cache.lock().unwrap().difference(&news);

        for news_item in news {
            if let Ok(msg) = news_item.as_message() {
                // println!("{}", msg);
                self.channel_id.say(&self.connection, msg).await.unwrap();
            }
        }
    }

    /// Send a message or messages to the news channel with information about Baro Ki'Teer's
    /// current or next visit.
    pub async fn notify_baro(&self) {
        todo!()
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, _ctx: Context, msg: Message) {
        let content = msg.content;

        match content.as_str() {
            "!baro" => self.notify_baro().await,
            "!help" => todo!(),
            "!news" => self.notify_news().await,
            _ => {}
        }
    }
}

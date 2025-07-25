use crate::cache::SeenCache;
use crate::News;

use std::sync::Arc;

use serenity::all::{ChannelId, Http, Message};
use serenity::async_trait;
use serenity::prelude::*;
use tokio::sync::Mutex;
use warframe::worldstate::client::Client;
use warframe::worldstate::queryable;

#[derive(Debug, Clone)]
pub struct Handler {
    channel_id: ChannelId,
    connection: Arc<Mutex<Option<Arc<Http>>>>,
    news_cache: Arc<Mutex<SeenCache<News, 20>>>,
    worldstate: Client,
}

impl Handler {
    pub fn new(channel_id: ChannelId) -> Self {
        Self {
            channel_id,
            connection: Arc::new(Mutex::new(None)),
            news_cache: Arc::new(Mutex::new(SeenCache::new())),
            worldstate: Client::new(),
        }
    }

    /// Initialise the co
    pub async fn init_connection(&self, connection: Arc<Http>) {
        *self.connection.lock().await = Some(connection);
    }

    /// Send a message to the news channel with currently unseen news items.
    pub async fn notify_news(&self) {
        // Fetch the recent news, and map it into the correct type
        let news: Vec<News> = match self.worldstate.fetch::<queryable::News>().await {
            Ok(response) => response
                .into_iter()
                .filter_map(|news_item| {
                    let mapped = News::from(news_item);

                    // Ignore news that is alway active, e.g., "Join the Warframe Discord!"
                    white_listed(&mapped).then_some(mapped)
                })
                .collect(),

            Err(e) => {
                eprintln!("[error fetching news]: {e}");
                return;
            }
        };

        let mut cache = self.news_cache.lock().await;
        let connection: Arc<Http> = self.connection().await;

        let news = cache.difference(&news);

        for news_item in news {
            if let Ok(msg) = news_item.as_message() {
                self.channel_id.say(&connection, msg).await.unwrap();
            }
        }

        match cache.dump() {
            Ok(unit) => unit,
            Err(e) => eprint!("[error dumping cache]: {e}"),
        }
    }

    /// Send a message or messages to the news channel with information about Baro Ki'Teer's
    /// current or next visit.
    pub async fn notify_baro(&self) {
        todo!()
    }

    async fn show_help(&self, channel_id: &ChannelId) {
        let help_message = "Available Commands:\n\
                            - !baro: Show when baro will be here next, or his inventory if he's here\n\
                            - !news: Show unseen news\n\
                            - !help: Print this message";

        channel_id
            .say(self.connection().await, help_message)
            .await
            .unwrap();
    }

    async fn connection(&self) -> Arc<Http> {
        self.connection
            .lock()
            .await
            .clone()
            .expect("[Error - internal]: attempt to use event handler without connection.")
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, _ctx: Context, msg: Message) {
        let content = msg.content;

        match content.as_str() {
            "!baro" => self.notify_baro().await,
            "!help" => self.show_help(&msg.channel_id).await,
            "!news" => self.notify_news().await,
            _ => {}
        }
    }
}

fn white_listed(news_item: &News) -> bool {
    !crate::BLACKLIST.contains(news_item)
}

use crate::cache::SeenCache;
use crate::item_display::calculate_baro_string;
use crate::News;

use std::sync::Arc;

use serenity::all::{ChannelId, Http, Message};
use serenity::async_trait;
use serenity::prelude::*;
use tokio::sync::Mutex;
use warframe::worldstate::client::Client;
use warframe::worldstate::{queryable, TimedEvent};

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
            news_cache: Arc::new(Mutex::new(SeenCache::new("NEWS"))),
            worldstate: Client::new(),
        }
    }

    /// Initialise the connection to the Discord Client.
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
                eprintln!("[Error -- fetching news]: {e}");
                return;
            }
        };

        // Get a handle for the cache and the Discord connection.
        let mut cache = self.news_cache.lock().await;

        // Get the subset of news listings that have not been seen thus far.
        let news = cache.difference(&news);

        // If there's nothing to report, we log it and move on.
        if news.is_empty() {
            println!("[info]: no unseen news");
            return;
        }

        // Send a message for each.
        let messages = news
            .into_iter()
            .filter_map(|news_item| {
                news_item
                    .as_message()
                    .inspect_err(|e| eprintln!("[Error -- Formatting news]: {e}"))
                    .ok()
            })
            .collect::<Vec<_>>();
        self.say_multiple(&messages).await;

        // Update the cache.
        match cache.dump() {
            Ok(unit) => unit,
            Err(e) => eprintln!("[Error -- dumping cache]: {e}"),
        }
    }

    /// Returns `true` if Baro Ki'Teer is active.
    pub async fn check_baro(&self) -> bool {
        let trader = match self.worldstate.fetch::<queryable::VoidTrader>().await {
            Ok(trader) => trader,
            Err(e) => {
                eprint!("[Error -- fetching trader]: {e}");
                return false;
            }
        };

        trader.active()
    }

    /// Send a message or messages to the news channel with information about Baro Ki'Teer's
    /// current or next visit.
    pub async fn notify_baro(&self) {
        // Fetch the most recent information about Baro Ki'Teer.
        let trader = match self.worldstate.fetch::<queryable::VoidTrader>().await {
            Ok(trader) => trader,
            Err(e) => {
                eprint!("[Error -- fetching trader]: {e}");
                return;
            }
        };

        // Construct the messages, and prepare the connecion.
        let messages = calculate_baro_string(&trader).await;

        self.say_multiple(&messages).await;
    }

    /// Send a message displaying the bot's capabilities
    async fn show_help(&self, channel_id: &ChannelId) {
        let help_message = "Available Commands:\n\
                            - !baro: Show when baro will be here next, or his inventory if he's here\n\
                            - !news: Show unseen news\n\
                            - !help: Print this message";

        match channel_id.say(self.connection().await, help_message).await {
            Ok(_) => {}
            Err(e) => eprintln!("[Error -- sending message]: {e}"),
        }
    }

    /// Get the cached connection.
    async fn connection(&self) -> Arc<Http> {
        self.connection
            .lock()
            .await
            .clone()
            .expect("[Error -- internal]: attempt to use event handler without connection.")
    }

    /// Write multiple messages to the news channel.
    async fn say_multiple(&self, contents: &[String]) {
        let connection = self.connection().await;
        for msg in contents.iter() {
            match self.channel_id.say(&connection, msg).await {
                Ok(_) => {}
                Err(e) => eprintln!("[Error -- sending message]: {e}"),
            }
        }
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

/// Returns `true` if the news_listing is blacklisted from being cached or sent to the channel.
fn white_listed(news_item: &News) -> bool {
    !crate::BLACKLIST.contains(news_item)
}

use crate::News;
use crate::cache::SeenCache;
use crate::item_display::{WeeklyInfo, calculate_baro_string};
use crate::{error, info, warning};

use std::sync::Arc;

use poise::serenity_prelude::{ChannelId, Http};
use tokio::sync::Mutex;
use warframe::worldstate::client::Client;
use warframe::worldstate::{TimedEvent, queryable};

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

    /// Returns a list of news items as messages. Empty if no news items were found.
    pub async fn news_messages(&self) -> Vec<String> {
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
                warning!(context = "fetching news", "{e}");
                return vec![];
            }
        };

        // Get a handle for the cache.
        let mut cache = self.news_cache.lock().await;

        // Get the subset of news listings that have not been seen thus far.
        let news = cache.difference(&news);

        // If there's nothing to report, we log it and move on.
        if news.is_empty() {
            return vec![];
        }

        // Update the cache.
        if let Err(e) = cache.dump() {
            warning!(context = "dumping cache", "{e}");
        }

        // If there are news items, map them into messages for consumption by other functions.
        news.into_iter()
            .filter_map(|news_item| {
                news_item
                    .as_message()
                    .inspect_err(|e| warning!(context = "formatting news", "{e}"))
                    .ok()
            })
            .collect::<Vec<_>>()
    }

    /// Send messages to the news channel with currently unseen news items.
    pub async fn notify_news(&self) {
        let messages = self.news_messages().await;
        if messages.is_empty() {
            info!("no unseen news");
            return;
        }

        self.say_multiple(&messages).await;
    }

    /// Returns `true` if Baro Ki'Teer is active.
    pub async fn check_baro(&self) -> bool {
        let trader = match self.worldstate.fetch::<queryable::VoidTrader>().await {
            Ok(trader) => trader,
            Err(e) => {
                warning!(context = "fetching trader", "{e}");
                return false;
            }
        };

        trader.active()
    }

    /// Utility function that fetches the active trader information and passes it along to the
    /// formatter function.
    pub async fn baro_messages(&self) -> Vec<String> {
        // Fetch the most recent information about Baro Ki'Teer.
        let trader = match self.worldstate.fetch::<queryable::VoidTrader>().await {
            Ok(trader) => trader,
            Err(e) => {
                warning!(context = "fetching trader", "{e}");
                return vec![];
            }
        };

        // Construct the messages
        calculate_baro_string(&trader).await
    }

    /// Send a message or messages to the news channel with information about Baro Ki'Teer's
    /// current or next visit.
    pub async fn notify_baro(&self) {
        let messages = self.baro_messages().await;

        self.say_multiple(&messages).await;
    }

    /// Create the weekly reset message.
    pub async fn weekly_messages(&self) -> Option<String> {
        let archon_boss = self
            .worldstate
            .fetch::<queryable::ArchonHunt>()
            .await
            .inspect_err(|e| warning!(context = "fetching archon", "{e}"))
            .ok()?
            .boss;

        let info = WeeklyInfo::new(&archon_boss);

        Some(info.as_message())
    }

    /// Send a message summarising the weekly reset to the news channel.
    pub async fn notify_weekly(&self) {
        let message = match self.weekly_messages().await {
            Some(m) => m,
            None => return,
        };

        self.say_multiple(&[message]).await;
    }

    /// Get the cached connection.
    async fn connection(&self) -> Arc<Http> {
        match self.connection.lock().await.clone() {
            Some(connection) => connection,
            None => {
                error!(
                    context = "internal",
                    "attempt to use handler without connection"
                );
                std::process::exit(1);
            }
        }
    }

    /// Write multiple messages to the news channel.
    async fn say_multiple(&self, contents: &[String]) {
        let connection = self.connection().await;
        for msg in contents.iter() {
            if let Err(e) = self.channel_id.say(&connection, msg).await {
                warning!(context = "sending message", "{e}");
            }
        }
    }
}

/// Returns `true` if the news_listing is blacklisted from being cached or sent to the channel.
fn white_listed(news_item: &News) -> bool {
    !crate::BLACKLIST.contains(news_item)
}

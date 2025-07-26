mod blacklist;
mod cache;
pub mod cli;
pub mod handler;
mod item_display;
mod news_wrapper;
pub mod periodic;

pub use blacklist::BLACKLIST;
#[allow(unused)]
pub use news_wrapper::*;

use cli::Cli;

use std::env;
use std::sync::Arc;

use anyhow::{Context, Result};

/// Initialise the Discord bot client.
pub async fn init_bot(args: &Cli, handler: Arc<handler::Handler>) -> Result<serenity::Client> {
    use serenity::prelude::*;
    use serenity::Client;
    let intents = GatewayIntents::default()
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let client = Client::builder(&args.api_token, intents)
        .event_handler_arc(handler)
        .await?;

    Ok(client)
}

/// Load the Discord Channel ID from an environment variable.
pub fn load_channel_id() -> Result<u64> {
    const ERR_MESG: &str = "A valid Discord Channel ID is required to run wf-bot.";
    let channel_id = env::var("WF_CHANNELID").context(ERR_MESG)?;

    Ok(channel_id.parse()?)
}

/// Format the date style given by the warframe API.
pub fn fmt_api_date(date: &chrono::DateTime<chrono::Utc>) -> Result<String> {
    let local = date.with_timezone(&chrono::Local);
    Ok(format!("{}", local.format("%a, %b %d")))
}

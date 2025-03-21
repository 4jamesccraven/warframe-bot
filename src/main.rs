mod api;
mod date;
mod discord;

use api::{baro_loop, news_loop, Cache};
use discord::Handler;

use std::boxed::Box;
use std::env;
use std::error::Error;
use std::sync::Arc;

use serenity::all::ChannelId;
use serenity::prelude::*;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let token_err = "Unable to get discord application token. \
                     Please ensure that the environment variable \
                     `TOKEN` is available or present in ./.env";

    let token = env::var("TOKEN").expect(token_err);

    let news_err = "Warning: `NEWS_CHANNEL_ID` not present in environment. \
                    News updates will not be sent.";

    let news_channel_id = env::var("NEWS_CHANNEL_ID");

    let cache = Cache::default();

    let intents = GatewayIntents::default()
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await?;

    match news_channel_id {
        Ok(id) => {
            let channel_id = ChannelId::new(id.parse::<u64>().expect("Invalid Channel Id"));
            let news_http = Arc::clone(&client.http);
            let baro_http = Arc::clone(&client.http);

            cache.update_news_from_channel(&client, &channel_id).await;

            tokio::spawn(news_loop(cache.clone(), news_http, channel_id));
            tokio::spawn(baro_loop(baro_http, channel_id));
        }
        Err(_) => {
            eprintln!("{}", news_err);
        }
    }

    if let Err(why) = client.start().await {
        eprintln!("Client error: {why:?}");
    }

    Ok(())
}

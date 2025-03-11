use crate::api::{VoidTrader, NewsItem};

use serenity::async_trait;
use serenity::model::channel::Message;
// use serenity::model::gateway::Ready;
use serenity::prelude::*;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let response: Option<String> = match msg.content.as_str() {
            "!baro" => {
                if let Ok(trader) = VoidTrader::get().await {
                    let content = trader.message();

                    Some(content)
                } else {
                    let msg = "Unable to get information on the void trader. \
                    Please try again";

                    Some(msg.into())
                }
            },
            response if response.starts_with("!") => {
                let msg = "Unknown command";

                Some(msg.into())
            },
            _ => {
                None
            },
        };

        if let Some(response) = response {
            if let Err(why) = msg.channel_id.say(&ctx.http, response).await {
                println!("Error sending message: {why:?}");
            }
        }
    }
}

pub trait ToDiscordMessage {
    fn message(&self) -> String;
}

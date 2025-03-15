mod utils;

use crate::api::handle_baro;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let response: Option<String> = match msg.content.as_str() {
            "!baro" => {
                let message = handle_baro().await;
                Some(message)
            },
            "!clear" => {
                if let Some(channel) = msg.channel_id.to_channel(&ctx.http).await.ok() {
                    utils::clear_messages(&ctx, channel.id()).await;
                }
                None
            },
            response if response.starts_with("!") => {
                let msg = "Unknown command";
                Some(msg.into())
            },
            _ => None,
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

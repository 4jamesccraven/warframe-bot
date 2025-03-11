mod api;
mod date;
mod discord;

use discord::Handler;

use std::boxed::Box;
use std::env;
use std::error::Error;

use dotenv::dotenv;
use serenity::prelude::*;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _ = dotenv()?;

    let token_err = "Unable to get discord application token. \
                     Please ensure that the environment variable \
                     `TOKEN` is available or present in /.env";

    let token = env::var("TOKEN")
        .expect(token_err);

    let intents = GatewayIntents::default()
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler).await?;

    if let Err(why) = client.start().await {
        eprintln!("Client error: {why:?}");
    }

    Ok(())
}

use std::sync::Arc;

use anyhow::Ok;
use clap::Parser;
use wf_bot::{cli::Cli, handler, init_bot, periodic};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    // Get the channel id to be used as the news channel.
    let channel_id = args
        .channel_id
        .map_or_else(wf_bot::load_channel_id, Ok)
        .unwrap();

    // Create a new handler and client.
    let handler = Arc::new(handler::Handler::new(channel_id.into()));
    let mut client = init_bot(&args, handler.clone()).await.unwrap();

    // Provide the handler with a connection to the Discord client.
    handler.init_connection(client.http.clone()).await;

    // Start periodic task loops.
    periodic::start_tasks(handler.clone()).await;

    // Respond to user messages.
    client.start().await?;

    Ok(())
}

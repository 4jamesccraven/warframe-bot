use std::sync::Arc;

use anyhow::Ok;
use clap::Parser;
use wf_bot::{cli::Cli, handler, init_bot};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    let channel_id = args
        .channel_id
        .map_or_else(wf_bot::load_channel_id, Ok)
        .unwrap();

    let handler = Arc::new(handler::Handler::new(channel_id.into()));
    let mut client = init_bot(&args, handler.clone()).await.unwrap();

    handler.init_connection(client.http.clone()).await;

    client.start().await?;

    Ok(())
}

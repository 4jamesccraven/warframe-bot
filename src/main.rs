use anyhow::Ok;
use wf_bot::{cli::Cli, handler, init_bot};

use clap::Parser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    let mut client = init_bot(&args).await.unwrap();

    let channel_id = args
        .channel_id
        .clone()
        .map_or_else(wf_bot::load_channel_id, Ok)
        .unwrap();

    let handler = handler::Handler::new(client.http.clone(), channel_id.into());

    client.start().await?;

    Ok(())
}

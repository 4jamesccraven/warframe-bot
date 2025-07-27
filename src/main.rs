use std::sync::Arc;

use clap::Parser;
use wf_bot::{cli::Cli, error, handler, init_bot, periodic};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    // Create a new handler and client.
    let handler = Arc::new(handler::Handler::new(args.channel_id.into()));
    let mut client = match init_bot(&args, handler.clone()).await {
        Ok(client) => client,
        Err(e) => {
            error!(context = "loading client", "{e}");
            std::process::exit(1);
        }
    };

    // Provide the handler with a connection to the Discord client.
    handler.init_connection(client.http.clone()).await;

    // Start periodic task loops.
    periodic::start_tasks(handler.clone()).await;

    // Respond to user messages.
    client.start().await?;

    Ok(())
}

use std::boxed::Box;
use std::sync::Arc;

use clap::Parser;
use poise::serenity_prelude as serenity;
use wf_bot::{cli::Cli, commands::*, handler, periodic};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv()?;
    let args = Cli::parse();

    // Create a new handler and client.
    let handler = Arc::new(handler::Handler::new(args.channel_id.into()));
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![baro(), news(), help()],
            ..Default::default()
        })
        .setup(move |ctx, _ready, framework| {
            let handler = handler.clone();
            Box::pin(async move {
                handler.init_connection(ctx.http.clone()).await;
                periodic::start_tasks(handler.clone()).await;

                poise::builtins::register_in_guild(
                    ctx,
                    &framework.options().commands,
                    poise::serenity_prelude::GuildId::from(470047704098013184),
                )
                .await?;
                Ok(handler.as_ref().clone())
            })
        })
        .build();

    let mut client =
        serenity::Client::builder(&args.api_token, serenity::GatewayIntents::non_privileged())
            .framework(framework)
            .await?;

    // Respond to user messages.
    client.start().await?;

    Ok(())
}

use std::boxed::Box;
use std::sync::Arc;

use clap::Parser;
use poise::serenity_prelude as serenity;
use wf_bot::{cli::Cli, commands::*, handler, periodic, warning};

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

                match args.guild_id {
                    Some(id) => {
                        poise::builtins::register_in_guild(
                            ctx,
                            &framework.options().commands,
                            id.into(),
                        )
                        .await?;
                    }
                    None => {
                        poise::builtins::register_globally(ctx, &framework.options().commands)
                            .await?;

                        warning!(
                            context = "initialisation",
                            "registering slash commands globally. this may take up to an hour."
                        );
                    }
                }

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

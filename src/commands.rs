use crate::handler::Handler;
use crate::warning;

use anyhow::{Error, Result};
use poise::command;

type Context<'a> = poise::Context<'a, Handler, Error>;

/// Show when baro will be here next, or his inventory if he's here
#[command(slash_command, guild_cooldown = 360)]
pub async fn baro(ctx: Context<'_>) -> Result<()> {
    let handler = ctx.data();
    let messages = handler.baro_messages().await;

    if messages.is_empty() {
        ctx.say("Internal error, try again soon.").await?;
    }
    for msg in messages.into_iter() {
        if let Err(e) = ctx.say(msg).await {
            warning!(context = "sending message", "{e}");
        }
    }

    Ok(())
}

/// Show unseen news
#[command(slash_command, guild_cooldown = 360)]
pub async fn news(ctx: Context<'_>) -> Result<()> {
    let handler = ctx.data();
    let messages = handler.news_messages().await;

    if messages.is_empty() {
        ctx.say("No news to show.").await?;
    }
    for msg in messages.into_iter() {
        if let Err(e) = ctx.say(msg).await {
            warning!(context = "sending message", "{e}");
        }
    }

    Ok(())
}

/// Show what's in The Circuit, and what Archon Shard is available from this week's Archon Hunt.
#[command(slash_command, guild_cooldown = 360)]
pub async fn weekly(ctx: Context<'_>) -> Result<()> {
    let handler = ctx.data();
    let message = handler
        .weekly_messages()
        .await
        .unwrap_or("Internal error, try again soon.".into());

    if let Err(e) = ctx.say(&message).await {
        warning!(context = "sending message", "{e}");
    }

    Ok(())
}

/// Print a help message
#[command(slash_command, guild_cooldown = 360)]
pub async fn help(ctx: Context<'_>) -> Result<()> {
    let help_message = "Available Commands:\n\
                        - `/baro`  : Show when baro will be here next, or his inventory if he's here\n\
                        - `/news`  : Show unseen news\n\
                        - `/help`  : Print this message\n\
                        - `/weekly`: Show what's in The Circuit and what Archon Hunt is available";
    ctx.say(help_message).await?;
    Ok(())
}

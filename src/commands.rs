use crate::handler::Handler;

use anyhow::{Error, Result};
use poise::command;

type Context<'a> = poise::Context<'a, Handler, Error>;

/// Show when baro will be here next, or his inventory if he's here
#[command(slash_command, guild_cooldown = 360)]
pub async fn baro(ctx: Context<'_>) -> Result<()> {
    let handler = ctx.data();
    handler.notify_baro().await;
    ctx.say("Update sent to news channel.").await?;
    Ok(())
}

/// Show unseen news
#[command(slash_command)]
pub async fn news(ctx: Context<'_>) -> Result<()> {
    let handler = ctx.data();
    let something_sent = handler.notify_news().await;
    let message = if something_sent {
        "Update sent to news channel."
    } else {
        "No news to show."
    };
    ctx.say(message).await?;
    Ok(())
}

/// Print a help message
#[command(slash_command)]
pub async fn help(ctx: Context<'_>) -> Result<()> {
    let help_message = "Available Commands:\n\
                        - !baro: Show when baro will be here next, or his inventory if he's here\n\
                        - !news: Show unseen news\n\
                        - !help: Print this message";
    ctx.say(help_message).await?;
    Ok(())
}

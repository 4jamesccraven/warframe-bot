mod cache;

use std::env;

use anyhow::{Context, Result};

/// Load the Discord API token from an environment variable.
fn load_api_token() -> Result<String> {
    const ERR_MESG: &str = "A valid discord API token is required to run wf-bot.";
    let api_token = env::var("WF_DISCORDTOKEN").context(ERR_MESG)?;

    Ok(api_token)
}

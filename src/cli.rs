use clap::Parser;

#[derive(Debug, Clone, Parser)]
pub struct Cli {
    /// A valid token from the Discord Developer Portal for a discord bot.
    #[arg(long)]
    pub api_token: Option<String>,
    /// A valid ID for a discord channel for which the given bot has permissions.
    #[arg(long)]
    pub channel_id: Option<u64>,
}

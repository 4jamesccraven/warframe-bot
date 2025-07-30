use clap::Parser;

/// A Discord bot that interacts with the WarframeStatus API to send news to a give Discord channel.
/// For help see https://github.com/4jamesccraven/warframe-bot
#[derive(Debug, Clone, Parser)]
pub struct Cli {
    /// A valid token from the Discord Developer Portal for a discord bot.
    #[arg(long, env = "WF_DISCORDTOKEN")]
    pub api_token: String,

    /// A valid ID for a Discord channel for which the given bot has permissions.
    #[arg(long, env = "WF_CHANNELID")]
    pub channel_id: u64,

    /// A valid ID for a Discord server that should be used to register the slash commands. Global
    /// assumed if no id is provided.
    #[arg(long, env = "WF_GUILDID")]
    pub guild_id: Option<u64>,
}

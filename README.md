# wf-bot
This is a Discord bot that uses the [WarframeStatus
API](https://docs.warframestat.us/) to provide news updates as well as
information on Baro Ki'Teer

## Features
- Send periodic updates to a provided channel
- Handle on-demand information requests using commands (e.g., `!news` or `!baro`)

## Running
To run wf-bot you need a valid Discord Application token, and the ID of a channel in a server
that you are capable of adding the bot to:

1. Install the bot: `cargo install --git https://github.com/4jamesccraven/warframe-bot`
2. Make an application on the [Discord Developer Portal](https://discord.com/developers/applications)
3. Add the bot to your server with the following permissions (`OAuth2 > Scopes > bot`)
    - Send Messages
    - Manage Messages
    - Embed Links
    - Attach Files
4. Make a news channel, and copy its ID (`right click > Copy Channel ID`)
5. Run wf-bot like this:
    ```bash
    wf-bot \
        --api-token YOUR_TOKEN \
        --channel-id YOUR_ID
    ```
    or like this
    ```bash
    WF_DISCORDTOKEN=YOUR_TOKEN WF_CHANNELID=YOUR_ID wf-bot
    ```

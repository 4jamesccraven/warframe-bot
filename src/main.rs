mod api;
mod date;
mod discord;

use api::{NewsItem, VoidTrader};
use discord::ToDiscordMessage;

use std::boxed::Box;
use std::error::Error;

use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let trader = VoidTrader::get().await?;

    println!("{}", trader.message());

    Ok(())
}

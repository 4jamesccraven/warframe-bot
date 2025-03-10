mod api;

use api::{NewsItem, VoidTrader};

use std::boxed::Box;
use std::error::Error;

use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let news = NewsItem::get_all().await?;
    let trader = VoidTrader::get().await?;

    for item in news {
        println!("{}", item.asString);
    }

    println!("{}", "-".repeat(20));

    println!("{:?}", trader);

    Ok(())
}

mod blacklist;
mod cache;
mod circuit;
pub mod cli;
pub mod commands;
pub mod handler;
mod item_display;
pub mod logging;
mod news_wrapper;
pub mod periodic;

pub use blacklist::BLACKLIST;
pub use news_wrapper::*;

use anyhow::Result;

/// Format the date style given by the warframe API.
pub fn fmt_api_date(date: &chrono::DateTime<chrono::Utc>) -> Result<String> {
    let local = date.with_timezone(&chrono::Local);
    Ok(format!("{}", local.format("%a, %b %d")))
}

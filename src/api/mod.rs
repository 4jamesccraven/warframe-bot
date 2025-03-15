mod news;
mod trader;

pub use news::*;
pub use trader::*;

use crate::discord::ToDiscordMessage;

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use anyhow::Error;
use reqwest as rq;
use serde::{Deserialize, Serialize};
use serde_json;
use tokio::sync::Mutex;

#[derive(Clone, Default)]
pub struct Cache {
    seen_news: Arc<Mutex<HashSet<String>>>,
}

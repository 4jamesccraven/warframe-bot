use std::{hash::Hash, ops::Deref};

use anyhow::Result;
use serde::{Deserialize, Serialize, Serializer};
use warframe::worldstate::queryable;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct News(#[serde(serialize_with = "serialize_news")] pub queryable::News);

impl News {
    pub fn as_message(&self) -> Result<String> {
        Ok(format!(
            "[{}] [{}]({})",
            crate::fmt_api_date(&self.date)?,
            self.message,
            self.image_link,
        ))
    }
}

impl PartialEq for News {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for News {}

impl Hash for News {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Deref for News {
    type Target = queryable::News;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<queryable::News> for News {
    fn from(value: queryable::News) -> Self {
        Self(value)
    }
}

fn serialize_news<S>(news: &queryable::News, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    use serde::ser::SerializeStruct;

    let mut state = serializer.serialize_struct("News", 10)?;

    state.serialize_field("id", &news.id)?;
    state.serialize_field("message", &news.message)?;
    state.serialize_field("image_link", &news.image_link)?;
    state.serialize_field("priority", &news.priority)?;
    state.serialize_field("update", &news.update)?;
    state.serialize_field("stream", &news.stream)?;
    state.serialize_field("date", &news.date)?;
    state.serialize_field("start_date", &news.start_date)?;
    state.serialize_field("end_date", &news.end_date)?;

    state.end()
}

#[cfg(test)]
mod news_wrapper_test {
    use super::*;

    #[tokio::test]
    async fn serialize_is_deserialize() {
        use bincode::serde::{decode_from_slice, encode_to_vec};
        use warframe::worldstate::{Client, queryable};

        let cfg = bincode::config::standard();

        let most_recent_news = Client::default()
            .fetch::<queryable::News>()
            .await
            .unwrap()
            .last()
            .cloned()
            .unwrap();
        let most_recent_news = News(most_recent_news);
        let serialized = encode_to_vec(&most_recent_news, cfg).unwrap();
        let (deserialized, _): (News, _) = decode_from_slice(&serialized, cfg).unwrap();

        assert_eq!(most_recent_news, deserialized)
    }
}

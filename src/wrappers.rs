use std::{hash::Hash, ops::Deref};

use anyhow::Result;
use warframe::worldstate::queryable;

#[derive(Debug, Clone)]
pub struct News(queryable::News);

impl News {
    pub fn as_message(&self) -> Result<String> {
        let end = self.as_string.split('[').last().unwrap();
        Ok(format!("[{}] [{}", crate::fmt_api_date(&self.date)?, end))
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

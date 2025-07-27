use crate::News;

use std::collections::HashSet;

use once_cell::sync::Lazy;
use warframe::worldstate::queryable;

pub static BLACKLIST: Lazy<HashSet<News>> = Lazy::new(|| {
    let mut set = HashSet::new();
    set.insert(News(queryable::News {
        id: "62d31b87106360aa5703954d".into(),
        message: "Join the official Warframe Discord server".into(),
        image_link: "https://cdn.warframestat.us/genesis/img/news-placeholder.png".into(),
        priority: false,
        update: false,
        stream: false,
        as_string: "[20294d 23h 35m 50s ago] [Join the official Warframe Discord server](https://discord.com/invite/playwarframe)".into(),
        date: "1970-01-01T00:00:00Z".parse().unwrap(),
        start_date: None,
        end_date: None,
    }));
    set.insert(News(queryable::News {
        id: "67ae4e9fca4611344608d246".into(),
        message: "Check out the official Warframe Wiki ".into(),
        image_link: "https://cdn.warframestat.us/genesis/img/news-placeholder.png".into(),
        priority: false,
        update: false,
        stream: false,
        as_string: "[20294d 23h 35m 50s ago] [Check out the official Warframe Wiki ](https://wiki.warframe.com/)".into(),
        date: "1970-01-01T00:00:00Z".parse().unwrap(),
        start_date: None,
        end_date: None,
    }));
    set.insert(News(queryable::News {
        id: "6824c85b6c30b5a005004018".into(),
        message: "Visit the official Warframe Forums!".into(),
        image_link: "https://cdn.warframestat.us/genesis/img/news-placeholder.png".into(),
        priority: false,
        update: false,
        stream: false,
        as_string: "[20294d 23h 35m 50s ago] [Visit the official Warframe Forums!](https://forums.warframe.com/)".into(),
        date: "1970-01-01T00:00:00Z".parse().unwrap(),
        start_date: None,
        end_date: None,
    }));
    set
});

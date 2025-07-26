use crate::handler::Handler;

use std::sync::Arc;

use chrono::{DateTime, Datelike, Local, Timelike};
use tokio::time::{sleep, Duration};

/// Checks every minute whether a task should be run if a time-based condition is met.
pub async fn task<F, C, Fut>(mut condition: C, mut task: F)
where
    F: FnMut() -> Fut + Send + 'static,
    C: FnMut(DateTime<Local>) -> bool + Send + 'static,
    Fut: std::future::Future<Output = ()> + Send,
{
    tokio::spawn(async move {
        loop {
            let now = Local::now();

            if condition(now) {
                task().await;
            }

            let next_minute = now.with_second(0).unwrap().with_nanosecond(0).unwrap()
                + chrono::Duration::minutes(1);

            let sleep_duration = (next_minute - Local::now())
                .to_std()
                .unwrap_or(Duration::from_secs(60));

            sleep(sleep_duration).await;
        }
    });
}

pub async fn start_tasks(handler: Arc<Handler>) {
    // Check for news updates every minute
    let handler_clone = handler.clone();
    task(
        |_| true,
        move || {
            let handler = handler_clone.clone();
            async move {
                println!("Checking for news...");
                handler.notify_news().await;
            }
        },
    )
    .await;

    // Check for Baro Ki'Teer updates every Friday at 2pm
    let handler_clone = handler.clone();
    task(
        |now| {
            let now_utc = now.with_timezone(&chrono::Utc);
            now_utc.weekday() == chrono::Weekday::Fri && now_utc.hour() == 14
        },
        move || {
            let handler = handler_clone.clone();
            async move {
                println!("Checking for Baro...");
                handler.notify_baro().await;
            }
        },
    )
    .await;
}

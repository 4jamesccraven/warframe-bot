use crate::handler::Handler;

use std::sync::Arc;

use chrono::{DateTime, Datelike, Local, Timelike};
use tokio::time::{Duration, sleep};

/// Spawn the periodic tasks that the bot does.
pub async fn start_tasks(handler: Arc<Handler>) {
    // Check for news updates every minute
    let handler_clone = handler.clone();
    task(
        |_| true,
        move || {
            let handler = handler_clone.clone();
            async move {
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
            now_utc.weekday() == chrono::Weekday::Fri
                && now_utc.hour() == 14
                && now_utc.minute() == 0
        },
        move || {
            let handler = handler_clone.clone();
            async move {
                // Only send an update if he is in fact active.
                // This is necessary because, unlike `Handler::notify_news`, this method *always*
                // produces output, which is undesirable for the generic auto-check.
                if handler.check_baro().await {
                    handler.notify_baro().await;
                }
            }
        },
    )
    .await;

    // Send an update about Weekly offerings every Monday at 0:00 UTC.
    let handler_clone = handler.clone();
    task(
        |now| {
            let now_utc = now.with_timezone(&chrono::Utc);
            now_utc.weekday() == chrono::Weekday::Mon
                && now_utc.hour() == 0
                && now_utc.minute() == 0
        },
        move || {
            let handler = handler_clone.clone();
            async move {
                handler.notify_weekly().await;
            }
        },
    )
    .await;
}

/// Checks every minute whether a task should be run if a time-based condition is met.
async fn task<F, C, Fut>(mut condition: C, mut task: F)
where
    F: FnMut() -> Fut + Send + 'static,
    C: FnMut(DateTime<Local>) -> bool + Send + 'static,
    Fut: std::future::Future<Output = ()> + Send,
{
    tokio::spawn(async move {
        loop {
            // Get the time.
            let now = Local::now();

            // Determine if it is time to run the task.
            if condition(now) {
                task().await;
            }

            // Find the time until the start of the next minute, or simply wait a whole minut if it
            // cannot be determined.
            //
            // It is safe to unwrap these calls because 0 is a valid value for both.
            let next_minute = now.with_second(0).unwrap().with_nanosecond(0).unwrap()
                + chrono::Duration::minutes(1);

            let sleep_duration = (next_minute - Local::now())
                .to_std()
                .unwrap_or(Duration::from_secs(60));

            // Wait until the next minute to check.
            sleep(sleep_duration).await;
        }
    });
}

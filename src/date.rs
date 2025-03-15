use chrono::{DateTime, Duration, Local, NaiveDateTime, Utc};

pub fn api_to_chrono(date: &str) -> Option<DateTime<Local>> {
    let naive = NaiveDateTime::parse_from_str(date, "%Y-%m-%dT%H:%M:%S%.3fZ").ok()?;

    let utc: DateTime<Utc> = naive.and_utc();

    let local = utc.with_timezone(&Local);

    Some(local)
}

pub fn within_5_days(date: &str) -> bool {
    let naive = NaiveDateTime::parse_from_str(date, "%Y-%m-%dT%H:%M:%S%.3fZ");
    if let Err(why) = naive {
        eprintln!("[ERROR]: could not map api time (3): {why:?}");
        return false;
    }
    let naive = naive.unwrap();
    let utc: DateTime<Utc> = naive.and_utc();
    let now = Utc::now();

    now - utc <= Duration::days(5)
}

#[cfg(test)]
mod test {
    use super::*;
    use chrono::{Local, TimeZone, Utc};

    #[test]
    fn conversion() {
        let api_response = "2025-03-21T13:00:00.000Z";

        let datetime = Utc
            .with_ymd_and_hms(2025, 3, 21, 13, 0, 0)
            .unwrap()
            .with_timezone(&Local);

        assert_eq!(api_to_chrono(api_response).unwrap(), datetime);
    }
}

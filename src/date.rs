use chrono::{DateTime, Local, NaiveDateTime, Utc};

pub fn api_to_chrono(date: &str) -> Option<DateTime<Local>> {
    let naive = NaiveDateTime::parse_from_str(date, "%Y-%m-%dT%H:%M:%S%.3fZ").ok()?;

    let utc: DateTime<Utc> = naive.and_utc();

    let local = utc.with_timezone(&Local);

    Some(local)
}

#[cfg(test)]
mod test {
    use super::*;
    use chrono::{Local, TimeZone, Utc};

    #[test]
    fn conversion() {
        let api_response = "2025-03-21T13:00:00.000Z";

        let datetime = Utc.ymd(2025, 3, 21).and_hms(13, 0, 0).with_timezone(&Local);

        assert_eq!(api_to_chrono(api_response).unwrap(), datetime);
    }
}

use chrono::{Duration, NaiveDateTime, TimeZone, Utc};
use sea_orm::prelude::DateTimeUtc;

pub fn now() -> DateTimeUtc {
    Utc::now() + Duration::hours(8)
}

pub fn get_date_time_by_millis(millis: i64) -> DateTimeUtc {
    DateTimeUtc::from_timestamp_millis(millis + 8 * 3600 * 1000).unwrap()
}

pub fn gtm_time(time: DateTimeUtc) -> DateTimeUtc {
    time + Duration::hours(8)
}

pub fn string_to_date_time(time: &str) -> DateTimeUtc {
    let naive = NaiveDateTime::parse_from_str(time, "%Y-%m-%dT%H:%M:%S%.fZ")
        .expect("Failed to parse datetime string");
    Utc.from_utc_datetime(&naive)
}

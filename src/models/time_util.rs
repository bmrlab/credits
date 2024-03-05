use chrono::{Duration, Utc};
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

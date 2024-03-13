use chrono::{DateTime, Utc};
use serde::Deserialize;

pub(crate) fn datetime_from_unix_timestamp<'de, D>(
    deserializer: D,
) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let timestamp =
        chrono::NaiveDateTime::from_timestamp_opt(i64::deserialize(deserializer)?, 0).unwrap();
    Ok(Some(DateTime::<Utc>::from_naive_utc_and_offset(
        timestamp, Utc,
    )))
}
